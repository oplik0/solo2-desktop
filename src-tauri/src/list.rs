
use std::{
	collections::BTreeMap,
	panic::catch_unwind,
};
use solo2::{Solo2, UuidSelectable, firmware::github};
use tauri::{command, State, Window};
use crate::solo::{Solo2Info, Solo2List};
use usb_enumeration::Observer;

#[command]
pub async fn list_keys(state: State<'_, Solo2List>) -> Result<BTreeMap<String, Solo2Info>, String> {
	let mut list = state.0.lock().await;
	match get_key_list() {
		Ok(keys) => {
			list.clone_from(&keys);
			Ok(keys)
		},
		Err(_e) => Err("Error while listing keys".to_string()),
	}
}

pub fn get_key_list() -> Result<BTreeMap<String, Solo2Info>, String> {
	match catch_unwind(|| {
		Solo2::list()
			.into_iter()
			.map(|solo| (solo.uuid().simple().to_string().to_uppercase(), Solo2Info::from(solo)))
			.collect()
	}) {
		Ok(keys) => {
			Ok(keys)
		},
		Err(_e) => Err("Error while listing keys".to_string()),
	}
}

pub async fn init_key_listing(window: Window) {
	let sub = Observer::new()
	.with_vendor_id(0x1209)
    .subscribe();
	for _event in sub.rx_event.iter() {
		window.emit("usb_change", false).unwrap_or_else(|e| {
			println!("Error while emitting usb_change: {:?}", e);
		});
	}
}

#[command]
pub async fn latest_version() -> Result<String, String> {
	match github::Release::fetch_spec() {
		Ok(release) => Ok(release.tag),
		Err(e) => Err(format!("Error while fetching latest version: {:?}", e)),
	}
}

pub async fn init_fetching_latest_version(window:Window) {
	loop {
		let version = latest_version().await.unwrap_or("0.0.0".to_string());
		window.emit("latest_version", version).unwrap_or_else(|e| {
			println!("Error while emitting latest_version: {:?}", e);
		});
		tokio::time::sleep(std::time::Duration::from_secs(15 * 60)).await;
	}
}