// Provide implementations for image GUID here
// FIXME! Should I keep this GUID implementation or defer to another GUID library?

use crate::image::GUID;
use std::fmt;

#[inline(always)]
fn group(guid: &GUID) -> (u32, u16, u16, u16, u64) {
    let g1 = guid.Data1;
    let g2 = guid.Data2;
    let g3 = guid.Data3;
    // Mind the (little-) endianness
    let g4 = (guid.Data4[0] as u16) << 8 | guid.Data4[1] as u16;
    let g5 = (guid.Data4[2] as u64) << 8 * 5
        | (guid.Data4[3] as u64) << 8 * 4
        | (guid.Data4[4] as u64) << 8 * 3
        | (guid.Data4[5] as u64) << 8 * 2
        | (guid.Data4[6] as u64) << 8 * 1
        | (guid.Data4[7] as u64) << 8 * 0;
    (g1, g2, g3, g4, g5)
}

fn lower_dashed(guid: &GUID, f: &mut fmt::Formatter) -> fmt::Result {
    let (g1, g2, g3, g4, g5) = group(guid);
    write!(
        f,
        "{{{:08x}-{:04x}-{:04x}-{:04x}-{:012x}}}",
        g1, g2, g3, g4, g5
    )
}
// fn upper_dashed(guid: &GUID, f: &mut fmt::Formatter) -> fmt::Result {
// 	let (g1, g2, g3, g4, g5) = group(guid);
// 	write!(f, "{{{:08X}-{:04X}-{:04X}-{:04X}-{:012X}}}", g1, g2, g3, g4, g5)
// }
fn lower_hex(guid: &GUID, f: &mut fmt::Formatter) -> fmt::Result {
    let (g1, g2, g3, g4, g5) = group(guid);
    write!(f, "{:08x}{:04x}{:04x}{:04x}{:012x}", g1, g2, g3, g4, g5)
}
fn upper_hex(guid: &GUID, f: &mut fmt::Formatter) -> fmt::Result {
    let (g1, g2, g3, g4, g5) = group(guid);
    write!(f, "{:08X}{:04X}{:04X}{:04X}{:012X}", g1, g2, g3, g4, g5)
}

/// example: `{00000000-0000-0000-c000-000000000046}`
impl fmt::Display for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lower_dashed(self, f)
    }
}
/// example: `{00000000-0000-0000-c000-000000000046}`
impl fmt::Debug for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lower_dashed(self, f)
    }
}
/// example: `0000000000000000c000000000000046`
impl fmt::LowerHex for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        lower_hex(self, f)
    }
}
/// example: `0000000000000000C000000000000046`
impl fmt::UpperHex for GUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        upper_hex(self, f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for GUID {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
