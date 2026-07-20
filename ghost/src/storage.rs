use serde_json;
use crate::identity::Identity;
use crate::friends::Friend;

pub fn friend_key(public_key: &str) -> String {
    format!("friend:{public_key}")
}

pub fn identity_to_json(identity: &Identity) -> String {
    serde_json::to_string(identity).unwrap()
}

// pub fn identity_from_json(json: &str) -> Result<Identity, String> {
//     match serde_json::from_str::<Identity>(json) {
//         Ok(id) => Ok(id),
//         Err(_) => Err( "Couldn't convert to identity from json!".to_string())
//     }
// }

pub fn friend_to_json(friend: &Friend) -> String {
    serde_json::to_string(friend).unwrap()
}

// pub fn friend_from_json(json: &str) -> Result<Friend, String> {
//     match serde_json::from_str::<Friend>(json) {
//         Ok(fr) => Ok(fr),
//         Err(_) => Err("Couldn't convert to friend from json!".to_string())
//     }
// }

pub fn index_to_json(index: &Vec<String>) -> String {
    serde_json::to_string(index).unwrap()
}

pub fn index_from_json(json: &str) -> Result<Vec<String>, String> {
    match serde_json::from_str::<Vec<String>>(json) {
        Ok(index) => Ok(index),
        Err(_) => Err("Couldn't convert to index from json!".to_string())
    }
}

pub fn add_to_index(index: &Vec<String>, public_key: &str) -> Vec<String> {
    let mut new_index = index.clone();
    // To avoid pushing duplicate key
    if !new_index.iter().any(|k| k == public_key){
        new_index.push(public_key.to_string())
    };
    new_index
}