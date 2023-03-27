use tauri::command;

#[cfg(windows)]
fn is_admin() -> bool {
	// based on https://stackoverflow.com/a/8196291
	let mut ret_value = false;
	unsafe {
		let mut h_token = std::mem::zeroed();
		if windows::Win32::System::Threading::OpenProcessToken(
			windows::Win32::System::Threading::GetCurrentProcess(),
			windows::Win32::Security::TOKEN_QUERY,
			&mut h_token,
		)
		.as_bool()
		{
			let mut size = std::mem::size_of::<windows::Win32::Security::TOKEN_ELEVATION>() as u32;
			let mut token_privileges_vec = vec![0u8; size as usize];
			let token_privileges =
				token_privileges_vec.as_mut_ptr() as *mut windows::Win32::Security::TOKEN_ELEVATION;

			if windows::Win32::Security::GetTokenInformation(
				h_token,
				windows::Win32::Security::TokenElevation,
				Some(token_privileges as *mut std::ffi::c_void),
				size,
				&mut size,
			)
			.as_bool()
			{
				ret_value = (*token_privileges).TokenIsElevated != 0
			}
		}
		if h_token.0 != 0 {
			windows::Win32::Foundation::CloseHandle(h_token);
		}
	}
	return ret_value;
}

#[command]
pub async fn list_fido() -> Result<(), String> {
	#[cfg(windows)]
	if !is_admin() {
		return Err("You need to run this app as administrator".to_string());
	}

	let devs = ctap_hid_fido2::get_hid_devices();
	println!("Found {} devices", devs.len());
	for dev in devs {
		if dev.vid != 0x1209 {
			continue;
		}
		println!("Device: {:?}", dev.info);
	}
	let fdevs = ctap_hid_fido2::get_fidokey_devices();
	println!("Found {} fido devices", fdevs.len());
	for fdev in fdevs {
		println!("Fido Device: {:?}", fdev.info);
	}
	Ok(())
}
