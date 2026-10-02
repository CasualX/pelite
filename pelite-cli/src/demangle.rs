use super::*;

pub fn command() -> clap::Command {
	clap::Command::new("demangle")
		.about("Demangle compiler-generated symbol names")
		.after_help(include_str!("docs/demangle.md"))
		.arg(clap::Arg::new("symbol")
			.value_name("SYMBOL")
			.num_args(1..)
			.required(true)
			.help("Mangled symbol names"))
		.arg(clap::Arg::new("abi")
			.long("abi")
			.value_name("ABI")
			.value_parser(["msvc"])
			.required(true)
			.help("Select the symbol ABI"))
		.arg(clap::Arg::new("flags")
			.long("flags")
			.value_name("FLAGS")
			.help("Comma-separated ABI-specific demangler flags"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let symbols = matches.get_many::<String>("symbol").expect("required by clap");
	let abi = matches.get_one::<String>("abi").expect("required by clap");

	let flags = matches.get_one::<String>("flags");
	let msvc_flags = match abi.as_str() {
		"msvc" => flags.map(|flags| parse_msvc_flags(flags)).transpose()?
			.unwrap_or_else(msvc_demangler::DemangleFlags::llvm),
		_ => unreachable!(),
	};

	let mut results = Vec::new();
	let mut failed = false;
	for symbol in symbols {
		let demangled = match abi.as_str() {
			"msvc" => msvc_demangler::demangle(symbol, msvc_flags),
			_ => unreachable!(),
		};
		match demangled {
			Ok(demangled) => results.push((symbol, demangled)),
			Err(error) => {
				eprintln!("pelite-cli: cannot demangle {symbol:?} with {abi}: {error}");
				failed = true;
			},
		}
	}

	match format {
		OutputFormat::Nul => {},
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			for (_, demangled) in &results {
				writeln!(output, "{demangled}")?;
			}
		},
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let results: Vec<_> = results.iter()
				.map(|(symbol, demangled)| BTreeMap::from([(symbol.as_str(), demangled.as_str())]))
				.collect();
			print_json(&results, format == OutputFormat::JsonPretty)?;
		},
	}

	io::stdout().flush()?;
	if failed {
		Err(err("one or more symbols could not be demangled"))
	}
	else {
		Ok(())
	}
}

fn parse_msvc_flags(input: &str) -> Result<msvc_demangler::DemangleFlags> {
	let mut flags = msvc_demangler::DemangleFlags::empty();
	if input.trim().is_empty() {
		return Ok(flags);
	}
	for name in input.split(',') {
		let name = name.trim();
		let flag = msvc_demangler::DemangleFlags::from_name(name)
			.ok_or_else(|| err(format!("invalid msvc flag {name:?}")))?;
		flags |= flag;
	}
	Ok(flags)
}
