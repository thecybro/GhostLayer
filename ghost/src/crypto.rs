use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}};
use rand_core::{OsRng, TryRngCore};

use x25519_dalek::{StaticSecret, PublicKey};

use crate::parser;

// Written with `\` line continuations, which swallow the newline and the
// indent after it. A missing trailing space joins two words silently, so both
// are asserted in the tests at the bottom of this file.
const NO_SAVED_FRIEND_CAN_OPEN: &str =
    "You sent this message, but none of your saved friends can open it. \
     Either you removed the friend you sent it to, or this identity was moved \
     to another browser without its friend list. The message cannot be \
     recovered on this device.";

const NOT_ENCRYPTED_FOR_YOU: &str =
    "This message was not encrypted for you. It was sent to someone else, or \
     the sender made a new identity and the invite key you have saved for them \
     is out of date.";
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
    let key = Key::try_from(key_bytes.as_slice())
        .map_err(|_| "GhostLayer built an encryption key of the wrong size. This is a bug in GhostLayer, please report it.".to_string())?;
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = Nonce::try_from(nonce.as_slice())
        .map_err(|_| "This message's nonce is the wrong size, so the message was damaged before it reached you.".to_string())?;

    match cipher.decrypt(&nonce, ciphertext.as_ref()) {
        // Decrypted, but the bytes underneath are not text.
        Ok(bytes) => String::from_utf8(bytes)
            .map_err(|_| "This message decrypted, but what came out is not readable text. It was probably damaged in transit.".to_string()),

        // The authentication tag did not verify, which means the key is wrong.
        // Callers know which key they tried, so they say why it was wrong.
        Err(_) => Err("wrong key".to_string()),
    }
}

/// Computes the shared secret both sides will independently arrive at.
///
/// `whose_public_key` names the owner of the public key in plain words, so a
/// failure can say which of the two keys is the broken one.
fn compute_shared_secret(
    my_private_b64: &str,
    their_public_b64: &str,
    whose_public_key: &str,
) -> Result<[u8; 32], String> {
    use base64::{engine::general_purpose, Engine };

    let priv_bytes = general_purpose::STANDARD.decode(my_private_b64)
        .map_err(|_| "Your saved identity is damaged: its private key is not valid Base64. Create a new identity in the GhostLayer popup.".to_string())?;
    let pub_bytes = general_purpose::STANDARD.decode(their_public_b64)
        .map_err(|_| format!("The public key of {whose_public_key} is not valid Base64."))?;

    let priv_arr: [u8; 32] = priv_bytes.try_into()
        .map_err(|_| "Your saved identity is damaged: its private key is the wrong length. Create a new identity in the GhostLayer popup.".to_string())?;
    let pub_arr: [u8; 32] = pub_bytes.try_into()
        .map_err(|_| format!("The public key of {whose_public_key} is the wrong length. A GhostLayer key is 32 bytes."))?;

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

    let key_bytes = match compute_shared_secret(
        &my_private_b64,
        &recipient_public_b64,
        "the friend you picked",
    ) {
        Ok(k) => k,
        Err(e) => {
            return CryptoFunctionResult {
                success: false,
                error: Some("compute_shared_secret failed in encrypt".to_string()),
                nonce: None,
                display: format!("{e} Remove that friend and add them again with a fresh invite key."),
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
                error: Some("create_message_key failed in encrypt".to_string()),
                nonce: None,
                display: format!("GhostLayer encrypted your message but could not package it. {e}"),
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
                error: Some("extract_details_from_message_key failed".to_string()),
                nonce: None,
                display: e,
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
                error: Some("base64 decode of nonce_b64 failed".to_string()),
                nonce: None,
                display: "This message's nonce is not valid Base64, so the message was damaged before it reached you. Ask the sender to send it again.".to_string(),
                message_key: None,
            }
        }
    };

    let ciphertext = match STANDARD.decode(&ciphertext_b64) {
        Ok(c) => c,
        Err(_) => {
            return CryptoFunctionResult {
                success: false,
                error: Some("base64 decode of ciphertext_b64 failed".to_string()),
                nonce: None,
                display: "This message's encrypted content is not valid Base64, so the message was damaged before it reached you. Ask the sender to send it again.".to_string(),
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
                    match compute_shared_secret(&my_private_b64, &friend_public_b64, "a saved friend"){
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

                // Every saved friend was tried and none of them opened it. The
                // recipient's key is not carried in the message, so if they are
                // no longer in the friend list there is nothing left to try.
                return CryptoFunctionResult {
                    success: false,
                    error: Some("sender branch: no key in friend_index verified the tag".to_string()),
                    nonce: None,
                    display: NO_SAVED_FRIEND_CAN_OPEN.to_string(),
                    message_key: None,
                }
            }
            Err(e) => {
                return CryptoFunctionResult {
                    success: false,
                    error: Some("index_from_json failed on friend_index".to_string()),
                    nonce: None,
                    display: format!("{e} Try adding a friend again to rebuild it."),
                    message_key: None,
                }
            }
        }
    }

    // Not our own message, so the sender's key is the one to pair with.
    let key_bytes = match compute_shared_secret(
        &my_private_b64,
        &sender_public_b64,
        "the sender of this message",
    ) {
        Ok(k) => k,
        Err(e) => {
            return CryptoFunctionResult {
                success: false,
                error: Some("compute_shared_secret failed in decrypt".to_string()),
                nonce: None,
                display: e,
                message_key: None,
            }
        }
    };

    let plaintext = match decrypt_text(&key_bytes, &nonce, &ciphertext) {
        Ok(text) => text,
        Err(e) => {
            // "wrong key" is the tag check failing, which here can only mean the
            // message was addressed to someone else. Anything else is a real
            // description of what went wrong, so it is passed straight through.
            let reason = if e == "wrong key" {
                NOT_ENCRYPTED_FOR_YOU.to_string()
            } else {
                e
            };

            return CryptoFunctionResult {
                success: false,
                error: Some("decrypt_text failed on the receiver path".to_string()),
                nonce: None,
                display: reason,
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


#[cfg(test)]
mod tests {
    // The two long user facing messages in this file are written with `\`
    // line continuations, which swallow the newline and the indent after it.
    // A missing trailing space silently joins two words together, and that is
    // invisible in review, so it is asserted here instead.
    use super::{NOT_ENCRYPTED_FOR_YOU, NO_SAVED_FRIEND_CAN_OPEN};

    #[test]
    fn continued_messages_keep_their_spaces() {
        assert!(NO_SAVED_FRIEND_CAN_OPEN.contains("open it. Either you removed"));
        assert!(NO_SAVED_FRIEND_CAN_OPEN.contains("identity was moved to another browser"));
        assert!(NO_SAVED_FRIEND_CAN_OPEN.contains("message cannot be recovered"));
        assert!(!NO_SAVED_FRIEND_CAN_OPEN.contains("  "));

        assert!(NOT_ENCRYPTED_FOR_YOU.contains("someone else, or the sender"));
        assert!(NOT_ENCRYPTED_FOR_YOU.contains("you have saved for them"));
        assert!(!NOT_ENCRYPTED_FOR_YOU.contains("  "));
    }
}
