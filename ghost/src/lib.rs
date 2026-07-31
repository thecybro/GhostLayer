use wasm_bindgen::prelude::*;
use serde::Serialize;
use serde_json;

mod identity;
mod friends;
mod storage;
mod parser;
mod clipboard;
mod crypto;

#[derive(Serialize)]
pub struct StorageWrite {
    pub key: String,
    pub value: String
}

#[derive(Serialize)]
pub struct FunctionResult {
    pub success: bool,
    pub username: Option<String>,
    pub error: Option<String>,
    pub display: String, // info to show to user, eg: key_id
    pub write: Vec<StorageWrite>,
}

#[derive(Serialize)]
pub struct CryptoFunctionResult {
    pub success: bool,
    pub error: Option<String>,
    pub nonce: Option<String>,
    pub display: String,
    pub message_key: Option<String>,
}

// #[derive(Serialize)]
// pub struct CryptoResult {
//     pub success: bool,
//     pub error: Option<String>,
//     pub message: String,
//     pub nonce: Option<String>,
//     // pub ciphertext: String,
//     pub display: String, // display is ciphertext after encryption, display is decrypted text after decryption
//     pub write: Vec<StorageWrite>,
// }


// These are the json types we need:
// 
// identity        → { "name": null, "public_key": "...", "private_key": "...", "key_id": "..." }
// friend_index    → ["<pubkey1>", "<pubkey2>", ...]
// friend:pubkeyA  → { "nickname": "...", "public_key": "...", "key_id": "..." }
// friend:pubkeyN

// Error handling is not yet as well done
#[wasm_bindgen]
pub fn create_identity(username: Option<String>) -> String {   
    let identity: identity::Identity = identity::create_identity(&username);
    let display = username
        .as_ref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&identity.key_id)
        .clone();
    
        // let public_key = &identity.public_key.to_string();
        // let private_key = &identity.private_key.to_string();
    let result = FunctionResult {
        success: true,
        username: username,
        error: None,
        display: format!("Identity {display} has been created!").to_string(),
        // display: format!("Public_key: {public_key}, Private_key: {private_key}"),
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
pub fn add_friend(
    nickname: Option<String>, 
    invite_key: String, 
    storage_json: String ) -> String {
    // We need our own identity to add/create friends, so a check here,
    // the implementation might not be good and might be redundant
    // but will improve later
    let parsed_storage = storage::parse_storage(storage_json);
    if !parsed_storage.has_identity{
        let result = FunctionResult{
            success: false,
            username: None,
            error: Some("Identity not found!".to_string()),
            display: "Friends can't be added without having an identity yourself!".to_string(),
            write: vec![]
        };
        return serde_json::to_string(&result).unwrap();
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
                                
            match friends::create_friend(nickname.clone(), public_key, key_id) {
                Ok(friend) => {
                    // when there aren't any friends yet
                    // let index = storage::index_from_json(&current_index).unwrap_or_default(); 
                    let new_index = storage::add_to_index(&current_index, &friend.public_key);
                    let display = nickname.unwrap_or_else(|| friend.key_id.to_string());
                    match new_index {
                        Ok(n_index) => {
                            let result = FunctionResult {
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
                            };
                            return serde_json::to_string(&result).unwrap()
                        },
                        Err(e) => {
                            let result = FunctionResult {
                                success: false,
                                username: None,
                                error: Some(e),
                                display: "Couldn't add friend details to storage!".to_string(),
                                write: vec![]
                                };
                            return serde_json::to_string(&result).unwrap()
                        }
                    }
                },
                Err(e) => {
                    let result = FunctionResult {
                        success: false,
                        username: None,
                        error: Some(e),
                        display: "Couldn't create friend!".to_string(),
                        write: vec![]
                        };
                    return serde_json::to_string(&result).unwrap()
                },
            }
        },
        Err(e) => {
            let result = FunctionResult {
                success: false,
                username: None,
                error: Some(e),
                display: "Invalid invite key!".to_string(),
                write: vec![]
                };
            return serde_json::to_string(&result).unwrap()
        }
    }
}


#[wasm_bindgen]
pub fn load_display_data(storage_json: String) -> String {
    let result = storage::parse_storage(storage_json);
    serde_json::to_string(&result).unwrap()
}

// #[wasm_bindgen]
// pub async fn copy_to_clipboard(storage_json: String, item: String) -> Result<String, JsValue> {
//     storage::copy_to_clipboard(storage_json, item).await
// }

#[wasm_bindgen]
pub async fn copy_to_clipboard(storage_json: String, item: String) -> Result<String, JsValue> {
    let parsed_data = storage::parse_storage(storage_json);
    let item = item.to_lowercase();

    let result: FunctionResult = match item.as_str() {
        "public_key" => {
            if !parsed_data.has_identity {
                FunctionResult { 
                    success: false,
                    username: None,
                    error: Some("Public key doesn't exist without identity!".to_string()),
                    display: "No identity found!".to_string(),
                    write: vec![] 
                }
            } else {
                let text = parsed_data.identity_key_id.unwrap_or_default();
                let display = text.clone().to_string();
                clipboard::copy_to_clipboard(text.clone()).await?; // only happens here, inside success
                FunctionResult { 
                    success: true,
                    username: None,
                    error: None,
                    display: format!("Public key {display}.. was copied to clipboard",).to_string(),
                    write: vec![] 
                }
            }
        },
        "username" => {
            // same shape: if !has_identity -> error FunctionResult
            // else -> build text, await clipboard write, success FunctionResult
            if !parsed_data.has_identity {
                FunctionResult {
                    success: false,
                    username: None,
                    error: Some("Username doesn't exist without identity!".to_string()),
                    display: "No identity!".to_string(),
                    write: vec![]
                }
            } else {
                    let text = parsed_data.username.unwrap_or_default();
                    let display = text.clone().to_string();
                    clipboard::copy_to_clipboard(text.clone()).await?;
                    FunctionResult {
                        success: true,
                        username: Some(text.clone().to_string()),
                        error: None,
                        display: format!("Username {display} was copied to clipboard").to_string(),
                        write: vec![]
                    }
                }
            },
        "invite_key" => {
            if !parsed_data.has_identity {
                FunctionResult { 
                    success: false,
                    username: None,
                    error: Some("Invite key doesn't exist without identity!".to_string()),
                    display: "No identity found!".to_string(),
                    write: vec![] 
                }
            } else {
                let public_key = parsed_data.public_key.unwrap_or_default();
                
                // Converts Option<String> into Option<&str>.
                let username = parsed_data.username.as_deref();
                let text = parser::create_invite_key(&public_key, username);
                let display = &text.clone()[0..6];
                
                clipboard::copy_to_clipboard(text.clone()).await?; // only happens here, inside success
                FunctionResult { 
                    success: true,
                    username: None,
                    error: None,
                    display: format!("Invite key {display} has been copied to clipboard!").to_string(),
                    write: vec![] 
                }
            }
        },
        _ => {
            FunctionResult {
                success: false,
                username: None,
                error: Some("Invalid item".to_string()),
                display: "Invalid item".to_string(),
                write: vec![]
            }
        }
    };
    Ok(serde_json::to_string(&result).unwrap())
}


#[wasm_bindgen]
pub fn encrypt(
    my_public_b64: String,
    my_private_b64: String,
    their_public_b64: String,
    message: String ) -> String {
        
    use base64::{engine::general_purpose::STANDARD, Engine};

    let key_bytes = match crypto::compute_shared_secret(&my_private_b64, &their_public_b64) {
        Ok(k) => k,
        Err(e) => {
            let display = &e.clone();
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: format!("Error: {display}").to_string(),
                message_key: None,
            })
            .unwrap();
        }
    };

    let (nonce, ciphertext) = crypto::encrypt_message(&key_bytes, &message);
    let nonce_b64 = STANDARD.encode(&nonce);
    let ciphertext_b64 = STANDARD.encode(&ciphertext);
    
    let message_key = match parser::create_message_key(
        &my_public_b64,
        &nonce_b64,
        &ciphertext_b64
    ) {
        Ok(key) => key,
        Err(e) => e,
    };
    
    let result = CryptoFunctionResult {
        success: true,
        error: None,
        nonce: Some(nonce_b64),
        // display: STANDARD.encode(&ciphertext_b64),
        display: message_key.to_string(),
        message_key: Some(message_key.to_string()),
    };

    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn decrypt(
    my_private_b64: String,
    message_key: String ) -> String {
        
    use base64::{engine::general_purpose::STANDARD, Engine};
        
    let details = match parser::extract_details_from_message_key(&message_key) {
        Ok(detail) => detail,
        Err(e) => {
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while trying to extract details from message key".to_string(),
                message_key: None,
            }).unwrap()
        },
    };

    let their_public_b64 = details.sender_public_key;
    let nonce_b64 = details.nonce_b64;
    let ciphertext_b64 = details.ciphertext_b64;
    
    let key_bytes = match crypto::compute_shared_secret(&my_private_b64, &their_public_b64) {
        Ok(k) => k,
        Err(e) => {
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            })
            .unwrap();
        }
    };

    let nonce = match STANDARD.decode(&nonce_b64) {
        Ok(n) => n,
        Err(_) => {
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some("Invalid nonce".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            })
            .unwrap();
        }
    };

    let ciphertext = match STANDARD.decode(&ciphertext_b64) {
        Ok(c) => c,
        Err(_) => {
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some("Invalid ciphertext".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            })
            .unwrap();
        }
    };

    let plaintext = match crypto::decrypt_text(&key_bytes, nonce, ciphertext) {
        Ok(text) => text,
        Err(e) => {
            return serde_json::to_string(&CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            })
            .unwrap();
        }
    };

    let result = CryptoFunctionResult {
        success: true,
        error: None,
        nonce: None,
        display: plaintext,
        message_key: None,
    };

    serde_json::to_string(&result).unwrap()
}
