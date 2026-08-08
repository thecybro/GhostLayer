use web_sys::window;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::types::FunctionResult;
use crate::storage;
use crate::parser;

async fn copy(text: String) -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global window found"))?;
    let clipboard = window.navigator().clipboard();

    JsFuture::from(clipboard.write_text(&text)).await?;

    Ok(())
}

pub async fn copy_to_clipboard(storage_json: String, item: String) -> Result<String, JsValue> {
    let parsed_data = storage::parse_storage(storage_json);
    let item = item.to_lowercase();

    let result: FunctionResult = match item.as_str() {
        "public_key" => {
            if !parsed_data.has_identity {
                FunctionResult { 
                    success: false,
                    username: None,
                    error: Some("copy_to_clipboard public_key with has_identity = false".to_string()),
                    display: "You have no public key yet. Click Create Identity first.".to_string(),
                    write: vec![] 
                }
            } else {
                let text = parsed_data.identity_key_id.unwrap_or_default();
                let display = text.clone().to_string();
                copy(text.clone()).await?; // only happens here, inside success
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
                    error: Some("copy_to_clipboard username with has_identity = false".to_string()),
                    display: "You have no username yet. Click Create Identity first.".to_string(),
                    write: vec![]
                }
            } else {
                    let text = parsed_data.username.unwrap_or_default();
                    let display = text.clone().to_string();
                    copy(text.clone()).await?;
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
                    error: Some("copy_to_clipboard invite_key with has_identity = false".to_string()),
                    display: "You have no invite key yet. Click Create Identity first.".to_string(),
                    write: vec![] 
                }
            } else {
                let public_key = parsed_data.public_key.unwrap_or_default();
                
                // Converts Option<String> into Option<&str>.
                let username = parsed_data.username.as_deref();
                match parser::create_invite_key(&public_key, username) {
                    Err(e) => FunctionResult {
                        success: false,
                        username: None,
                        display: e,
                        error: Some("create_invite_key failed".to_string()),
                        write: vec![],
                    },
                    Ok(text) => {
                        // take() rather than a slice, so a short key cannot panic
                        let display = text.chars().take(6).collect::<String>();

                        copy(text.clone()).await?; // only happens here, inside success
                        FunctionResult {
                            success: true,
                            username: None,
                            error: None,
                            display: format!("Invite key {display} has been copied to clipboard!").to_string(),
                            write: vec![]
                        }
                    }
                }
            }
        },
        _ => {
            FunctionResult {
                success: false,
                username: None,
                error: Some("copy_to_clipboard called with an unknown item".to_string()),
                display: "GhostLayer does not know how to copy that. This is a bug in GhostLayer, please report it.".to_string(),
                write: vec![]
            }
        }
    };
    Ok(
        match serde_json::to_string(&result){
            Ok(result) => result,
            Err(serde_err) => format!(
                        "{{\
                            \"success\": false,\
                            \"username\": null,\
                            \"error\": \"serde_json to_string failed: {}\",\
                            \"display\": \"GhostLayer could not package its own result. This is a bug in GhostLayer, please report it.\",\
                            \"write\": []\
                        }}",
                        serde_err
            ),
        }
    )
}
