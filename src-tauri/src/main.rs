#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use solo::Solo2List;
use tauri::Manager;
mod solo;
mod update;
mod list;
fn main() {
	tauri::Builder::default()
		.manage(Solo2List(Default::default()))
		.setup(|app| {
			let window = app.get_window("main").unwrap();
			tauri::async_runtime::spawn(list::init_key_listing(window));
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			list::list_keys,
			update::update_key
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
