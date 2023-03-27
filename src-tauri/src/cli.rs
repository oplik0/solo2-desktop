use tauri::{api::cli::Matches,  AppHandle};

use crate::{list};


pub fn run_command(matches: Matches, handle: AppHandle) {
    if matches.args.get("version").is_some() && matches.args.get("version").unwrap().value.as_bool().unwrap_or(false) {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    match matches.subcommand {
        Some(subcommand) => {
            match subcommand.name.as_str() {
                "list" => {
                    let keys = list::get_key_list();
                    match keys {
                        Ok(keys) => {
                            for info in keys.into_values() {
                                println!("Key {uuid} (firmware: v{major}.{minor}.{patch}, secure: {secure})", uuid = info.uuid, major = info.version.major, minor = info.version.minor, patch = info.version.patch, secure = info.secure.unwrap_or(false));
                            }
                        }
                        Err(_) => {
                            println!("Error while listing keys");
                        }
                    }
                },
                "update" => {
                    // always install latest version if update command is called
                    tauri::updater::builder(handle).should_install(|_current, _latest| true);
                },
                "fido2" => {
                    println!("Fido2 command");
                },
                _ => {
                    println!("Unknown command");
                }
            }
        },
        None => {}
    }
}