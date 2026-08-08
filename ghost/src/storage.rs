use serde_json;
use serde::Serialize;
use crate::identity::Identity;
use crate::friends::Friend;


#[derive(Serialize)]
pub struct LoadResult {
    pub has_identity: bool,
    pub identity_key_id: Option<String>,
    pub username: Option<String>,
    pub public_key: Option<String>,
    pub friends: Vec<Friend>,
    pub friend_index: Vec<String>,
}

pub fn friend_key(public_key: &str) -> String {
    format!("friend:{public_key}")
}

pub fn identity_to_json(identity: &Identity) -> String {
    serde_json::to_string(identity).unwrap()
}

pub fn identity_from_json(json: &str) -> Result<Identity, String> {
    match serde_json::from_str::<Identity>(json) {
        Ok(id) => Ok(id),
        Err(_) => Err("Your saved identity could not be read, so it is corrupted. Create a new identity in the GhostLayer popup.".to_string())
    }
}

pub fn friend_to_json(friend: &Friend) -> String {
    serde_json::to_string(friend).unwrap()
}

pub fn friend_from_json(json: &str) -> Result<Friend, String> {
    match serde_json::from_str::<Friend>(json) {
        Ok(fr) => Ok(fr),
        Err(_) => Err("A saved friend could not be read, so that entry is corrupted. Remove the friend and add them again.".to_string())
    }
}

pub fn index_to_json(index: &Vec<String>) -> String {
    serde_json::to_string(index).unwrap()
}

pub fn index_from_json(json: &str) -> Result<Vec<String>, String> {
    match serde_json::from_str::<Vec<String>>(json) {
        Ok(index) => Ok(index),
        Err(_) => Err("Your saved friend list could not be read, so it is corrupted.".to_string())
    }
}

pub fn add_to_index(index: &Vec<String>, public_key: &str) -> Result<Vec<String>, String> {
    let mut new_index = index.clone();
    // To avoid pushing duplicate key
    if !new_index.iter().any(|k| k == public_key){
        new_index.push(public_key.to_string());
        Ok(new_index)
    } else {
        Err("That friend is already in your list.".to_string())
    } 
    
}

pub fn get_identity(parsed: &serde_json::Value) -> Identity {
    let identity_str = parsed["identity"].as_str().unwrap();
    let identity: Identity = identity_from_json(identity_str).unwrap();
    identity
}

pub fn parse_storage(storage_json: String) -> LoadResult {
    let parsed: serde_json::Value = serde_json::from_str(&storage_json).unwrap();

    let has_identity = !parsed["identity"].is_null();
    
    let username = if has_identity {
        let identity: Identity = get_identity(&parsed);
        identity.username
    } else {
        None
    };

    let public_key = if has_identity {
        let identity: Identity = get_identity(&parsed);
        Some(identity.public_key)
    } else {
        None
    };
    
    let identity_key_id = if has_identity {
        let identity: Identity = get_identity(&parsed);
        Some(identity.key_id)
    } else {
        None
    };

    let mut friend_index: Vec<String> = Vec::new();
    
    let friends = if !parsed["friend_index"].is_null() {
        let friend_index_str = parsed["friend_index"].as_str().unwrap();
        friend_index = index_from_json(friend_index_str).unwrap();
        let mut friends = Vec::new();
        
        for public_key in &friend_index {
            let key = friend_key(&public_key);
            let friend_json = parsed[&key].as_str().unwrap();
            let friend = friend_from_json(friend_json).unwrap();
            
            friends.push(friend);
        }        
        friends
    } else {
        Vec::new()
    };

    LoadResult {
        has_identity: has_identity,
        identity_key_id: identity_key_id,
        username: username,
        public_key: public_key,
        friends: friends,
        friend_index: friend_index,
    }
}
