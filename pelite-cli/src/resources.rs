use pelite::resources::group::ResourceGroup;
use pelite::resources::{ResourceDirectory, ResourceDirectoryTable, ResourceEntry, ResourceName};

use super::*;

#[derive(serde::Serialize)]
struct Inventory {
	root: String,
	directories: usize,
	files: usize,
	bytes: usize,
	entries: Vec<InventoryEntry>,
}

#[derive(Copy, Clone, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum InventoryKind {
	File,
	Directory,
}

#[derive(serde::Serialize)]
struct InventoryEntry {
	path: String,
	name: String,
	kind: InventoryKind,
	depth: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	size: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	code_page: Option<u32>,
}

#[derive(serde::Serialize)]
struct FsckReport {
	valid: bool,
	directories: usize,
	files: usize,
	bytes: usize,
}

#[derive(serde::Serialize)]
struct CatOutput<'a> {
	path: &'a str,
	size: usize,
	code_page: u32,
	encoding: &'static str,
	data: String,
}

#[derive(Copy, Clone)]
enum GroupKind {
	Icon,
	Cursor,
}

impl GroupKind {
	fn title(self) -> &'static str {
		match self {
			Self::Icon => "Icon groups",
			Self::Cursor => "Cursor groups",
		}
	}

	fn plural(self) -> &'static str {
		match self {
			Self::Icon => "icons",
			Self::Cursor => "cursors",
		}
	}

	fn singular(self) -> &'static str {
		match self {
			Self::Icon => "icon",
			Self::Cursor => "cursor",
		}
	}

	fn extension(self) -> &'static str {
		match self {
			Self::Icon => "ico",
			Self::Cursor => "cur",
		}
	}
}

#[derive(serde::Serialize)]
struct GroupInfo {
	kind: &'static str,
	name: String,
	images: usize,
	bytes: usize,
}

#[derive(serde::Serialize)]
struct ExtractedGroup {
	kind: &'static str,
	name: String,
	path: String,
	images: usize,
	bytes: usize,
}

struct PreparedGroup {
	info: GroupInfo,
	file_name: String,
	contents: Vec<u8>,
}

pub fn command() -> clap::Command {
	clap::Command::new("resources")
		.about("Browse and validate the PE resource filesystem")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommand(clap::Command::new("tree")
			.about("Show the resource tree without dumping payloads")
			.arg(file_arg())
			.arg(clap::Arg::new("path")
				.value_name("PATH")
				.value_parser(clap::value_parser!(PathBuf))
				.default_value("/")
				.help("Absolute resource directory path")))
		.subcommand(clap::Command::new("cat")
			.about("Write one resource payload; JSON formats use base64")
			.arg(file_arg())
			.arg(clap::Arg::new("path")
				.value_name("PATH")
				.value_parser(clap::value_parser!(PathBuf))
				.required(true)
				.help("Absolute path to a resource data entry")))
		.subcommand(clap::Command::new("fsck")
			.about("Validate every directory, name, entry, and payload")
			.arg(file_arg()))
		.subcommand(group_command(GroupKind::Icon))
		.subcommand(group_command(GroupKind::Cursor))
		.after_help("Resource paths look like /#MANIFEST/#1/#1033. In text mode, 'cat' writes raw bytes with no decoration, making it safe to pipe or redirect.")
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let (operation, matches) = matches.subcommand().expect("required by clap");
	match operation {
		"tree" => with_resources(matches, |resources| tree(resources, matches.get_one::<PathBuf>("path").expect("defaulted by clap"), format)),
		"cat" => with_resources(matches, |resources| cat(resources, matches.get_one::<PathBuf>("path").expect("required by clap"), format)),
		"fsck" => with_resources(matches, |resources| fsck(resources, format)),
		"icons" => groups(matches, GroupKind::Icon, format),
		"cursors" => groups(matches, GroupKind::Cursor, format),
		_ => unreachable!("clap validates subcommands"),
	}
}

fn file_arg() -> clap::Arg {
	clap::Arg::new("file")
		.value_name("FILE")
		.value_parser(clap::value_parser!(PathBuf))
		.required(true)
}

fn group_command(kind: GroupKind) -> clap::Command {
	clap::Command::new(kind.plural())
		.about(match kind {
			GroupKind::Icon => "Inspect and extract icon groups as .ico files",
			GroupKind::Cursor => "Inspect and extract cursor groups as .cur files",
		})
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommand(clap::Command::new("list")
			.about("List and validate the available groups")
			.arg(file_arg()))
		.subcommand(clap::Command::new("extract")
			.about("Extract selected groups, or every group when no names are given")
			.arg(file_arg())
			.arg(clap::Arg::new("destination")
				.value_name("DESTINATION")
				.value_parser(clap::value_parser!(PathBuf))
				.required(true)
				.help("Directory for reconstructed files"))
			.arg(clap::Arg::new("names")
				.value_name("NAME")
				.num_args(0..)
				.help("Group names or IDs such as MAIN or #103; omit to extract all"))
			.arg(clap::Arg::new("force")
				.long("force")
				.short('f')
				.action(clap::ArgAction::SetTrue)
				.help("Overwrite existing files")))
}

fn with_resources<T>(matches: &clap::ArgMatches, run: impl FnOnce(ResourceDirectory<'_>) -> Result<T>) -> Result<T> {
	let file = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(file)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	run(pe.resources()?)
}

fn groups(matches: &clap::ArgMatches, kind: GroupKind, format: OutputFormat) -> Result {
	let (operation, matches) = matches.subcommand().expect("required by clap");
	with_resources(matches, |resources| match operation {
		"list" => list_groups(resources, kind, format),
		"extract" => extract_groups(resources, matches, kind, format),
		_ => unreachable!("clap validates subcommands"),
	})
}

fn list_groups(resources: ResourceDirectory<'_>, kind: GroupKind, format: OutputFormat) -> Result {
	let prepared = prepare_groups(resources, kind, &[])?;
	let groups = prepared.iter().map(|group| &group.info).collect::<Vec<_>>();
	print(kind.title(), &groups, format)
}

fn extract_groups(resources: ResourceDirectory<'_>, matches: &clap::ArgMatches, kind: GroupKind, format: OutputFormat) -> Result {
	let destination = matches.get_one::<PathBuf>("destination").expect("required by clap");
	let names = matches.get_many::<String>("names").into_iter().flatten().map(String::as_str).collect::<Vec<_>>();
	let prepared = prepare_groups(resources, kind, &names)?;
	let force = matches.get_flag("force");
	let mut targets = HashSet::new();
	for group in &prepared {
		let path = destination.join(&group.file_name);
		if !targets.insert(path.clone()) {
			return Err(err(format!("multiple resource names map to {}", path.display())));
		}
		if !force && path.exists() {
			return Err(err(format!("{} already exists; use --force to overwrite it", path.display())));
		}
	}

	fs::create_dir_all(destination)?;
	let mut extracted = Vec::with_capacity(prepared.len());
	for group in prepared {
		let path = destination.join(&group.file_name);
		if force {
			fs::write(&path, &group.contents)?;
		}
		else {
			let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&path)?;
			file.write_all(&group.contents)?;
		}
		extracted.push(ExtractedGroup {
			kind: group.info.kind,
			name: group.info.name,
			path: path.to_string_lossy().into_owned(),
			images: group.info.images,
			bytes: group.info.bytes,
		});
	}

	match format {
		OutputFormat::Text => {
			for group in &extracted {
				println!("{}", group.path);
			}
			Ok(())
		},
		OutputFormat::Json => print_json(&extracted, false),
		OutputFormat::JsonPretty => print_json(&extracted, true),
	}
}

fn prepare_groups(resources: ResourceDirectory<'_>, kind: GroupKind, names: &[&str]) -> Result<Vec<PreparedGroup>> {
	let mut groups = Vec::new();
	match kind {
		GroupKind::Icon => {
			for group in resources.icons() {
				groups.push(group?);
			}
		},
		GroupKind::Cursor => {
			for group in resources.cursors() {
				groups.push(group?);
			}
		},
	}

	let requested = names.iter().copied().collect::<BTreeSet<_>>();
	let mut found = BTreeSet::new();
	let mut prepared = Vec::new();
	for (name, group) in groups {
		let matches = requested.is_empty() || requested.iter().any(|requested| name == ResourceName::Str(requested));
		if !matches {
			continue;
		}
		for requested in &requested {
			if name == ResourceName::Str(requested) {
				found.insert(*requested);
			}
		}
		prepared.push(prepare_group(name, group, kind)?);
	}
	if let Some(missing) = requested.difference(&found).next() {
		return Err(err(format!("{} group '{missing}' was not found", kind.singular())));
	}
	Ok(prepared)
}

fn prepare_group(name: ResourceName<'_>, group: ResourceGroup<'_>, kind: GroupKind) -> Result<PreparedGroup> {
	let contents = group.to_vec()?;
	let display_name = name.to_string();
	Ok(PreparedGroup {
		info: GroupInfo {
			kind: kind.singular(),
			name: display_name,
			images: group.entries().len(),
			bytes: contents.len(),
		},
		file_name: format!("{}.{}", safe_file_stem(name), kind.extension()),
		contents,
	})
}

fn safe_file_stem(name: ResourceName<'_>) -> String {
	let name = match name {
		ResourceName::Id(id) => id.to_string(),
		ResourceName::Wide(words) => String::from_utf16_lossy(words),
		ResourceName::Str(name) => name.to_owned(),
	};
	let name = name.chars().map(|character| {
		if character.is_alphanumeric() || matches!(character, '-' | '_' | '.') {
			character
		}
		else {
			'_'
		}
	}).collect::<String>();
	if name.is_empty() || name == "." || name == ".." {
		"resource".to_owned()
	}
	else {
		name
	}
}

fn tree(resources: ResourceDirectory<'_>, path: &Path, format: OutputFormat) -> Result {
	let inventory = inventory(resources, path)?;
	match format {
		OutputFormat::Text => {
			let stdout = io::stdout();
			let mut output = stdout.lock();
			writeln!(output, "{}", inventory.root)?;
			for entry in &inventory.entries {
				write!(output, "{}{}", "  ".repeat(entry.depth + 1), entry.name)?;
				if matches!(entry.kind, InventoryKind::Directory) {
					writeln!(output, "/")?;
				}
				else {
					writeln!(output, "  ({} bytes, code page {})", entry.size.unwrap_or(0), entry.code_page.unwrap_or(0))?;
				}
			}
			Ok(())
		},
		OutputFormat::Json => print_json(&inventory, false),
		OutputFormat::JsonPretty => print_json(&inventory, true),
	}
}

fn cat(resources: ResourceDirectory<'_>, path: &Path, format: OutputFormat) -> Result {
	let data = resources.find_data(path)?;
	let bytes = data.bytes()?;
	match format {
		OutputFormat::Text => {
			io::stdout().lock().write_all(bytes)?;
			Ok(())
		},
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let path = path.to_str().ok_or_else(|| err("resource path is not valid UTF-8"))?;
			let output = CatOutput {
				path,
				size: bytes.len(),
				code_page: data.code_page(),
				encoding: "base64",
				data: basenc::Base64Std.encode(bytes),
			};
			print_json(&output, format == OutputFormat::JsonPretty)
		},
	}
}

fn fsck(resources: ResourceDirectory<'_>, format: OutputFormat) -> Result {
	let inventory = inventory(resources, Path::new("/"))?;
	let report = FsckReport {
		valid: true,
		directories: inventory.directories,
		files: inventory.files,
		bytes: inventory.bytes,
	};
	match format {
		OutputFormat::Text => {
			println!("ok: {} directories, {} files, {} bytes", report.directories, report.files, report.bytes);
			Ok(())
		},
		OutputFormat::Json => print_json(&report, false),
		OutputFormat::JsonPretty => print_json(&report, true),
	}
}

fn inventory(resources: ResourceDirectory<'_>, path: &Path) -> Result<Inventory> {
	let root = resources.find_dir(path)?;
	let root_path = normalized_path(path)?;
	let mut inventory = Inventory {
		root: root_path.clone(),
		directories: 1,
		files: 0,
		bytes: 0,
		entries: Vec::new(),
	};
	let mut active = HashSet::new();
	walk(root, &root_path, 0, &mut active, &mut inventory)?;
	Ok(inventory)
}

fn walk(directory: ResourceDirectoryTable<'_>, parent: &str, depth: usize, active: &mut HashSet<usize>, inventory: &mut Inventory) -> Result {
	if depth >= 64 {
		return Err(err("resource directory nesting exceeds 64 levels"));
	}
	let identity = directory.image() as *const _ as usize;
	if !active.insert(identity) {
		return Err(err(format!("resource directory cycle below {parent}")));
	}

	for child in directory.entries() {
		let name = display_name(child.name()?, parent == "/");
		let path = join_path(parent, &name);
		match child.entry()? {
			ResourceEntry::Directory(directory) => {
				inventory.directories += 1;
				inventory.entries.push(InventoryEntry {
					path: path.clone(),
					name,
					kind: InventoryKind::Directory,
					depth,
					size: None,
					code_page: None,
				});
				walk(directory, &path, depth + 1, active, inventory)?;
			},
			ResourceEntry::Data(data) => {
				let bytes = data.bytes()?;
				inventory.files += 1;
				inventory.bytes = inventory.bytes.checked_add(bytes.len()).ok_or_else(|| err("resource byte count overflows"))?;
				inventory.entries.push(InventoryEntry {
					path,
					name,
					kind: InventoryKind::File,
					depth,
					size: Some(bytes.len()),
					code_page: Some(data.code_page()),
				});
			},
		}
	}
	active.remove(&identity);
	Ok(())
}

fn normalized_path(path: &Path) -> Result<String> {
	let path = path.to_str().ok_or_else(|| err("resource path is not valid UTF-8"))?;
	if !path.starts_with('/') && !path.starts_with('\\') {
		return Err(err("resource paths must start with '/'"));
	}
	let path = path.trim_end_matches(['/', '\\']);
	Ok(if path.is_empty() { "/".to_owned() } else { path.replace('\\', "/") })
}

fn join_path(parent: &str, name: &str) -> String {
	if parent == "/" {
		format!("/{name}")
	}
	else {
		format!("{parent}/{name}")
	}
}

fn display_name(name: ResourceName<'_>, resource_type: bool) -> String {
	match name {
		ResourceName::Id(id) if resource_type => resource_type_name(id).map(str::to_owned).unwrap_or_else(|| format!("#{id}")),
		ResourceName::Id(id) => format!("#{id}"),
		ResourceName::Wide(words) => String::from_utf16_lossy(words),
		ResourceName::Str(name) => name.to_owned(),
	}
}

fn resource_type_name(id: u32) -> Option<&'static str> {
	Some(match id {
		1 => "#CURSOR",
		2 => "#BITMAP",
		3 => "#ICON",
		4 => "#MENU",
		5 => "#DIALOG",
		6 => "#STRING",
		7 => "#FONTDIR",
		8 => "#FONT",
		9 => "#ACCELERATOR",
		10 => "#RCDATA",
		11 => "#MESSAGETABLE",
		12 => "#GROUP_CURSOR",
		14 => "#GROUP_ICON",
		16 => "#VERSION",
		17 => "#DLGINCLUDE",
		19 => "#PLUGPLAY",
		20 => "#VXD",
		21 => "#ANICURSOR",
		22 => "#ANIICON",
		23 => "#HTML",
		24 => "#MANIFEST",
		_ => return None,
	})
}
