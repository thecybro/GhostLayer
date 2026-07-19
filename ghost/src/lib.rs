use wasm_bindgen::prelude::*;

mod identity;
use crate::identity::{
    Identity,
    create_identity as create_identity_internal
};

#[wasm_bindgen]
pub fn create_identity() -> String {     
    let identity: Identity = create_identity_internal();
    // TODO: Save identity in storage from here
    identity.key_id.to_string()
}

// just a prototype now
#[wasm_bindgen]
pub fn add_friend() -> String {
    "Friend Added!".to_string()
}