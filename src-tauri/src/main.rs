#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::{panic::catch_unwind, thread};
use std::sync::{Arc, Mutex};
use list::init_key_listing;
use solo2::{Device, Firmware, Uuid, UuidSelectable};
use tauri::{command, Manager, State};

mod list;
mod update;
fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let window = app.get_window("main").unwrap();
			tauri::async_runtime::spawn(
				init_key_listing(window)
			);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			list::list_keys,
			update::update_key
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
