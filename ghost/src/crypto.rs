use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}};
use rand_core::{OsRng, TryRngCore};

use x25519_dalek::{StaticSecret, PublicKey};

use crate::parser;
use crate::storage;
use crate::types::CryptoFunctionResult;

fn encrypt_message(key_bytes: &[u8; 32], plaintext: &str) -> (Vec<u8>, Vec<u8>) {
    let key = Key::try_from(key_bytes.as_slice()).unwrap();
    let cipher = ChaCha20Poly1305::new(&key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.try_fill_bytes(&mut nonce_bytes).expect("OS RNG failed");
    let nonce = Nonce::try_from(nonce_bytes.as_slice()).unwrap();

    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).unwrap();
    (nonce.to_vec(), ciphertext)
}

fn decrypt_text(key_bytes: &[u8; 32], nonce: &Vec<u8>, ciphertext: &Vec<u8>) -> Result<String, String> {
    let key = Key::try_from(key_bytes.as_slice()).map_err(|_| "Invalid key".to_string())?;
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = Nonce::try_from(nonce.as_slice()).map_err(|_| "Invalid nonce".to_string())?;

    match cipher.decrypt(&nonce, ciphertext.as_ref()) {
        Ok(bytes) => String::from_utf8(bytes).map_err(|_| "Invalid utf8".to_string()),
        Err(_) => Err("Decryption failed".to_string()),
    }
}

/// Computes the shared secret both sides will independently arrive at.
fn compute_shared_secret(my_private_b64: &str, sender_public_b64: &str) -> Result<[u8; 32], String> {
    use base64::{engine::general_purpose, Engine };

    let priv_bytes = general_purpose::STANDARD.decode(my_private_b64).map_err(|_| "Bad private key".to_string())?;
    let pub_bytes = general_purpose::STANDARD.decode(sender_public_b64).map_err(|_| "Bad public key".to_string())?;

    let priv_arr: [u8; 32] = priv_bytes.try_into().map_err(|_| "Private key wrong length".to_string())?;
    let pub_arr: [u8; 32] = pub_bytes.try_into().map_err(|_| "Public key wrong length".to_string())?;

    let my_secret = StaticSecret::from(priv_arr);
    let their_public = PublicKey::from(pub_arr);

    let shared = my_secret.diffie_hellman(&their_public);
    Ok(*shared.as_bytes())
}

pub fn encrypt(
    my_public_b64: String,
    my_private_b64: String,
    recipient_public_b64: String,
    message: String ) -> CryptoFunctionResult {
        
    use base64::{engine::general_purpose::STANDARD, Engine};

    let key_bytes = match compute_shared_secret(&my_private_b64, &recipient_public_b64) {
        Ok(k) => k,
        Err(e) => {
            let display = &e.clone();
            return CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: format!("Error: {display}"),
                message_key: None,
            }
        },
    };

    let (nonce, ciphertext) = encrypt_message(&key_bytes, &message);
    let nonce_b64 = STANDARD.encode(&nonce);
    let ciphertext_b64 = STANDARD.encode(&ciphertext);
    
    let message_key = match parser::create_message_key(
        &my_public_b64,
        &nonce_b64,
        &ciphertext_b64
    ) {
        Ok(key) => key,
        Err(e) => {
            return CryptoFunctionResult{
                success: false,
                error: Some(e),
                nonce: None,
                display: format!("Couldn't create a message key!"),
                message_key: None,
            }
        },
    };
    
    CryptoFunctionResult {
        success: true,
        error: None,
        nonce: Some(nonce_b64),
        // display: STANDARD.encode(&ciphertext_b64),
        display: message_key.to_string(),
        message_key: Some(message_key.to_string()),
    }
}

pub fn decrypt(
    my_public_b64: String,
    my_private_b64: String,
    friend_index_json: String, 
    message_key: String,
) -> CryptoFunctionResult {
        
    use base64::{engine::general_purpose::STANDARD, Engine};

    let details = match parser::extract_details_from_message_key(&message_key) {
        Ok(detail) => detail,
        Err(e) => {
            return CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while trying to extract details from message key".to_string(),
                message_key: None,
            }
        },
    };

    // the persons' public key who had created the message key
    // myself if i am trying to decrypt my own message,
    // my friend if he is trying to decrypt my message
    
    let sender_public_b64 = details.sender_public_key; 
    
    let nonce_b64 = details.nonce_b64;
    let ciphertext_b64 = details.ciphertext_b64;

    let nonce = match STANDARD.decode(&nonce_b64) {
        Ok(n) => n,
        Err(_) => {
            return CryptoFunctionResult {
                success: false,
                error: Some("Invalid nonce".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            }
        }
    };

    let ciphertext = match STANDARD.decode(&ciphertext_b64) {
        Ok(c) => c,
        Err(_) => {
            return CryptoFunctionResult {
                success: false,
                error: Some("Invalid ciphertext".to_string()),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            }
        }
    };
    
    if my_public_b64 == sender_public_b64 {
        // get the index/Vec so we can loop over it
        match storage::index_from_json(&friend_index_json){
            Ok(friend_index) => {
                // Got the Vec!!
                // 
                // Now we are looping over the friend index to get
                // the friend for whom we had encrypted the text,
                // so we can use his public key and my private key
                // to decrypt the message
                for friend_public_b64 in friend_index.iter(){
                    // we compute the secret key/key byte so we can 
                    // decrypt it later to see if it's successfull
                    match compute_shared_secret(&my_private_b64, &friend_public_b64){
                        // friend_public_b64: the friends' public key we had in our storage
                        Ok(key_byte) => {
                            // we decrypt the text now and it's successfull
                            // only when the shared_secret we generated is the
                            // same which was used to generate the ciphertext
                            // shared secrets are same when we use:
                            // 1. My private and friends' public key
                            // 2. Friends private and my public key
                            match decrypt_text(&key_byte, &nonce, &ciphertext){
                                Ok(plaintext) => {
                                    return CryptoFunctionResult {
                                        success: true,
                                        nonce: None,
                                        error: None,
                                        display: plaintext,
                                        message_key: None,
                                    }
                                }
                                Err(_) => continue,
                            }
                        },
                        Err(_) => continue,
                    }
                }
            }
            Err(e) => {
                return CryptoFunctionResult {
                    success: false,
                    error: Some(e),
                    nonce: None,
                    display: "Couldn't parsing friend index!".to_string(),
                    message_key: None,
                }
            }
        }
    }

    
    let key_bytes = match compute_shared_secret(&my_private_b64, &sender_public_b64){ 
        // sender_public_b64: the public key we got from the message friend had created
        Ok(k) => k,
        Err(e) => {
            return CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while creating a shared secret key for you!".to_string(),
                message_key: None,
            }
        }
    };

    let plaintext = match decrypt_text(&key_bytes, &nonce, &ciphertext) {
        Ok(text) => text,
        Err(e) => {
            return CryptoFunctionResult {
                success: false,
                error: Some(e),
                nonce: None,
                display: "Error occured while decrypting!".to_string(),
                message_key: None,
            }
        }
    };

    CryptoFunctionResult {
        success: true,
        error: None,
        nonce: None,
        display: plaintext,
        message_key: None,
    }
}

