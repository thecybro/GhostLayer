// This file is responsible for creating:
// Public and private keys
// And give the public key as base64

use x25519_dalek::{
    StaticSecret,
    PublicKey,
};
use base64::{
    Engine,
    engine::general_purpose::STANDARD,
};
use serde::{Serialize, Deserialize};

use crate::types::{FunctionResult, StorageWrite};

use crate::storage;

// Unused for now, would take it to work later
#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub username: Option<String>, // optional
    pub public_key: String,
    pub private_key: String,
    pub key_id: String // first 5 letters of the base64 encoded for users to see
}

fn create(username: &Option<String>) -> Identity {
    let private_key = StaticSecret::random();
    let public_key = PublicKey::from(&private_key);

    let private_bytes = private_key.to_bytes();
    let public_bytes = public_key.to_bytes();
    
    let private_b64 = STANDARD.encode(private_bytes);
    let public_b64 = STANDARD.encode(public_bytes);

    let key_id = public_b64[0..5].to_string(); // first 5 letters
    
    Identity {
        username: username.clone(),
        public_key: public_b64,
        private_key: private_b64,
        key_id,
    }
}

pub fn create_identity(username: Option<String>) -> FunctionResult {   
    let identity: Identity = create(&username);
    let display = username
        .as_ref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&identity.key_id)
        .clone();
    
        // let public_key = &identity.public_key.to_string();
        // let private_key = &identity.private_key.to_string();
    FunctionResult {
        success: true,
        username: username,
        error: None,
        display: format!("Identity {display} has been created!").to_string(),
        write: vec! [
            StorageWrite {
                key: "identity".to_string(), value: storage::identity_to_json(&identity)
            }
        ]
    }
}
