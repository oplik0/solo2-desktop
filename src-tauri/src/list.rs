
use std::{
	collections::BTreeMap,
	panic::catch_unwind,
};
use solo2::{Solo2, UuidSelectable};
use tauri::{command, State, Window};
use crate::solo::{Solo2Info, Solo2List};
use usb_enumeration::Observer;

#[command]
pub async fn list_keys(state: State<'_, Solo2List>) -> Result<BTreeMap<String, Solo2Info>, String> {
	let mut list = state.0.lock().await;
	match catch_unwind(|| {
		Solo2::list()
			.into_iter()
			.map(|solo| (solo.uuid().to_simple().to_string().to_uppercase(), Solo2Info::from(solo)))
			.collect()
	}) {
		Ok(keys) => {
			list.clone_from(&keys);
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