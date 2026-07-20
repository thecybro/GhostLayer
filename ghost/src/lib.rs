use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_json;

mod identity;
mod friends;
mod storage;
use crate::identity::{
    Identity,
    create_identity as create_identity_internal
};

#[derive(Serialize)]
pub struct StorageWrite {
    pub key: String,
    pub value: String
}

#[derive(Serialize)]
pub struct FunctionResult {
    pub success: bool,
    pub error: Option<String>,
    pub display: String, // info to show to user, eg: key_id
    pub write: Vec<StorageWrite>,
}

// These are the json types we need:
// 
// identity        → { "name": null, "public_key": "...", "private_key": "...", "key_id": "..." }
// friend_index    → ["<pubkey1>", "<pubkey2>", ...]
// friend:pubkeyA  → { "nickname": "...", "public_key": "...", "key_id": "..." }
// friend:pubkeyN

// Error handling is not yet as well done
#[wasm_bindgen]
pub fn create_identity() -> String {     
    let identity: Identity = create_identity_internal();
    let result = FunctionResult {
        success: true,
        error: None,
        display: identity.key_id.to_string(),
        write: vec! [
            StorageWrite {
                key: "identity".to_string(), value: storage::identity_to_json(&identity)
            }
        ]
    };
    serde_json::to_string(&result).unwrap()
}

// Error handling is not yet as well done
#[wasm_bindgen]
pub fn add_friend(nickname: Option<String>, public_key: String, current_index_json: String) -> String {
    match friends::create_friend(nickname, public_key) {
        Ok(friend) => {
            // when there aren't any friends yet
            let index = storage::index_from_json(&current_index_json).unwrap_or_default(); 
            let new_index = storage::add_to_index(&index, &friend.public_key);
            
            let result = FunctionResult {
                success: true,
                error: None,
                display: friend.key_id.to_string(),
                write: vec![
                    StorageWrite { // json with details of one friend
                        key: storage::friend_key(&friend.public_key),
                        value: storage::friend_to_json(&friend)
                    },
                    StorageWrite { // json with just the public keys of friends
                        key: "friend_index".to_string(),
                        value: storage::index_to_json(&new_index)
                    },
                ]
            };
            return serde_json::to_string(&result).unwrap()
        },
        Err(e) => {
            let result = FunctionResult {
                success: false,
                error: Some(e),
                display: "Error!".to_string(),
                write: vec![]
                };
            serde_json::to_string(&result).unwrap()
        },
    }
}

