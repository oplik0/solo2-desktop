use std::{
	collections::BTreeMap,
	panic::catch_unwind,
};
use std::sync::{Arc, Mutex};
use solo2::apps::Admin;
use solo2::Select;
use solo2::Uuid;
use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use memoize::memoize;
use solo2::{Solo2, UuidSelectable, Version};
use tauri::{command, State, Window};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
// impl PartialEq for Solo2Info {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.uuid == other.uuid && self.version == other.version
// 	}
// }

#[command]
pub fn list_keys() -> Result<BTreeMap<String, Solo2Info>, String> {
	match catch_unwind(|| {
		Solo2::list()
			.into_iter()
			.map(|solo| (solo.uuid().to_simple().to_string(), Solo2Info::from(solo)))
			.collect()
	}) {
		Ok(keys) => Ok(keys),
		Err(_e) => Err("Error while listing keys".to_string()),
	}
}
#[memoize]
pub fn get_secure_status(uuid: String) -> Option<bool> {
	let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
	let mut device = Solo2::having(converted_uuid).unwrap();
	let mut admin = Admin::select(&mut device).unwrap();
	Some(admin.locked().unwrap())
	// let mut result = None;
	// match ureq::get(&format!("https://s2pki.net/s2/{}/x255.txt", uuid))
	// 	.set("User-Agent", "solo2-gui")
	// 	.call()
	// {
	// 	Ok(_response) => result = Some(true),
	// 	Err(ureq::Error::Status(code, _response)) => {
	// 		if code == 404 {
	// 			result = Some(false);
	// 		}
	// 	}
	// 	Err(_) => {}
	// };
	// result
}

pub async fn init_key_listing(window: Window) {
	let mut emitted = false;
		let mut last_list: BTreeMap<String, Solo2Info> = BTreeMap::new();
		loop {
			let list: BTreeMap<String, Solo2Info> = list_keys().unwrap_or_default();
			if list != last_list {
				last_list = list.clone();
				emitted = false;
			}
			if !emitted {
				println!("current list: {:?}", list);
				match window.emit("list_keys", list.clone()) {
					Ok(_) => {
						emitted = true;
					}
					Err(_) => {
						emitted = false;
					}
				};
			}
			sleep(Duration::from_millis(10)).await;
		}
}
