use pelite::PeFile;

static IP_OVER_USB_SVC: &[u8] = include_bytes!("IpOverUsbSvc.exe");

#[test]
fn test() {
    let pefile = PeFile::from_bytes(IP_OVER_USB_SVC).unwrap();
    let imports = pefile.imports().expect("Grabbing the import table to work.");
}