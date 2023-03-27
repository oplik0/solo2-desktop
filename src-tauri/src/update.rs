use serde::Serialize;
use solo2::{Device, Firmware, Uuid, UuidSelectable};
use tauri::{command, State, Window};

use crate::solo::Solo2List;

#[derive(Serialize, Clone)]
struct ProgressData {
	completed: usize,
	total: usize,
	uuid: String,
}

#[command]
pub async fn update_key(
	uuid: String,
	file: Option<String>,
	state: State<'_, Solo2List>,
	window: Window,
) -> Result<(), String> {
	let list = state.0.lock().await;
	if list.contains_key(&uuid) {
		let firmware = match file {
			Some(file) => {
				let path = std::path::Path::new(&file);
				if !path.exists()
					|| path.extension().is_none()
					|| path.extension().unwrap().to_str() == Some("bin")
				{
					return Err("Invalid firmware file".to_string());
				}
				Firmware::read_from_file(file).unwrap()
			}
			None => Firmware::download_latest().unwrap(),
		};
		let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
		let device = Device::having(converted_uuid).unwrap();
		let total = firmware.len() as u64;
		let progress = |bytes: usize| {
			window
				.emit(
					"update_progress",
					ProgressData {
						completed: bytes,
						total: total as usize,
						uuid: uuid.clone(),
					},
				)
				.unwrap();
		};

		match device.program(firmware, true, Some(&progress)) {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string()),
		}
	} else {
		Err("Key not found".to_string())
	}
}
