use serde::{Serialize};

// 'a because we are borrowing the data, not owning it
// 2 structs below dont have 'a because they own the data 
// 
#[derive(Debug, Serialize)]
pub struct CreateInviteInput<'a> {
    pub public_key: &'a str,
    pub nickname: Option<&'a str>
}

#[derive(Debug, Serialize)]
pub struct CreateMessageInput<'a> {
    pub sender_public_key: &'a str,
    pub nonce_b64: &'a str,
    pub ciphertext_b64: &'a str
}

#[derive(Debug, Serialize)]
pub struct InviteDetails {
    pub nickname: Option<String>,
    pub public_key: String,
    pub key_id: String,
}

#[derive(Debug, Serialize)]
pub struct MessageDetails {
    // This must be the sender's public key.
    // The receiver combines it with their own private key.
    pub sender_public_key: String,

    pub nonce_b64: String,
    pub ciphertext_b64: String,
}

// X25519 public keys are 32 bytes.
// Standard Base64 represents 32 bytes using 44 characters.
pub const PUBLIC_KEY_B64_LENGTH: usize = 44;

// ChaCha20Poly1305 uses a 12-byte nonce.
// Standard Base64 represents 12 bytes using 16 characters.
pub const NONCE_B64_LENGTH: usize = 16;
