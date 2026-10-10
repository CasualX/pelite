use super::*;

fn resource_name(name: &str) -> Result<pelite::resources::ResourceName<'_>, &'static str> {
	if let Some(id) = name.strip_prefix('#') {
		return id.parse().map(pelite::resources::ResourceName::Id).map_err(|_| "invalid resource id");
	}
	Ok(pelite::resources::ResourceName::Str(name))
}

#[unsafe(export_name = "pefileResourcesTree")]
pub unsafe fn resources_tree(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	return_pelite_result(pefile.resources());
}

#[unsafe(export_name = "pefileResourcesGetResource")]
pub unsafe fn resources_get_resource(pefile: *mut PeFile, path: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	let path = match str::from_utf8(unsafe { &*path }) {
		Ok(path) => path,
		Err(err) => return return_error(err),
	};
	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};
	let bytes = resources.find_data(path).and_then(|entry| entry.bytes().map_err(Into::into));
	match bytes {
		Ok(bytes) => return_bytes(bytes),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

fn return_resource_names<'a>(values: impl Iterator<Item = Result<(pelite::resources::ResourceName<'a>, pelite::resources::group::ResourceGroup<'a>), pelite::resources::ResourceFindError>>) {
	let names: Result<Vec<_>, _> = values.map(|value| value.map(|(name, _)| name)).collect();
	return_resource_find_result(names);
}

#[unsafe(export_name = "pefileListIcons")]
pub unsafe fn resources_list_icons(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	match pefile.resources() {
		Ok(resources) => return_resource_names(resources.icons()),
		Err(pelite::Error::Null) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileGetIcon")]
pub unsafe fn resources_get_icon(pefile: *mut PeFile, name: *const [u8]) {
	unsafe { resources_get_group(pefile, name, true) }
}

#[unsafe(export_name = "pefileListCursors")]
pub unsafe fn resources_list_cursors(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	match pefile.resources() {
		Ok(resources) => return_resource_names(resources.cursors()),
		Err(pelite::Error::Null) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileGetCursor")]
pub unsafe fn resources_get_cursor(pefile: *mut PeFile, name: *const [u8]) {
	unsafe { resources_get_group(pefile, name, false) }
}

unsafe fn resources_get_group(pefile: *mut PeFile, name: *const [u8], icon: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	let name = match str::from_utf8(unsafe { &*name }) {
		Ok(name) => name,
		Err(err) => return return_error(err),
	};
	let name = match resource_name(name) {
		Ok(name) => name,
		Err(err) => return return_error(err),
	};
	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};
	let bytes = if icon {
		resources.find_icon(name).and_then(|group| group.to_vec())
	}
	else {
		resources.find_cursor(name).and_then(|group| group.to_vec())
	};
	match bytes {
		Ok(bytes) => return_bytes(&bytes),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileResourcesManifest")]
pub unsafe fn resources_manifest(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};

	match resources.manifest() {
		Ok(manifest) => return_str(manifest),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileResourcesVersionInfo")]
pub unsafe fn resources_version_info(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};

	return_resource_find_result(resources.version_info());
}
