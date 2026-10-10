use super::*;

mod addr;
mod address;
mod analysis;
mod demangle;
mod disasm;
mod disasm_raw;
mod findsig;
mod hex;
mod hexdump;
mod iced;
mod msvc;
mod read;
mod brief;
mod rust;
mod scan;
mod strings;
mod symbol;
mod symbols;
mod xref;

use address::Address;
use binfact::*;

fn pointer_width(pe: pelite::PeFile<'_>) -> ty::PointerWidth {
	match pe {
		pelite::Wrap::T32(_) => ty::PointerWidth::Bits32,
		pelite::Wrap::T64(_) => ty::PointerWidth::Bits64,
	}
}

impl Address {
	fn to_rva(self, pe: pelite::PeFile<'_>) -> Result<u32> {
		match self {
			Address::Rva(rva) => Ok(rva),
			Address::Va(va) => Ok(pe.va_to_rva(va)?),
			Address::Fo(fo) => Ok(pe.headers().file_offset_to_rva(fo)?),
		}
	}
}

fn get_arch(matches: &clap::ArgMatches, pe: pelite::PeFile<'_>) -> Result<Arch> {
	if let Some(arch) = matches.get_one::<Arch>("arch") {
		Ok(*arch)
	}
	else {
		match pe.file_header().Machine {
			pelite::image::IMAGE_FILE_MACHINE_I386 => Ok(Arch::X86_32),
			pelite::image::IMAGE_FILE_MACHINE_AMD64 => Ok(Arch::X86_64),
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		}
	}
}

pub fn command() -> clap::Command {
	let command = clap::Command::new("re")
		.about("Reverse engineer code and data in binaries")
		.after_help(include_str!("docs/re.md"))
		.arg_required_else_help(true)
		.subcommand(addr::command())
		.subcommand(symbol::command())
		.subcommand(read::command())
		.subcommand(scan::command())
		.subcommand(disasm::command())
		.subcommand(brief::command())
		.subcommand(disasm_raw::command())
		.subcommand(hexdump::command())
		.subcommand(strings::command())
		.subcommand(findsig::command())
		.subcommand(xref::command())
		.subcommand(analysis::command())
		.subcommand(demangle::command())
		.subcommand(msvc::command())
		.subcommand(rust::command());

	with_command_guide(command)
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	match matches.subcommand() {
		Some(("addr", matches)) => addr::run(matches, format),
		Some(("symbol", matches)) => symbol::run(matches, format),
		Some(("read", matches)) => read::run(matches, format),
		Some(("scan", matches)) => scan::run(matches, format),
		Some(("disasm", matches)) => disasm::run(matches, format),
		Some(("brief", matches)) => brief::run(matches, format),
		Some(("disasm-raw", matches)) => disasm_raw::run(matches, format),
		Some(("hexdump", matches)) => hexdump::run(matches, format),
		Some(("strings", matches)) => strings::run(matches, format),
		Some(("findsig", matches)) => findsig::run(matches, format),
		Some(("xref", matches)) => xref::run(matches, format),
		Some(("analysis", matches)) => analysis::run(matches, format),
		Some(("demangle", matches)) => demangle::run(matches, format),
		Some(("msvc", matches)) => msvc::run(matches, format),
		Some(("rust", matches)) => rust::run(matches, format),
		_ => unreachable!("all reverse engineering subcommands are handled"),
	}
}
