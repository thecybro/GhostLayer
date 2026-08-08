use crate::protocol::{
    types::{
        CreateInviteInput,
        InviteDetails,
    },
    framing::{
        PUBLIC_KEY_B64_LENGTH,
    },
    v1::validation,
};

pub fn create(
    input: &CreateInviteInput<'_>
) -> Result<String, String> {
    let nickname = input
        .nickname
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("");

    // the framing.rs inside protocol already adds the root prefix and all that,
    // we just gotta make the payload
    Ok(format!("{}{}", input.public_key, nickname)) 
}

pub fn parse(
    invite: &str
) -> Result<InviteDetails, String> {
    // validate public key here
    // 
    if invite.len() < PUBLIC_KEY_B64_LENGTH {
        return Err("Invite key is too short.".to_string());
    };
    // First 44 characters are always the Base64 public key.
    // Anything afterward is the optional nickname.
    let (public_key, nickname) = invite.split_at(PUBLIC_KEY_B64_LENGTH);
    
    validation::public_key(public_key)?;

    let nickname = if nickname.trim().is_empty() {
        None
    } else {
        Some(nickname.to_string())
    };
    
    Ok(InviteDetails{
        nickname,
        public_key: public_key.to_string(),
        key_id: public_key[0..5].to_string(),
    })
}

