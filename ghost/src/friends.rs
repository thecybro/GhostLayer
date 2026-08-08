use serde::{Serialize, Deserialize};
use base64::{
    Engine,
    engine::general_purpose::STANDARD,
};

use crate::types::{FunctionResult, StorageWrite};
use crate::parser;
use crate::storage;

#[derive(Serialize, Deserialize)]
pub struct Friend {
    pub nickname: Option<String>,
    pub public_key: String,
    pub key_id: String
}

fn create_friend(nickname: Option<String>, public_key: String, key_id: String) -> Result<Friend, String> {
    match STANDARD.decode(&public_key){
        Ok(bytes) => {
            if public_key.trim().is_empty() {
                return Err("That invite key has no public key in it. Ask your friend to copy their invite again.".to_string())
            };

            // Isn't this how we check the length of a string?
            // Yes, when decoded, the length of the chars would be 32 
            // But if we checked before decoding, the length would be 44
            // Both valid
            if bytes.len() != 32{
                return Err("That invite key's public key is the wrong length, so the invite is damaged. Ask your friend to copy it again.".to_string())
            }
            
            Ok(Friend {
                nickname: nickname,
                public_key,
                key_id
            })
        }
        Err(_) => {
            Err("That invite key's public key is not valid Base64, so the invite is damaged. Ask your friend to copy it again.".to_string())
        }
    }
}

pub fn add_friend(
    nickname: Option<String>, 
    invite_key: String, 
    storage_json: String ) -> FunctionResult {
    // We need our own identity to add/create friends, so a check here,
    // the implementation might not be good and might be redundant
    // but will improve later
    let parsed_storage = storage::parse_storage(storage_json);
    if !parsed_storage.has_identity{
        return FunctionResult{
            success: false,
            username: None,
            error: Some("add_friend called with has_identity = false".to_string()),
            display: "Create your own identity first. GhostLayer needs your key pair to work out a shared secret with a friend.".to_string(),
            write: vec![]
        }
    }
        
    match parser::extract_details_from_invite_key(&invite_key) {
        Ok(details) => {
            let public_key = details.public_key;
            let key_id = details.key_id;
        
            // use nickname if given, if not, use the nickname that came from key
            let nickname = nickname 
                .filter(|n| !n.trim().is_empty())
                .or_else(|| {
                    if details.nickname.as_ref()?.trim().is_empty(){
                        None
                    } else {
                        details.nickname // already Some()
                    }
                });

            let current_index = parsed_storage.friend_index;
                // .friend_index
                // .unwrap_or_default();
                                
            match create_friend(nickname.clone(), public_key, key_id) {
                Ok(friend) => {
                    // when there aren't any friends yet
                    // let index = storage::index_from_json(&current_index).unwrap_or_default(); 
                    let new_index = storage::add_to_index(&current_index, &friend.public_key);
                    let display = nickname.unwrap_or_else(|| friend.key_id.to_string());
                    match new_index {
                        Ok(n_index) => {
                            return FunctionResult {
                                success: true,
                                username: None,
                                error: None,
                                display: format!("Friend {display} has been created!").to_string(),
                                write: vec![
                                    StorageWrite { // json with details of one friend
                                        key: storage::friend_key(&friend.public_key),
                                        value: storage::friend_to_json(&friend)
                                    },
                                    StorageWrite { // json with just the public keys of friends
                                        key: "friend_index".to_string(),
                                        value: storage::index_to_json(&n_index)
                                    },
                                ]
                            }
                        },
                        Err(e) => {
                            return FunctionResult {
                                success: false,
                                username: None,
                                display: e,
                                error: Some("add_to_index rejected the public key".to_string()),
                                write: vec![]
                                }
                        }
                    }
                },
                Err(e) => {
                    return FunctionResult {
                        success: false,
                        username: None,
                        display: e,
                        error: Some("create_friend rejected the invite key".to_string()),
                        write: vec![]
                        }
                },
            }
        },
        Err(e) => {
            return FunctionResult {
                success: false,
                username: None,
                display: e,
                error: Some("extract_details_from_invite_key failed".to_string()),
                write: vec![]
                }
        }
    }
}
