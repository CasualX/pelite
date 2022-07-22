/*!
Fun with MS Api Set Schemas

Resources:

* https://ofekshilon.com/2016/03/27/on-api-ms-win-xxxxx-dll-and-other-dependency-walker-glitches/
* https://blog.quarkslab.com/runtime-dll-name-resolution-apisetschema-part-i.html
* https://blog.quarkslab.com/runtime-dll-name-resolution-apisetschema-part-ii.html
* https://lucasg.github.io/2017/10/15/Api-set-resolution/
* https://www.geoffchappell.com/studies/windows/win32/apisetschema/index.htm

 */

pub mod image;
pub mod win10;

fn main() {
    let filemap = pelite::FileMap::open(r"C:\Windows\System32\ApiSetSchema.dll").unwrap();
    let pefile = pelite::PeFile::from_bytes(&filemap).unwrap();
    let section = pefile.section_headers().by_name(".apiset").unwrap();
    println!("Section: {:#x?}", section);
    let apisetschema = win10::Schema::parse(pefile.get_section_bytes(section).unwrap());
    println!("Contents: {:#?}", apisetschema);
}
