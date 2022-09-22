#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use solo::Solo2List;
use tauri::Manager;
#[cfg(target_os = "windows")]
use window_vibrancy::{apply_mica, apply_acrylic};
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
mod solo;
mod update;
mod list;
mod oath;
fn main() {
	tauri::Builder::default()
		.manage(Solo2List(Default::default()))
		.setup(|app| {
			let window = app.get_window("main").unwrap();
			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			#[cfg(target_os = "windows")]
			match apply_mica(&window) {
				Ok(_) => (),
				Err(_) => {
					apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! Acrylic is only supported on Windows");
				},
			}
			window.set_decorations(true).unwrap();

			tauri::async_runtime::spawn(list::init_key_listing(app.get_window("main").unwrap()));
			tauri::async_runtime::spawn(list::init_fetching_latest_version(app.get_window("main").unwrap()));
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			list::list_keys,
			list::latest_version,
			update::update_key,
			oath::list_oath,
			oath::get_oath_code,
			oath::register_oath,
			oath::delete_oath,
			solo::wink,
			solo::reboot,
			solo::maintenance,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
