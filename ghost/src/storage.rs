use serde_json;
use serde::Serialize;
use crate::identity::Identity;
use crate::friends::Friend;
use web_sys::window;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

// Commented this and friend_display_from_json because FriendDisplay was identical to Friend
// #[derive(Serialize, Deserialize)]
// pub struct Friend {
//     pub nickname: Option<String>,
//     pub public_key: String,
//     pub key_id: String,
// }

#[derive(Serialize)]
pub struct LoadResult {
    pub has_identity: bool,
    pub identity_key_id: Option<String>,
    pub username: Option<String>,
    pub public_key: Option<String>,
    pub friends: Vec<Friend>,
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
        Err(_) => Err( "Couldn't convert to identity from json!".to_string())
    }
}

pub fn friend_to_json(friend: &Friend) -> String {
    serde_json::to_string(friend).unwrap()
}

pub fn friend_from_json(json: &str) -> Result<Friend, String> {
    match serde_json::from_str::<Friend>(json) {
        Ok(fr) => Ok(fr),
        Err(_) => Err("Couldn't convert to friend from json!".to_string())
    }
}

pub fn index_to_json(index: &Vec<String>) -> String {
    serde_json::to_string(index).unwrap()
}

pub fn index_from_json(json: &str) -> Result<Vec<String>, String> {
    match serde_json::from_str::<Vec<String>>(json) {
        Ok(index) => Ok(index),
        Err(_) => Err("Couldn't convert to index from json!".to_string())
    }
}

pub fn add_to_index(index: &Vec<String>, public_key: &str) -> Result<Vec<String>, String> {
    let mut new_index = index.clone();
    // To avoid pushing duplicate key
    if !new_index.iter().any(|k| k == public_key){
        new_index.push(public_key.to_string());
        Ok(new_index)
    } else {
        Err("Duplicate public key".to_string())
    } 
    
}

// Commented this and FriendDisplay because FriendDisplay was identical to Friend
// pub fn friend_from_json(json: &str) -> Result<Friend, String> {
//     match serde_json::from_str::<Friend>(json) {
//         Ok(friend) => Ok(friend),
//         Err(_) => Err("Couldn't convert driend display from json!".to_string())
//     }
// }

fn get_identity(parsed: &serde_json::Value) -> Identity {
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
    let friends = if !parsed["friend_index"].is_null() {
        let friend_index_str = parsed["friend_index"].as_str().unwrap();
        let friend_index = index_from_json(friend_index_str).unwrap();
        let mut friends = Vec::new();
        
        for public_key in friend_index {
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
    }
}

pub async fn copy_to_clipboard(storage_json: String, item: String) -> Result<String, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global window found"))?;
    let clipboard = window.navigator().clipboard();

    let parsed_data = parse_storage(storage_json);
    let item = item.to_lowercase();

    let text = match item.as_str() {
        "public_key" => {
            if !parsed_data.has_identity {
                return Err(JsValue::from_str("No identity found!"));
            }
            parsed_data.public_key.unwrap_or_default()
        }
        "username" => parsed_data.username.unwrap_or_default(),
        _ => return Err(JsValue::from_str("Invalid clipboard item")),
    };

    JsFuture::from(clipboard.write_text(&text)).await?;

    Ok(text)
}

