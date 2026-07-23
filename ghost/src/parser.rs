use serde::Serialize;

#[derive(Serialize)]
pub struct ParsedOutput {
    pub nickname: String, // friend's nickname, from the invite
    pub public_key: String,
    pub key_id: String, // public_key[0..5]
}

// output: ghl<pubkey fixed-width><nickname remainder>
pub fn create_invite_key(public_key: String, name: Option<String>) -> String {
    let used_name = name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_default();
    
    format!("ghl{public_key}{used_name}")
}

// Raw Key(input): ghl<pubkey fixed-width(44 in normal form, 32 in decoded form)><nickname remainder>
// processing:
// brand       = invite_key[0..3]      // "ghl"
// public_key  = invite_key[3..47]     // 44 chars, right after brand
// nickname    = invite_key[47..]      // whatever remains
// output: 
// Ok(ParsedOutput {
//  name: friend's name,
//  public_key: friend's public key
//  key_id: public_key[0..5]
// })
// Err(String) if any error happens or the key doesn't contain ghl at the front
// but that's just a invite we accept, our logic would still work with the raw public
// key, after strapping ghl invite
// 
pub fn extract_details_from_invite_key(invite_key: String) -> Result<ParsedOutput, String> {
    if !invite_key.starts_with("ghl") {
        return Err("Invalid invite key.".into());
    }

    if invite_key.len() < 47 {
        return Err("invite key is too short.".into());
    }

    let public_key = &invite_key[3..47];

    Ok(ParsedOutput {
        nickname: invite_key[47..].to_string(),
        public_key: public_key.to_string(),
        key_id: public_key[..5].to_string(),
    })
}
