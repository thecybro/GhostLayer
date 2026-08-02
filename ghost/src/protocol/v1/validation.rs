use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};

// Ensure the public key is valid Base64 and decodes to exactly 32 bytes.
pub fn public_key(public_key: &str) -> Result<(), String> {
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
pub fn nonce_b64(nonce_b64: &str) -> Result<(), String> {
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

pub fn ciphertext_b64(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(
            "Ciphertext cannot be empty.".into(),
        );
    }

    STANDARD.decode(value).map_err(|_| {
            "Ciphertext is not valid Base64."
    })?;

    Ok(())
}
