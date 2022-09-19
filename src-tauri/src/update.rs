use solo2::{Device, Firmware, Uuid, UuidSelectable};
use tauri::{command, State};

use crate::solo::Solo2List;


#[command]
pub async fn update_key(uuid: String, state: State<'_, Solo2List>) -> Result<(), String> {
	let list = state.0.lock().await;
	if list.contains_key(&uuid) {
		let firmware = Firmware::download_latest().unwrap();
		let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
		let device = Device::having(converted_uuid).unwrap();
		match device.program(firmware, true) {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string()),
		}
	} else {
		println!("List: {:?}", list);
		Err("Key not found".to_string())
	}
}
