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
    pub nonce: Option<String>, // used to return nonce for encryption/decryption
    pub display: String, // info to show to user, eg: key_id
    pub write: Vec<StorageWrite>,
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
        nonce: None,
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
pub fn add_friend(nickname: Option<String>, invite_key: String, current_index_json: String) -> String {
    match parser::extract_details_from_invite_key(invite_key) {
        Ok(details) => {
            let public_key = details.public_key;
            let nickname_from_key = details.nickname;
            let key_id = details.key_id;
        
            // use nickname if given, if not, use the nickname that came from key
            let nickname = Some(nickname 
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| nickname_from_key.clone()).to_string());
            
            match friends::create_friend(nickname.clone(), public_key, key_id) {
                Ok(friend) => {
                    // when there aren't any friends yet
                    let index = storage::index_from_json(&current_index_json).unwrap_or_default(); 
                    let new_index = storage::add_to_index(&index, &friend.public_key);
                    let display = nickname.unwrap_or_else(|| friend.key_id.to_string());
                    match new_index {
                        Ok(n_index) => {
                            let result = FunctionResult {
                                success: true,
                                username: None,
                                error: None,
                                nonce: None,
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
                                nonce: None,
                                display: "Couldn't add friend details to storage!".to_string(),
                                write: vec![]
                                };
                            serde_json::to_string(&result).unwrap()
                        }
                    }
                },
                Err(e) => {
                    let result = FunctionResult {
                        success: false,
                        username: None,
                        error: Some(e),
                        nonce: None,
                        display: "Couldn't create friend".to_string(),
                        write: vec![]
                        };
                    serde_json::to_string(&result).unwrap()
                },
            }
        },
        Err(e) => {
            let result = FunctionResult {
                success: false,
                username: None,
                error: Some(e),
                nonce: None,
                display: "Invalid invite key!".to_string(),
                write: vec![]
                };
            serde_json::to_string(&result).unwrap()
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
                    nonce: None,
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
                    nonce: None,
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
                    nonce: None,
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
                        nonce: None,
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
                    nonce: None,
                    display: "No identity found!".to_string(),
                    write: vec![] 
                }
            } else {
                let public_key = parsed_data.public_key.unwrap_or_default();
                let username = Some(parsed_data.username.unwrap_or_default());
                let text = parser::create_invite_key(public_key, username).to_string();
                let display = &text.clone()[0..6];
                clipboard::copy_to_clipboard(text.clone()).await?; // only happens here, inside success
                FunctionResult { 
                    success: true,
                    username: None,
                    error: None,
                    nonce: None,
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
                nonce: None,
                display: "Invalid item".to_string(),
                write: vec![]
            }
        }
    };
    Ok(serde_json::to_string(&result).unwrap())
}

// This function worked fine when I tested it across Js and wasm,
// Now I want to split the logic into 2 files encrypt and decrypt
// This test_.. is a temporary function that i will delete soon
// #[wasm_bindgen]
// pub fn test_encrypt_roundtrip(my_private_b64: String, their_public_b64: String, message: String) -> String {
//     use base64::{engine::general_purpose, Engine };

//     let key_bytes = match crypto::compute_shared_secret(&my_private_b64, &their_public_b64) {
//         Ok(k) => k,
//         Err(e) => return format!("KEY DERIVATION FAILED: {}", e),
//     };

//     let (nonce, ciphertext) = crypto::encrypt_message(&key_bytes, &message);

//     let nonce_b64 = general_purpose::STANDARD.encode(&nonce);
//     let ciphertext_b64 = general_purpose::STANDARD.encode(&ciphertext);

//     let decrypted = match crypto::decrypt_text(&key_bytes, nonce, ciphertext) {
//         Ok(text) => text,
//         Err(e) => return format!(
//             "Original: {}\nNonce: {}\nCiphertext: {}\nDECRYPT FAILED: {}",
//             message, nonce_b64, ciphertext_b64, e
//         ),
//     };

//     format!(
//         "Original: {}    \nNonce: {}\n      Ciphertext/encrypted: {}\n     Decrypted: {}\n     Match: {}",
//         message, nonce_b64, ciphertext_b64, decrypted, message == decrypted
//     )
// }

#[wasm_bindgen]
pub fn encrypt(
    my_private_b64: String,
    their_public_b64: String,
    message: String ) -> String {
        
    use base64::{engine::general_purpose::STANDARD, Engine};

    let key_bytes = match crypto::compute_shared_secret(&my_private_b64, &their_public_b64) {
        Ok(k) => k,
        Err(e) => {
            let display = &e.clone();
            return serde_json::to_string(&FunctionResult {
                success: false,
                username: None,
                error: Some(e),
                nonce: None,
                display: format!("Error: {display}").to_string(),
                write: vec![],
            })
            .unwrap();
        }
    };

    let (nonce, ciphertext) = crypto::encrypt_message(&key_bytes, &message);

    let result = FunctionResult {
        success: true,
        username: None,
        error: None,
        nonce: Some(STANDARD.encode(&nonce)),
        display: STANDARD.encode(&ciphertext),
        write: vec![],
    };

    serde_json::to_string(&result).unwrap()
}

#[wasm_bindgen]
pub fn decrypt(
    my_private_b64: String,
    their_public_b64: String,
    nonce_b64: String,
    ciphertext_b64: String ) -> String {
        
    use base64::{engine::general_purpose::STANDARD, Engine};

    let key_bytes = match crypto::compute_shared_secret(&my_private_b64, &their_public_b64) {
        Ok(k) => k,
        Err(e) => {
            return serde_json::to_string(&FunctionResult {
                success: false,
                username: None,
                error: Some(e),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                write: vec![],
            })
            .unwrap();
        }
    };

    let nonce = match STANDARD.decode(&nonce_b64) {
        Ok(n) => n,
        Err(_) => {
            return serde_json::to_string(&FunctionResult {
                success: false,
                username: None,
                error: Some("Invalid nonce".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                write: vec![],
            })
            .unwrap();
        }
    };

    let ciphertext = match STANDARD.decode(&ciphertext_b64) {
        Ok(c) => c,
        Err(_) => {
            return serde_json::to_string(&FunctionResult {
                success: false,
                username: None,
                error: Some("Invalid ciphertext".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                write: vec![],
            })
            .unwrap();
        }
    };

    let plaintext = match crypto::decrypt_text(&key_bytes, nonce, ciphertext) {
        Ok(text) => text,
        Err(e) => {
            return serde_json::to_string(&FunctionResult {
                success: false,
                username: None,
                error: Some(e),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                write: vec![],
            })
            .unwrap();
        }
    };

    let result = FunctionResult {
        success: true,
        username: None,
        error: None,
        nonce: None,
        display: plaintext,
        write: vec![],
    };

    serde_json::to_string(&result).unwrap()
}
