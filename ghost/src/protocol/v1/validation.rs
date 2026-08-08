use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};

// Ensure the public key is valid Base64 and decodes to exactly 32 bytes.
pub fn public_key(public_key: &str) -> Result<(), String> {
    let decoded = STANDARD
        .decode(public_key)
        .map_err(|_| "That public key is not valid Base64, so whatever it came from is damaged.".to_string())?;

    if decoded.len() != 32 {
        return Err(format!(
            "That public key is {} bytes long, but a GhostLayer public key is 32 bytes. Whatever it came from is damaged.",
            decoded.len()
        ));
    }

    Ok(())
}


// Ensure the nonce is valid Base64 and decodes to exactly 12 bytes.
pub fn nonce_b64(nonce_b64: &str) -> Result<(), String> {
    let decoded = STANDARD
        .decode(nonce_b64)
        .map_err(|_| "This message's nonce is not valid Base64, so the message was damaged before it reached you.".to_string())?;

    if decoded.len() != 12 {
        return Err(format!(
            "This message's nonce is {} bytes long, but it should be 12. The message was damaged before it reached you.",
            decoded.len()
        ));
    }

    Ok(())
}

pub fn ciphertext_b64(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(
            "This message has a GhostLayer header but no encrypted content, so it was cut off.".into(),
        );
    }

    STANDARD.decode(value).map_err(|_| {
            "This message's encrypted content is not valid Base64, so it was damaged before it reached you."
    })?;

    Ok(())
}
