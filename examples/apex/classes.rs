
use pelite;
use pelite::pe64::{Va, Ptr, Pe, PeFile};
use pelite::{util::CStr, Pod};
use pelite::pattern as pat;

//----------------------------------------------------------------

pub fn print(bin: PeFile, dll_name: &str) {
	let classes = classes(bin);

	println!("## ClientClasses\n");
	for cls in &classes {
		println!("<details>");
		println!("<summary><code>client_class {}</code></summary>\n", cls.name);
		println!("class_id: `{}`  ", cls.id);
		println!("sizeof: `{}`  ", cls.size);
		println!("</details>");
	}
	println!("\n### Addresses\n\n```");
	for cls in &classes {
		println!("{}!{:#010x} ClientClass {}", dll_name, cls.address, cls.name);
	}
	println!("```\n");
}

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
struct ClientClass {
	pCreateFn: Ptr,
	pCreateEventFn: Ptr,
	pNetworkName: Ptr<CStr>,
	pRecvTable: Va,
	pNext: Ptr<ClientClass>,
	ClassID: i32,
	SizeOfClass: u32,
}

//----------------------------------------------------------------

#[derive(Debug)]
pub struct Class<'a> {
	pub name: &'a str,
	pub address: u32,
	pub id: i32,
	pub size: u32,
}

pub fn classes<'a>(bin: PeFile<'a>) -> Vec<Class<'a>> {
	let mut save = [0; 8];
	let mut list = Vec::new();

	// The ClientClasses aren't fully constructed yet, find these constructors
	// ```
	// mov     rax, g_pClientClassHead
	// mov     s_ClientClass.pNext, rax
	// lea     rax, s_ClientClass
	// mov     g_pClientClassHead, rax
	// retn
	// ```
	let mut matches = bin.scanner().matches_code(pat!("488B05${'} 488905${'} 488D05${'} 488905${'} C3"));
	while matches.next(&mut save) {
		// Remove false positives
		if save[1] != save[4] || save[2] != save[3] + 0x20 {
			continue;
		}
		// Now dealing with a ClientClass
		let address = save[3];
		let client_class: &ClientClass = bin.derva(address).unwrap();
		let name = bin.deref_c_str(client_class.pNetworkName).unwrap().to_str().unwrap();
		let id = client_class.ClassID;
		let size = client_class.SizeOfClass;
		list.push(Class { name, address, id, size })
	}

	list.sort_by_key(|item| item.name);
	list
}
