/*!
The ClientClass links client and server entities.
*/

use pelite;
use pelite::pe32::{Va, Ptr, Pe, PeFile};
use pelite::util::{CStr, Pod};
use pelite::pattern as pat;

//----------------------------------------------------------------

pub fn print(client: PeFile) {
	for class in classes(client).unwrap() {
		println!("{:?}", class);
	}
}

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[derive(Debug)]
#[repr(C)]
struct ClientClass {
	pCreateFn: Ptr,
	pCreateEventFn: Ptr,
	pNetworkName: Ptr<CStr>,
	pRecvTable: Va,
	pNext: Ptr<ClientClass>,
	ClassID: i32,
}
unsafe impl Pod for ClientClass {}

//----------------------------------------------------------------

#[derive(Debug)]
pub struct Class<'a> {
	pub network_name: &'a str,
	pub class_id: i32,
	pub size_of: u32,
}

pub fn classes<'a>(client: PeFile<'a>) -> pelite::Result<Vec<Class<'a>>> {
	let mut save = [0; 8];
	let mut list = Vec::new();

	// The ClientClasses aren't fully constructed yet, find these constructors
	// ```
	// mov     eax, g_pClientClassHead
	// mov     s_ClientClass.pNext, eax
	// mov     g_pClientClassHead, offset s_ClientClass
	// retn
	// ```
	let pat = pat::parse("A1*{'} A3*{'} C705*{'}*{'???? ???? *{'}} C3").unwrap();
	let mut matches = client.scanner().matches_code(&pat);
	while matches.next_match(&mut save) {
		// Remove false positives
		if save[1] != save[3] || save[2] != save[4] + 0x10 {
			continue;
		}
		// Now dealing with a ClientClass
		let client_class: &ClientClass = client.derva(save[4]).unwrap();
		let network_name = client.deref_c_str(client_class.pNetworkName).unwrap().to_str().unwrap();
		// Figure out the size of the entity type:
		// The CreateFn is a function to create instances of this entity type, it allocates memory and thus includes its size
		let size_of = client.deref_copy(client_class.pCreateFn.shift(39)).unwrap_or(0);
		// Class ids are initialized somewhere else...
		let class_id = 0;
		list.push(Class { network_name, class_id, size_of })
	}

	Ok(list)
}
