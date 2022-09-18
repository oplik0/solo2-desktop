use std::{panic::catch_unwind, thread};
use std::thread::{spawn};
use solo2::{Device, Firmware, Solo2, Uuid, UuidSelectable};
use tauri::{command};


#[command]
pub fn update_key(uuid: String) -> Result<(), String> {
	let firmware = Firmware::download_latest().unwrap();
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let device = Device::having(converted_uuid).unwrap();
	match device.program(firmware, true) {
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}
