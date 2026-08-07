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
                    error: Some("Public key doesn't exist without identity!".to_string()),
                    display: "No identity found!".to_string(),
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
                    error: Some("Username doesn't exist without identity!".to_string()),
                    display: "No identity!".to_string(),
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
                
                copy(text.clone()).await?; // only happens here, inside success
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
    Ok(
        match serde_json::to_string(&result){
            Ok(result) => result,
            Err(serde_err) => format!(
                        "{{\
                            \"success\": false,\
                            \"username\": null,\
                            \"error\": \"Serialization failed: {}\",\
                            \"display\": \"Error occured while copying data to clipboard!\",\
                            \"write\": []\
                        }}",
                        serde_err
            ),
        }
    )
}
