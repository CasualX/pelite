/*!
Analyzing strings in binary data.
 */

#[derive(Copy, Clone, Debug)]
pub enum Heuristic {
    /// Printable ascii heuristic.
    ///
    /// Considers strings valid if all characters are printable ascii characters.
    /// Allows TAB, LF, CR and everything from space to tilde.
    PrintableAscii,
}

/// Configure the string finding heuristics.
#[derive(Clone, Debug)]
// #[non_exhaustive]
pub struct Config {
    /// Minimum string length to accept as a valid string.
    pub min_length: u8,
    /// Minimum string length when there is a nul terminator.
    /// Can have a lower threshold as having a nul terminator increases confidence that this is a c string literal.
    pub min_length_nul: u8,
    /// When true, requires any found string to be terminated by a nul terminator.
    /// A nul terminator increases confidence that this is indeed a c string literal.
    pub strict_nul: bool,
    /// Heuristic to use to validate sequences.
    pub heuristic: Heuristic,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            min_length: 6,
            min_length_nul: 3,
            strict_nul: true,
            heuristic: Heuristic::PrintableAscii,
        }
    }
}
impl Config {
    /// Constructs the [enumerator](struct.Enumerator.html) with this configuration.
    ///
    /// Given the `base` argument the relative virtual address of the `bytes` slice.
    pub fn enumerate(self, base: u32, bytes: &'_ [u8]) -> Enumerator<'_> {
        Enumerator {
            base,
            offset: 0,
            bytes,
            config: self,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Found<'a> {
    pub string: &'a [u8],
    pub address: u32,
    pub has_nul: bool,
}
impl<'a> Found<'a> {
    pub fn nul(string: &'a [u8], address: u32) -> Found<'a> {
        Found {
            string,
            address,
            has_nul: true,
        }
    }
    pub fn non_nul(string: &'a [u8], address: u32) -> Found<'a> {
        Found {
            string,
            address,
            has_nul: false,
        }
    }
}

fn is_printable_ascii(byte: u8) -> bool {
    if byte >= 0x20 {
        byte < 0x80
    } else {
        (1 << byte as u32) & (1 << b'\n' | 1 << b'\r' | 1 << b'\t') != 0
    }
}

/// Iterator over the strings in binary data.
#[derive(Clone)]
pub struct Enumerator<'a> {
    base: u32,
    offset: u32,
    bytes: &'a [u8],
    config: Config,
}
impl<'a> Iterator for Enumerator<'a> {
    type Item = Found<'a>;
    fn next(&mut self) -> Option<Found<'a>> {
        let mut start = self.offset as usize;
        let mut i = start;
        let bytes = self.bytes;
        match self.config.heuristic {
            Heuristic::PrintableAscii => {
                while i < bytes.len() {
                    if is_printable_ascii(bytes[i]) {
                        i += 1;
                        continue;
                    } else if bytes[i as usize] == b'\0' {
                        if i - start >= self.config.min_length_nul as usize {
                            self.offset = (i + 1) as u32;
                            return Some(Found::nul(&bytes[start..i], self.base + start as u32));
                        }
                    } else if !self.config.strict_nul {
                        if i - start >= self.config.min_length as usize {
                            self.offset = (i + 1) as u32;
                            return Some(Found::non_nul(
                                &bytes[start..i],
                                self.base + start as u32,
                            ));
                        }
                    }
                    i += 1;
                    start = i;
                }
                if start != i {
                    if !self.config.strict_nul && i - start >= self.config.min_length as usize {
                        self.offset = i as u32;
                        return Some(Found::non_nul(&bytes[start..i], self.base + start as u32));
                    }
                }
            }
        }
        None
    }
}

#[test]
fn testing() {
    let bytes = b"\x1fC-STRING\0\x80\x81AAAAAAAAAA\xff";
    let strings: Vec<_> = Config {
        strict_nul: false,
        ..Config::default()
    }
    .enumerate(0x1000, bytes)
    .collect();
    assert_eq!(
        strings,
        vec![
            Found {
                string: b"C-STRING",
                address: 0x1000 + 1,
                has_nul: true
            },
            Found {
                string: b"AAAAAAAAAA",
                address: 0x1000 + 12,
                has_nul: false
            },
        ]
    );
}
