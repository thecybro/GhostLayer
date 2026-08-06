use wasm_bindgen::prelude::*;
use serde_json;

mod identity;
mod friends;
mod storage;
mod parser;
mod clipboard;
mod crypto;
mod protocol;
mod types;


#[wasm_bindgen]
pub fn create_identity(username: Option<String>) -> String {
    serde_json::to_string(
        &identity::create_identity(username)
    ).unwrap()
}

#[wasm_bindgen]
pub fn add_friend(
    nickname: Option<String>, 
    invite_key: String, 
    storage_json: String ) -> String {

    serde_json::to_string(
        &friends::add_friend(
            nickname,
            invite_key,
            storage_json,
        )
    ).unwrap()
        
}


#[wasm_bindgen]
pub fn load_display_data(storage_json: String) -> String {
    serde_json::to_string(&storage::parse_storage(storage_json)).unwrap()
}


#[wasm_bindgen]
pub async fn copy_to_clipboard(
    storage_json: String,
    item: String ) -> Result<String, JsValue> {

        clipboard::copy_to_clipboard(
            storage_json,
            item,
        ).await
}

#[wasm_bindgen]
pub fn encrypt(
    my_public_b64: String,
    my_private_b64: String,
    their_public_b64: String,
    message: String ) -> String {
    
    serde_json::to_string(
        &crypto::encrypt(
            my_public_b64,
            my_private_b64,
            their_public_b64,
            message
        )
    ).unwrap()
}

#[wasm_bindgen]
pub fn decrypt(
    my_private_b64: String,
    message_key: String ) -> String {
    serde_json::to_string(
        &crypto::decrypt(
            my_private_b64,
            message_key,
        )
    ).unwrap()
}
