use serde::{Serialize, Deserialize};
use base64::{
    Engine,
    engine::general_purpose::STANDARD,
};

#[derive(Serialize, Deserialize)]
pub struct Friend {
    pub nickname: Option<String>,
    pub public_key: String,
    pub key_id: String
}

pub fn create_friend(nickname: &Option<String>, public_key: String, key_id: String) -> Result<Friend, String> {
    match STANDARD.decode(&public_key){
        Ok(bytes) => {
            if public_key.trim().is_empty() {
                return Err("Public key is required to add a friend!".to_string())
            };

            // Isn't this how we check the length of a string?
            // Yes, when decoded, the length of the chars would be 32 
            // But if we checked before decoding, the length would be 44
            // Both valid
            if bytes.len() != 32{
                return Err("Invalid encoding!".to_string())
            }
            
            Ok(Friend {
                nickname: nickname.clone() ,
                public_key,
                key_id
            })
        }
        Err(_) => {
            Err("Error occured while decoding public key!".to_string())
        }
    }
}

