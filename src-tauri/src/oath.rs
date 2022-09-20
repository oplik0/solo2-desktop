use std::collections::BTreeMap;

use tauri::{command, State, Window};
use solo2::apps::Oath;
use solo2::apps::oath::Authenticate;
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
                let converted_uuid = Uuid::from_u128(u128::from_str_radix(&uuid, 16).unwrap());
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
pub async fn get_oath_code(uuid: String, credential: String) -> Result<String, String> {
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