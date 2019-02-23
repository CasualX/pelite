
use pelite;
use pelite::pe64::{Va, Ptr, Pe, PeFile};
use pelite::{util::CStr, Pod};
use pelite::pattern as pat;

//----------------------------------------------------------------

pub fn print(bin: PeFile) {
	for class in classes(bin).unwrap() {
		print!("[ClientClass.{:?}]\n\taddress = {:#x}\n\tsize = {}\n", class.name, class.address, class.size);
	}
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
	pub size: u32,
}

pub fn classes<'a>(bin: PeFile<'a>) -> pelite::Result<Vec<Class<'a>>> {
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
	let pat = pat::parse("488B05${'} 488905${'} 488D05${'} 488905${'} C3").unwrap();

	let mut matches = bin.scanner().matches_code(&pat);
	while matches.next(&mut save) {
		// Remove false positives
		if save[1] != save[4] || save[2] != save[3] + 0x20 {
			continue;
		}
		// Now dealing with a ClientClass
		let address = save[3];
		let client_class: &ClientClass = bin.derva(address).unwrap();
		let name = bin.deref_c_str(client_class.pNetworkName).unwrap().to_str().unwrap();
		let size = client_class.SizeOfClass;
		list.push(Class { name, address, size })
	}

	list.sort_by_key(|item| item.name);

	Ok(list)
}
