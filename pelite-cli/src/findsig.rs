use pelite::pattern;

use super::*;

#[derive(serde::Serialize)]
struct PatternMatches {
	pattern: String,
	matches: Vec<Vec<u32>>,
}

pub fn command() -> clap::Command {
	clap::Command::new("findsig")
		.about("Find byte patterns in a PE image")
		.after_help("If no patterns are supplied, patterns are read one per line from standard input.\nPattern syntax: https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html\nExpression example: --expr \"gt(save(1), 0)\" (save(0) is the match RVA).")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("expr")
			.long("expr")
			.value_name("EXPR")
			.help("Keep matches where the pupil expression is greater than zero; save(n) reads save slot n"))
		.arg(clap::Arg::new("patterns")
			.value_name("PATTERN")
			.num_args(0..)
			.action(clap::ArgAction::Append)
			.help("Pattern to scan for"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let mut patterns: Vec<String> = matches.get_many::<String>("patterns").map(|values| values.cloned().collect()).unwrap_or_default();
	if patterns.is_empty() {
		let interactive = format == OutputFormat::Text && io::stdin().is_terminal();
		if interactive {
			println!("Interactive mode; enter one pattern per line. Press Ctrl-Z/Ctrl-D to quit.");
		}
		let stdin = io::stdin();
		let mut lines = stdin.lock().lines();
		loop {
			if interactive {
				print!(">>> ");
				io::stdout().flush()?;
			}
			let Some(line) = lines.next() else {
				break;
			};
			let line = line?;
			if !line.trim().is_empty() {
				patterns.push(line.trim().to_owned());
			}
		}
	}

	let expression = matches.get_one::<String>("expr");
	let expression_tokens = expression.map(|source| pupil::tokenize(source).collect::<Vec<_>>());
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
	let mut results = Vec::new();
	for source in patterns {
		let opts = pattern::ParseOptions::default();
		let parsed = pattern::parse(&source, opts).map_err(|error| err(format!("pattern '{source}': {error}")))?;
		let captures_len = pattern::captures_len(&parsed);
		let mut save = vec![0; pattern::save_len(&parsed)];
		let mut scanner = pe.scanner().sections(|_| true).matches(&parsed);
		let mut matches = Vec::new();
		while scanner.next(&mut save).is_some() {
			if let (Some(source), Some(tokens)) = (expression, &expression_tokens) {
				let env = MatchEnv { save: &save, builtins: pupil::BasicEnv::default() };
				let value = pupil::eval_tokens(&env, tokens);
				let value = value.map_err(|error| err(format!("expression '{source}': {error}")))?;
				if value <= 0.0 {
					continue;
				}
			}
			matches.push(save[..captures_len].to_vec());
		}
		results.push(PatternMatches { pattern: source, matches });
	}
	match format {
		OutputFormat::Json => print_json(&results, false),
		OutputFormat::JsonPretty => print_json(&results, true),
		OutputFormat::Text => {
			for result in results {
				println!("Pattern {:?} matches:", result.pattern);
				for captures in result.matches {
					if let Some((address, captures)) = captures.split_first() {
						print!("  {file_name}!{address:#010x}");
						if !captures.is_empty() {
							print!("  [");
							for (index, capture) in captures.iter().enumerate() {
								print!("{}/{capture:#010x} ", index + 1);
							}
							print!("]");
						}
						println!();
					}
				}
			}
			Ok(())
		},
	}
}

struct MatchEnv<'a> {
	save: &'a [u32],
	builtins: pupil::BasicEnv,
}

impl pupil::Env for MatchEnv<'_> {
	fn function(&self, name: &str) -> std::result::Result<pupil::Function, pupil::ErrorKind> {
		Ok(match name {
			"save" => pupil::Function::Call(0),
			_ => return self.builtins.function(name),
		})
	}

	fn call(&self, index: usize, vals: &mut [pupil::Value]) -> std::result::Result<pupil::Value, pupil::ErrorKind> {
		match index {
			0 => save_value(self, vals),
			_ => self.builtins.call(index, vals),
		}
	}

	fn value(&self, name: &str) -> std::result::Result<pupil::Value, pupil::ErrorKind> {
		self.builtins.value(name)
	}

	fn set_value(&mut self, name: &str, value: pupil::Value) -> std::result::Result<(), pupil::ErrorKind> {
		self.builtins.set_value(name, value)
	}
}

fn save_value(env: &MatchEnv<'_>, values: &mut [pupil::Value]) -> std::result::Result<pupil::Value, pupil::ErrorKind> {
	let &mut [index] = values else {
		return Err(pupil::ErrorKind::BadArgument);
	};
	if !index.is_finite() || index.fract() != 0.0 || index < 0.0 || index >= env.save.len() as f64 {
		return Err(pupil::ErrorKind::BadArgument);
	}
	return Ok(env.save[index as usize] as f64)
}
