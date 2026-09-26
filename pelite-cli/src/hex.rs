use super::*;

pub struct HexPrinter {
	color: bool,
}

impl HexPrinter {
	pub fn new(color: bool) -> HexPrinter {
		HexPrinter { color }
	}
	pub fn for_output(output: &impl io::IsTerminal) -> HexPrinter {
		let color = output.is_terminal();
		HexPrinter { color }
	}

	pub fn write_byte(&self, output: &mut impl Write, byte: u8) -> io::Result<()> {
		if self.color {
			let (red, green, blue) = gradient(byte);
			write!(output, "\x1b[38;2;{red};{green};{blue}m{byte:02x}\x1b[0m")
		}
		else {
			write!(output, "{byte:02x}")
		}
	}

	pub fn write_bytes(&self, output: &mut impl Write, bytes: &[u8], sep: &[u8]) -> io::Result<()> {
		for (index, &byte) in bytes.iter().enumerate() {
			if index != 0 {
				if sep.len() > 0 {
					output.write_all(sep)?;
				}
			}
			self.write_byte(output, byte)?;
		}
		Ok(())
	}
}

// Dark slate blue through a muted sage, indexed by the byte value.
fn gradient(byte: u8) -> (u8, u8, u8) {
	fn lerp(start: u8, end: u8, byte: u8) -> u8 {
		start + (u16::from(end - start) * u16::from(byte) / 255) as u8
	}
	(lerp(50, 255, byte), lerp(62, 255, byte), lerp(75, 255, byte))
}
