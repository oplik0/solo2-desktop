use std::collections::BTreeMap;

use memoize::memoize;
use serde::{Serialize, Deserialize};
use solo2::{Solo2, Version, Admin, Uuid, Select as _, UuidSelectable};
use tauri::{command, State};
use tokio::sync::{Mutex};

#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
pub struct Solo2Info {
	pub uuid: String,
	pub version: Version,
	pub secure: Option<bool>,
}
impl From<Solo2> for Solo2Info {
	fn from(solo2: Solo2) -> Solo2Info {
		let uuid = solo2.uuid().to_simple().to_string();
		Solo2Info {
			uuid: uuid.to_uppercase(),
			version: solo2.version(),
			secure: get_secure_status(uuid.to_ascii_lowercase()),
		}
	}
}
impl Default for Solo2Info {
	fn default() -> Self {
		Solo2Info {
			uuid: String::default(),
			version: Version {
				major: 0,
				minor: 0,
				patch: 0,
			},
			secure: None,
		}
	}
}
impl PartialEq for Solo2Info {
	fn eq(&self, other: &Self) -> bool {
		self.uuid == other.uuid && self.version == other.version
	}
}
pub struct Solo2List(pub Mutex<BTreeMap<String, Solo2Info>>);

// impl Solo2List {
// 	pub fn new() {
// 		Solo2List{
// 			list: Mutex::new(BTreeMap::new())
// 		};
// 	}
	
// }

#[memoize]
pub fn get_secure_status(uuid: String) -> Option<bool> {
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let mut device = Solo2::having(converted_uuid).unwrap();
	let mut admin = Admin::select(&mut device).unwrap();
	Some(admin.locked().unwrap())
}

#[command]
pub async fn wink(uuid: String, state: State<'_, Solo2List>) -> Result<(), String> {
	let _list = state.0.lock().await;
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let mut device = Solo2::having(converted_uuid).unwrap();
	let mut admin = Admin::select(&mut device).unwrap();
	match admin.wink() {
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}
#[command]
pub async fn reboot(uuid: String, state: State<'_, Solo2List>) -> Result<(), String> {
	let _list = state.0.lock().await;
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let mut device = Solo2::having(converted_uuid).unwrap();
	let mut admin = Admin::select(&mut device).unwrap();
	match admin.reboot() {
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}
#[command]
pub async fn maintenance(uuid: String, state: State<'_, Solo2List>) -> Result<(), String> {
	let _list = state.0.lock().await;
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let mut device = Solo2::having(converted_uuid).unwrap();
	let mut admin = Admin::select(&mut device).unwrap();
	match admin.maintenance(){
		Ok(_) => Ok(()),
		Err(e) => Err(e.to_string()),
	}
}