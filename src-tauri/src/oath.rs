use std::collections::BTreeMap;

use tauri::{command, State, Window};
use solo2::apps::Oath;
use solo2::apps::oath::{Authenticate, Credential, Totp, Kind, Digest, Secret};
use solo2::{UuidSelectable, Uuid, Solo2, Select};

use crate::solo::Solo2List;

#[command]
pub async fn list_oath(uuid: Option<String>, state: State<'_, Solo2List>)-> Result<BTreeMap<String, String>, String> {
    let list = state.0.lock().await;
    let mut oath_list: BTreeMap<String, String> = BTreeMap::new();
    match uuid {
        Some(uuid) => {
            let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
            let mut device = Solo2::having(converted_uuid).unwrap();
            let mut oath = Oath::select(&mut device).unwrap();
            for credential in oath.list().unwrap() {
                oath_list.insert(credential, uuid.clone());
            }
        },
        None => {
            for (uuid, _) in list.iter() {
                let converted_uuid = Uuid::from_u128(u128::from_str_radix(uuid, 16).unwrap());
                let mut device = Solo2::having(converted_uuid).unwrap();
                let mut oath = Oath::select(&mut device).unwrap();
                for credential in oath.list().unwrap() {
                    oath_list.insert(credential, uuid.to_string());
                }
            }
        },
    }
    Ok(oath_list)
}

#[command]
pub async fn get_oath_code(uuid: String, credential: String, state: State<'_, Solo2List>) -> Result<String, String> {
    let _list = state.0.lock().await;
    let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
    let mut device = Solo2::having(converted_uuid).unwrap();
    let mut oath = Oath::select(&mut device).unwrap();
    match oath.authenticate(Authenticate::with_label(&credential)) {
        Ok(code) => {
            Ok(code)
        },
        Err(e) => Err(format!("Error while getting code: {:?}", e)),
    }
}
#[command]
pub async fn register_oath(uuid: String, label: String, issuer: Option<String>, secret: String, kind: String, algorithm: String, period: u32, digits: u8, state: State<'_, Solo2List>, window: Window) -> Result<String, String> {
    let _list = state.0.lock().await;
    let digest = match algorithm.as_str(){
        "sha1" => Digest::Sha1,
        "sha256" => Digest::Sha256,
        _ => return Err("Unsupported credential type".to_string())
    };
    let credential = Credential{
        label,
        issuer,
        secret: Secret::from_base32(&secret.to_uppercase(), digest).unwrap(),
        algorithm: digest,
        kind: match kind.as_str() {
            "totp" => Kind::Totp(Totp{period}),
            _ => return Err("Unsupported credential type".to_string()),
        },
        digits,
    };
    
    let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
    let mut device = Solo2::having(converted_uuid).unwrap();
    let mut oath = Oath::select(&mut device).unwrap();
    match oath.register(credential) {
        Ok(label) => {
            window.emit("oath_change", Some(true)).unwrap();
            Ok(label)
        }
        Err(e) => Err(format!("Error while registering credential: {:?}", e)),
    }
}
#[tauri::command]
pub async fn delete_oath(uuid: String, credential: String, state: State<'_, Solo2List>, window: tauri::Window) -> Result<(), String> {
    let _list = state.0.lock().await;
    let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
    let mut device = Solo2::having(converted_uuid).unwrap();
    let mut oath = Oath::select(&mut device).unwrap();
    match oath.delete(credential) {
        Ok(_) => {
            window.emit("oath_change", Some(true)).unwrap();
            Ok(())
        },
        Err(e) => Err(format!("Error while deleting credential: {:?}", e)),
    }
}