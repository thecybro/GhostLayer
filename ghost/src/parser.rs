use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};
use serde::Serialize;

// Fixed formats used throughout GhostLayer.
const INVITE_PREFIX: &str = "ghl";
const MESSAGE_PREFIX: &str = "ghl_message";

// X25519 public keys are 32 bytes.
// Standard Base64 represents 32 bytes using 44 characters.
const PUBLIC_KEY_B64_LENGTH: usize = 44;

// ChaCha20Poly1305 uses a 12-byte nonce.
// Standard Base64 represents 12 bytes using 16 characters.
const NONCE_B64_LENGTH: usize = 16;


// Invite keys and encrypted messages contain different information,
// so they should have different parsed result types.

#[derive(Serialize)]
pub struct InviteDetails {
    pub nickname: Option<String>,
    pub public_key: String,
    pub key_id: String,
}

#[derive(Serialize)]
pub struct MessageDetails {
    // This must be the sender's public key.
    // The receiver combines it with their own private key.
    pub sender_public_key: String,

    pub nonce_b64: String,
    pub ciphertext_b64: String,
}


// Format:
// ghl<44-character-public-key><optional-nickname>
//
// Example:
// ghlV/uAF4nN94xt9saSDytlmyjMuvcDJgIdE2Wl+80dc2M=Cybro
pub fn create_invite_key(public_key: &str, nickname: Option<&str>) -> String {
    let nickname = nickname
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("");

    format!("{INVITE_PREFIX}{public_key}{nickname}")
}


// Parse:
// ghl<44-character-public-key><optional-nickname>
pub fn extract_details_from_invite_key(invite_key: &str) -> Result<InviteDetails, String> {
    let remaining = invite_key
        .strip_prefix(INVITE_PREFIX)
        .ok_or_else(|| "Invalid invite key prefix.".to_string())?;

    if remaining.len() < PUBLIC_KEY_B64_LENGTH {
        return Err("Invite key is too short.".to_string());
    }

    // First 44 characters are always the Base64 public key.
    // Anything afterward is the optional nickname.
    let (public_key, nickname) = remaining.split_at(PUBLIC_KEY_B64_LENGTH);

    validate_public_key(public_key)?;

    let nickname = if nickname.trim().is_empty() {
        None
    } else {
        Some(nickname.to_string())
    };

    Ok(InviteDetails {
        nickname,
        public_key: public_key.to_string(),
        key_id: public_key[..5].to_string(),
    })
}


// Format:
// ghl_message<sender-public-key><nonce><ciphertext>
//
// The sender's public key is included because the recipient needs:
//
// recipient private key + sender public key
//     -> shared secret
pub fn create_message_key(
    sender_public_key: &str,
    nonce_b64: &str,
    ciphertext_b64: &str,
) -> Result<String, String> {
    
    validate_public_key(sender_public_key)?;
    validate_nonce(nonce_b64)?;

    if ciphertext_b64.is_empty() {
        return Err("Ciphertext cannot be empty.".to_string());
    }

    Ok(format!(
        "{MESSAGE_PREFIX}{sender_public_key}{nonce_b64}{ciphertext_b64}"
    ))
}


// Parse:
// ghl_message<44-char-sender-public-key><16-char-nonce><ciphertext>
// // because in base64 encoded form, nonces are always 16 chars
pub fn extract_details_from_message_key(message_key: &str) -> Result<MessageDetails, String> {
    let remaining = message_key
        .strip_prefix(MESSAGE_PREFIX)
        .ok_or_else(|| "Invalid message prefix.".to_string())?;

    let fixed_length = PUBLIC_KEY_B64_LENGTH + NONCE_B64_LENGTH;

    if remaining.len() <= fixed_length {
        return Err("Encrypted message is incomplete.".to_string());
    }

    let (sender_public_key, nonce_and_ciphertext) = remaining.split_at(PUBLIC_KEY_B64_LENGTH);

    let (nonce_b64, ciphertext_b64) = nonce_and_ciphertext.split_at(NONCE_B64_LENGTH);

    validate_public_key(sender_public_key)?;
    validate_nonce(nonce_b64)?;

    // Validate that the ciphertext is actually Base64.
    STANDARD
        .decode(ciphertext_b64)
        .map_err(|_| "Ciphertext is not valid Base64.".to_string())?;

    Ok(MessageDetails {
        sender_public_key: sender_public_key.to_string(),
        nonce_b64: nonce_b64.to_string(),
        ciphertext_b64: ciphertext_b64.to_string(),
    })
}


// Ensure the public key is valid Base64 and decodes to exactly 32 bytes.
fn validate_public_key(public_key: &str) -> Result<(), String> {
    let decoded = STANDARD
        .decode(public_key)
        .map_err(|_| "Public key is not valid Base64.".to_string())?;

    if decoded.len() != 32 {
        return Err(format!(
            "Public key must decode to 32 bytes, got {}.",
            decoded.len()
        ));
    }

    Ok(())
}


// Ensure the nonce is valid Base64 and decodes to exactly 12 bytes.
fn validate_nonce(nonce_b64: &str) -> Result<(), String> {
    let decoded = STANDARD
        .decode(nonce_b64)
        .map_err(|_| "Nonce is not valid Base64.".to_string())?;

    if decoded.len() != 12 {
        return Err(format!(
            "Nonce must decode to 12 bytes, got {}.",
            decoded.len()
        ));
    }

    Ok(())
}