use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit, OsRng as AeadOsRng},
};
use chacha20poly1305::aead::rand_core::RngCore;

pub fn encrypt_message(key_bytes: &[u8; 32], plaintext: &str) -> (Vec<u8>, Vec<u8>) {
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut AeadOsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).unwrap();

    (nonce.to_vec(), ciphertext)
}

// Dont yet know what it should get and give
// referenced section "In-place Usage (eliminates alloc requirement)"
// from "https://docs.rs/chacha20poly1305/latest/chacha20poly1305/#xchacha20poly1305"
// for encryption function
// 
pub fn decrypt_text(key_bytes: &[u8; 32], nonce: Vec<u8>, ciphertext: Vec<u8>) -> Result<String, String> {
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    
    let nonce = Nonce::from_slice(&nonce);
    let decrypted_text = cipher.decrypt(nonce, ciphertext.as_ref());

    match decrypted_text {
        Ok(bytes) => {
            let decrypted_text = String::from_utf8(bytes).expect("invalid utf8");
            Ok(decrypted_text)
        }
        Err(_) => {
            Err("Decryption failed".to_string())
        }
    }
}