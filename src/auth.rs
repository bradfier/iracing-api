use base64ct::{Base64, Encoding};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub type AuthHash = String;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AuthRequest {
    email: String,
    password: AuthHash,
}

impl AuthRequest {
    pub(crate) fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: generate_hash(email, password),
        }
    }

    pub(crate) fn new_from_hash(email: &str, hash: AuthHash) -> Self {
        Self {
            email: email.to_string(),
            password: hash,
        }
    }
}

/// Generate a login hash in the format specified by the iRacing Devs
///
/// Concatenate the lowercase email onto the end of the password string, generate a SHA256 hash of
/// that combined data then base64 encode the result.
pub fn generate_hash(email: &str, password: &str) -> AuthHash {
    let hash_input = format!("{}{}", password, email.to_lowercase());
    let mut hasher = Sha256::new();
    hasher.update(&hash_input);
    let hash = hasher.finalize();
    Base64::encode_string(&hash)
}
