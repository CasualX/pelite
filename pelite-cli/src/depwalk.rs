use super::*;
use pelite::{Export, Import, PeFile};

#[derive(serde::Serialize)]
struct Report {
	modules: Vec<Module>,
	issues: Vec<Issue>,
}

#[derive(serde::Serialize)]
struct Module {
	name: String,
	path: Option<PathBuf>,
	imports: Vec<String>,
}

#[derive(serde::Serialize)]
struct Issue {
	kind: &'static str,
	module: String,
	detail: String,
}

#[derive(Clone, PartialEq, Eq)]
enum Symbol {
	Name(String),
	Ordinal(u16),
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Name(name) => f.write_str(name),
			Self::Ordinal(ord) => write!(f, "#{ord}"),
		}
	}
}

struct Request {
	from: usize,
	to: usize,
	symbol: Symbol,
	checked: bool,
}

struct Walker {
	search_dirs: Vec<PathBuf>,
	machine: Option<u16>,
	report: Report,
	indices: HashMap<String, usize>,
	requests: Vec<Request>,
	parsed: usize,
}

pub fn command() -> clap::Command {
	clap::Command::new("depwalk")
		.about("Walk PE imports and check that modules and exported symbols exist")
		.after_help(include_str!("docs/depwalk.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("search-directory")
			.long("search-directory")
			.short('L')
			.value_name("DIR")
			.value_parser(clap::value_parser!(PathBuf))
			.action(clap::ArgAction::Append)
			.help("Search this directory for imported modules (repeatable)"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let file = matches.get_one::<PathBuf>("file").expect("required by clap");
	let machine = read_machine(file);
	let extra = matches.get_many::<PathBuf>("search-directory");
	let mut search_dirs = Vec::new();
	if let Some(parent) = file.parent() {
		search_dirs.push(if parent.as_os_str().is_empty() { PathBuf::from(".") } else { parent.to_path_buf() });
	}
	if let Some(extra) = extra { search_dirs.extend(extra.cloned()); }
	#[cfg(windows)]
	{
		if let Some(root) = std::env::var_os("SystemRoot") {
			let root = PathBuf::from(root);
			let system = if machine == Some(pelite::image::IMAGE_FILE_MACHINE_I386) && root.join("SysWOW64").is_dir() {
				root.join("SysWOW64")
			}
			else if machine == Some(pelite::image::IMAGE_FILE_MACHINE_AMD64) && cfg!(target_pointer_width = "32") {
				root.join("Sysnative")
			}
			else { root.join("System32") };
			search_dirs.push(system);
			search_dirs.push(root);
		}
	}
	search_dirs.push(std::env::current_dir()?);
	#[cfg(windows)]
	{
		if let Some(path) = std::env::var_os("PATH") {
			search_dirs.extend(std::env::split_paths(&path));
		}
	}
	let mut walker = Walker {
		search_dirs,
		machine,
		report: Report { modules: vec![Module { name: file.file_name().unwrap_or_default().to_string_lossy().into_owned(), path: Some(file.clone()), imports: Vec::new() }], issues: Vec::new() },
		indices: HashMap::new(),
		requests: Vec::new(),
		parsed: 0,
	};
	walker.indices.insert(walker.report.modules[0].name.to_ascii_lowercase(), 0);
	walker.walk();
	match format {
		OutputFormat::Json => print_json(&walker.report, false),
		OutputFormat::JsonPretty => print_json(&walker.report, true),
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			for (index, module) in walker.report.modules.iter().enumerate() {
				match &module.path {
					Some(path) => writeln!(output, "[{index}] {} -> {}", module.name, path.display())?,
					None => writeln!(output, "[{index}] {} -> <not found>", module.name)?,
				}
				for name in &module.imports { writeln!(output, "    {name}")?; }
			}
			for issue in &walker.report.issues {
				writeln!(output, "{}: {}: {}", issue.kind, issue.module, issue.detail)?;
			}
			writeln!(output, "{} modules, {} issues", walker.report.modules.len(), walker.report.issues.len())?;
			Ok(())
		},
	}?;
	if walker.report.issues.is_empty() { Ok(()) }
	else { Err(err(format!("dependency walk found {} issues", walker.report.issues.len()))) }
}

fn read_machine(path: &Path) -> Option<u16> {
	let map = pelite::FileMap::open(path).ok()?;
	Some(PeFile::from_bytes(&map).ok()?.file_header().Machine)
}

fn is_api_set(name: &str) -> bool {
	name.get(..4).is_some_and(|prefix| prefix.eq_ignore_ascii_case("api-") || prefix.eq_ignore_ascii_case("ext-"))
}

impl Walker {
	fn issue(&mut self, kind: &'static str, module: usize, detail: impl Into<String>) {
		self.report.issues.push(Issue { kind, module: self.report.modules[module].name.clone(), detail: detail.into() });
	}

	fn discover(&mut self, name: &str) -> usize {
		let key = name.to_ascii_lowercase();
		if let Some(&index) = self.indices.get(&key) { return index; }
		let (path, wrong_machine) = self.find_module(name);
		let index = self.report.modules.len();
		self.report.modules.push(Module { name: name.to_owned(), path, imports: Vec::new() });
		self.indices.insert(key, index);
		if let Some((path, actual)) = wrong_machine {
			self.issue("wrong_architecture", index, format!("{} has machine 0x{actual:04x}; expected 0x{:04x}", path.display(), self.machine.unwrap()));
		}
		else if self.report.modules[index].path.is_none() {
			self.issue("module_not_found", index, "no matching file in search directories");
		}
		index
	}

	fn find_module(&self, name: &str) -> (Option<PathBuf>, Option<(PathBuf, u16)>) {
		// Import names are file names, never paths supplied by the binary.
		if name.is_empty() || name.contains(['/', '\\']) { return (None, None); }
		let mut wrong_machine = None;
		for dir in &self.search_dirs {
			let exact = dir.join(name);
			let candidate = if exact.is_file() { Some(exact) }
			else {
			// Windows file names are case insensitive; Linux file names are not.
				fs::read_dir(dir).ok().and_then(|entries| entries.flatten().find(|entry| {
					entry.file_name().to_string_lossy().eq_ignore_ascii_case(name) && entry.path().is_file()
				}).map(|entry| entry.path()))
			};
			if let Some(candidate) = candidate {
				if let (Some(expected), Some(actual)) = (self.machine, read_machine(&candidate)) {
					if expected != actual {
						if wrong_machine.is_none() { wrong_machine = Some((candidate, actual)); }
						continue;
					}
				}
				return (Some(candidate), None);
			}
		}
		(None, wrong_machine)
	}

	fn walk(&mut self) {
		loop {
			while self.parsed < self.report.modules.len() {
				let index = self.parsed;
				self.parsed += 1;
				self.parse(index);
			}
			let next = self.requests.iter().position(|request| !request.checked);
			match next {
				Some(index) => {
					self.requests[index].checked = true;
					self.check(index);
				},
				None => break,
			}
		}
		self.cycles();
	}

	fn request(&mut self, from: usize, to: usize, symbol: Symbol) {
		if !self.requests.iter().any(|request| request.from == from && request.to == to && request.symbol == symbol) {
			self.requests.push(Request { from, to, symbol, checked: false });
		}
	}

	fn parse(&mut self, index: usize) {
		let Some(path) = self.report.modules[index].path.clone() else { return; };
		let map = match pelite::FileMap::open(&path) {
			Ok(map) => map,
			Err(error) => { self.issue("pe_read_error", index, format!("{}: {error}", path.display())); return; },
		};
		let pe = match PeFile::from_bytes(&map) {
			Ok(pe) => pe,
			Err(error) => { self.issue("pe_read_error", index, format!("{}: {error}", path.display())); return; },
		};
		let imports = match pe.imports() {
			Ok(imports) => imports,
			Err(error) if error.is_null() => return,
			Err(error) => { self.issue("pe_read_error", index, format!("import directory: {error}")); return; },
		};
		for descriptor in imports {
			let name = match descriptor.dll_name().and_then(|name| name.to_str().map_err(Into::into)) {
				Ok(name) => name,
				Err(error) => { self.issue("pe_read_error", index, format!("import module name: {error}")); continue; },
			};
			if is_api_set(name) { continue; }
			let target = self.discover(name);
			if !self.report.modules[index].imports.iter().any(|item| item.eq_ignore_ascii_case(name)) {
				self.report.modules[index].imports.push(name.to_owned());
			}
			let table = match descriptor.int() {
				Ok(table) => table,
				Err(error) => { self.issue("pe_read_error", index, format!("{name} import table: {error}")); continue; },
			};
			for import in table {
				let symbol = match import {
					Ok(Import::ByName { name, .. }) => match name.to_str() {
						Ok(name) => Symbol::Name(name.to_owned()),
						Err(error) => { self.issue("pe_read_error", index, format!("{name} import name: {error}")); continue; },
					},
					Ok(Import::ByOrdinal { ord }) => Symbol::Ordinal(ord),
					Err(error) => { self.issue("pe_read_error", index, format!("{name} import: {error}")); continue; },
				};
				self.request(index, target, symbol);
			}
		}
	}

	fn check(&mut self, index: usize) {
		let request = &self.requests[index];
		let (from, to, symbol) = (request.from, request.to, request.symbol.clone());
		let Some(path) = self.report.modules[to].path.clone() else { return; };
		let map = match pelite::FileMap::open(&path) {
			Ok(map) => map,
			Err(_) => return, // The module's parse pass reports this.
		};
		let pe = match PeFile::from_bytes(&map) {
			Ok(pe) => pe,
			Err(_) => return,
		};
		let export = match pe.exports().and_then(|exports| exports.by()) {
			Ok(by) => match &symbol {
				Symbol::Name(name) => by.name(name),
				Symbol::Ordinal(ord) => by.ordinal(*ord),
			},
			Err(error) if error.is_null() => { self.issue("import_not_found", from, format!("{}!{symbol}", self.report.modules[to].name)); return; },
			Err(error) => { self.issue("pe_read_error", to, format!("export directory: {error}")); return; },
		};
		match export {
			Ok(Export::Symbol(_)) => {},
			Ok(Export::Forward(forward)) => {
				let value = match forward.to_str() {
					Ok(value) => value,
					Err(error) => { self.issue("pe_read_error", to, format!("invalid forwarder: {error}")); return; },
				};
				if let Some((dll, name)) = value.rsplit_once('.') {
					let dll = if dll.contains('.') { dll.to_owned() } else { format!("{dll}.dll") };
					if is_api_set(&dll) { return; }
					let target = self.discover(&dll);
					if !self.report.modules[to].imports.iter().any(|item| item.eq_ignore_ascii_case(&dll)) {
						self.report.modules[to].imports.push(dll);
					}
					let forwarded = match name.strip_prefix('#') {
						Some(ord) => ord.parse().map(Symbol::Ordinal),
						None => Ok(Symbol::Name(name.to_owned())),
					};
					match forwarded {
						Ok(symbol) => self.request(to, target, symbol),
						Err(_) => self.issue("pe_read_error", to, format!("invalid forwarder: {value}")),
					}
				}
				else { self.issue("pe_read_error", to, format!("invalid forwarder: {value}")); }
			},
			Err(error) if error.is_null() || error == pelite::Error::Bounds => self.issue("import_not_found", from, format!("{}!{symbol}", self.report.modules[to].name)),
			Err(error) => self.issue("pe_read_error", to, format!("{}!{symbol}: {error}", self.report.modules[to].name)),
		}
	}

	fn cycles(&mut self) {
		let mut state = vec![0u8; self.report.modules.len()];
		for index in 0..state.len() {
			if state[index] == 0 { self.visit(index, &mut state); }
		}
	}

	fn visit(&mut self, index: usize, state: &mut [u8]) {
		state[index] = 1;
		let neighbors: Vec<_> = self.report.modules[index].imports.iter()
			.filter_map(|name| self.indices.get(&name.to_ascii_lowercase()).copied()).collect();
		for next in neighbors {
			match state[next] {
				0 => self.visit(next, state),
				1 => self.issue("cycle", index, format!("{} -> {}", self.report.modules[index].name, self.report.modules[next].name)),
				_ => {},
			}
		}
		state[index] = 2;
	}
}
