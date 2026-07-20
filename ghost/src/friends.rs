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

pub fn create_friend(nickname: Option<String>, public_key: String) -> Result<Friend, String> {
    let decoded_public_key = STANDARD.decode(&public_key);
    
    match decoded_public_key {
        Ok(bytes) => {
            let key_id = public_key[0..5].to_string(); // public_key is already base64 encoded

            // Isn't this how we check the length of a string?
            if bytes.len() != 32{
                return Err("Invalid encoding!".to_string())
            }
            
            Ok(Friend {
                nickname,
                public_key,
                key_id
            })
        }
        Err(_) => {
            Err("Invalid key!".to_string())
        }
    }
}