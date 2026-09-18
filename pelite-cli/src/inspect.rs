use pelite::{PeFile, Wrap, image};

use super::*;

const KEYWORDS: [&str; 11] = [
	"all",
	"dos",
	"headers",
	"sections",
	"imports",
	"exports",
	"relocations",
	"load-config",
	"tls",
	"exceptions",
	"debug",
];

pub fn command() -> clap::Command {
	clap::Command::new("inspect")
		.about("Inspect selected PE headers and directories")
		.after_help("With no keywords, every supported structure is inspected. The 'all' keyword also selects everything.\n\nExamples:\n  pelite-cli inspect program.exe sections\n  pelite-cli inspect program.exe imports exports\n  pelite-cli inspect program.exe all --format=json-pretty")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("topics")
			.value_name("KEYWORD")
			.value_parser(KEYWORDS)
			.num_args(0..)
			.help("Structures to include in the output"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let requested = matches.get_many::<String>("topics")
		.into_iter()
		.flatten()
		.map(String::as_str)
		.collect::<BTreeSet<_>>();
	let inspect_all = requested.is_empty() || requested.contains("all");
	let selected = |topic| inspect_all || requested.contains(topic);

	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	let mut output = BTreeMap::new();
	output.insert("format", value(match pe {
		Wrap::T32(_) => "PE32",
		Wrap::T64(_) => "PE32+",
	})?);
	if selected("dos") {
		output.insert("dos", value(pe.dos_header())?);
	}
	if selected("headers") {
		output.insert("headers", value(pe.headers())?);
	}
	if selected("sections") {
		output.insert("sections", value(pe.section_headers())?);
	}
	if selected("imports") {
		output.insert("imports", directory(pe.imports())?);
	}
	if selected("exports") {
		output.insert("exports", directory(pe.exports())?);
	}
	if selected("relocations") {
		output.insert("relocations", directory(pe.base_relocs())?);
	}
	if selected("load-config") {
		output.insert("load_config", directory(pe.load_config())?);
	}
	if selected("tls") {
		output.insert("tls", directory(pe.tls())?);
	}
	if selected("exceptions") {
		output.insert("exceptions", exceptions(pe)?);
	}
	if selected("debug") {
		output.insert("debug", directory(pe.debug())?);
	}

	match format {
		OutputFormat::Text => print_text("PE image", &value(output)?),
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
	}
}

fn value<T: serde::Serialize>(value: T) -> Result<serde_json::Value> {
	Ok(serde_json::to_value(value)?)
}

fn directory<T: serde::Serialize>(result: pelite::Result<T>) -> Result<serde_json::Value> {
	match result {
		Ok(directory) => value(directory),
		Err(error) if error.is_null() => Ok(serde_json::Value::Null),
		Err(error) => Err(error.into()),
	}
}

fn exceptions(pe: PeFile<'_>) -> Result<serde_json::Value> {
	match pe {
		Wrap::T32(_) => Ok(serde_json::Value::Null),
		Wrap::T64(file) => {
			use pelite::pe64::Pe;
			match file.file_header().Machine {
				image::IMAGE_FILE_MACHINE_AMD64 => directory(file.exception_x64()),
				image::IMAGE_FILE_MACHINE_ARM64 => directory(file.exception_arm64()),
				_ => Err(pelite::Error::Invalid.into()),
			}
		},
	}
}

fn print_text(title: &str, value: &serde_json::Value) -> Result {
	let stdout = io::stdout();
	let mut output = stdout.lock();
	writeln!(output, "{title}")?;
	writeln!(output, "{}", "=".repeat(title.chars().count()))?;
	render(&mut output, value, 0)?;
	Ok(())
}

fn render(output: &mut dyn Write, value: &serde_json::Value, indent: usize) -> io::Result<()> {
	match value {
		serde_json::Value::Object(object) => render_object(output, object, indent),
		serde_json::Value::Array(array) => render_array(output, array, indent),
		_ => writeln!(output, "{}{}", padding(indent), display(value)),
	}
}

fn render_object(output: &mut dyn Write, object: &serde_json::Map<String, serde_json::Value>, indent: usize) -> io::Result<()> {
	let simple = object.iter().filter(|(_, value)| is_inline(value)).collect::<Vec<_>>();
	let has_simple = !simple.is_empty();
	if has_simple {
		let width = simple.iter().map(|(key, _)| key.chars().count()).max().unwrap_or(0);
		for (key, value) in simple {
			writeln!(output, "{}{key:<width$} : {}", padding(indent), display(value))?;
		}
	}

	let mut first = !has_simple;
	for (key, value) in object.iter().filter(|(_, value)| !is_inline(value)) {
		if !first {
			writeln!(output)?;
		}
		writeln!(output, "{}{key}:", padding(indent))?;
		if value.is_null() {
			writeln!(output, "{}Not present.", padding(indent + 2))?;
		}
		else {
			render(output, value, indent + 2)?;
		}
		first = false;
	}
	Ok(())
}

fn render_array(output: &mut dyn Write, array: &[serde_json::Value], indent: usize) -> io::Result<()> {
	if array.is_empty() {
		writeln!(output, "{}[]", padding(indent))?;
		return Ok(());
	}
	if let Some(columns) = table_columns(array) {
		return render_table(output, array, &columns, indent);
	}
	if array.iter().all(is_scalar) {
		let values = array.iter().map(display).collect::<Vec<_>>();
		let joined = format!("[{}]", values.join(", "));
		if joined.chars().count() <= 120 {
			writeln!(output, "{}{joined}", padding(indent))?;
		}
		else {
			for (index, value) in values.iter().enumerate() {
				writeln!(output, "{}{index:>4}: {value}", padding(indent))?;
			}
		}
		return Ok(());
	}

	for (index, value) in array.iter().enumerate() {
		if index != 0 {
			writeln!(output)?;
		}
		writeln!(output, "{}[{index}]", padding(indent))?;
		render(output, value, indent + 2)?;
	}
	Ok(())
}

fn table_columns(array: &[serde_json::Value]) -> Option<Vec<String>> {
	let mut columns = BTreeSet::new();
	for value in array {
		let serde_json::Value::Object(object) = value else {
			return None;
		};
		if object.values().any(|value| !is_inline(value)) {
			return None;
		}
		columns.extend(object.keys().cloned());
	}
	(!columns.is_empty() && columns.len() <= 12).then(|| columns.into_iter().collect())
}

fn render_table(output: &mut dyn Write, array: &[serde_json::Value], columns: &[String], indent: usize) -> io::Result<()> {
	let rows = array.iter().map(|value| {
		let object = value.as_object().expect("validated by table_columns");
		columns.iter().map(|column| object.get(column).map(display).unwrap_or_default()).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	let widths = columns.iter().enumerate().map(|(column, name)| {
		rows.iter().map(|row| row[column].chars().count()).fold(name.chars().count(), usize::max)
	}).collect::<Vec<_>>();
	let prefix = padding(indent);

	write!(output, "{prefix}")?;
	write_row(output, columns, &widths)?;
	writeln!(output)?;
	write!(output, "{prefix}")?;
	for (index, width) in widths.iter().enumerate() {
		if index != 0 {
			write!(output, "-+-")?;
		}
		write!(output, "{}", "-".repeat(*width))?;
	}
	writeln!(output)?;
	for row in rows {
		write!(output, "{prefix}")?;
		write_row(output, &row, &widths)?;
		writeln!(output)?;
	}
	Ok(())
}

fn write_row(output: &mut dyn Write, cells: &[String], widths: &[usize]) -> io::Result<()> {
	for (index, cell) in cells.iter().enumerate() {
		if index != 0 {
			write!(output, " | ")?;
		}
		if index + 1 == cells.len() {
			write!(output, "{cell}")?;
		}
		else {
			write!(output, "{cell:<width$}", width = widths[index])?;
		}
	}
	Ok(())
}

fn is_inline(value: &serde_json::Value) -> bool {
	is_scalar(value) || matches!(value, serde_json::Value::Array(array) if array.iter().all(is_scalar))
}

fn is_scalar(value: &serde_json::Value) -> bool {
	matches!(value, serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) | serde_json::Value::String(_))
}

fn display(value: &serde_json::Value) -> String {
	match value {
		serde_json::Value::Null => "null".to_owned(),
		serde_json::Value::Bool(value) => value.to_string(),
		serde_json::Value::Number(value) => match value.as_u64() {
			Some(value) if value >= 10 => format!("{value} (0x{value:x})"),
			_ => value.to_string(),
		},
		serde_json::Value::String(value) if value.is_empty() => "\"\"".to_owned(),
		serde_json::Value::String(value) => value.clone(),
		serde_json::Value::Array(array) => format!("[{}]", array.iter().map(display).collect::<Vec<_>>().join(", ")),
		serde_json::Value::Object(_) => unreachable!("objects are not rendered inline"),
	}
}

fn padding(indent: usize) -> String {
	" ".repeat(indent)
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn flat_object_arrays_become_tables() {
		let value = json!([{ "Name": ".text", "VirtualAddress": 4096 }]);
		assert_eq!(table_columns(value.as_array().unwrap()).unwrap(), ["Name", "VirtualAddress"]);
	}

	#[test]
	fn nested_object_arrays_remain_structured() {
		let value = json!([{ "dll": "kernel32.dll", "image": { "FirstThunk": 4096 } }]);
		assert!(table_columns(value.as_array().unwrap()).is_none());
	}
}
