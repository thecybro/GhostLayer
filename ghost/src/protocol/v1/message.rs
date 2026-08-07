use crate::protocol::{
    types::{
        CreateMessageInput, 
        MessageDetails,
        PUBLIC_KEY_B64_LENGTH,
        NONCE_B64_LENGTH,
    },
    v1::validation,
};

pub fn create(
    input: &CreateMessageInput<'_>
) -> Result<String, String> {
    validation::public_key(input.sender_public_key)?;
    validation::nonce_b64(input.nonce_b64)?;
    validation::ciphertext_b64(input.ciphertext_b64)?;

    Ok(
        format!(
            "{}{}{}",
            input.sender_public_key,
            input.nonce_b64,
            input.ciphertext_b64)
        )
}

pub fn parse(
    message: &str
) -> Result<MessageDetails, String> {
    
    let fixed_length = PUBLIC_KEY_B64_LENGTH + NONCE_B64_LENGTH;

    if message.len() <= fixed_length {
        return Err("Encrypted message is incomplete.".to_string());
    }

    let (sender_public_key, nonce_and_ciphertext) = message.split_at(PUBLIC_KEY_B64_LENGTH);

    let (nonce_b64, ciphertext_b64) = nonce_and_ciphertext.split_at(NONCE_B64_LENGTH);

    validation::public_key(sender_public_key)?;
    validation::nonce_b64(nonce_b64)?;

    Ok(MessageDetails {
        sender_public_key: sender_public_key.to_string(),
        nonce_b64: nonce_b64.to_string(),
        ciphertext_b64: ciphertext_b64.to_string(),
    })
}
