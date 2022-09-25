use sha2::{Digest};

pub fn sha256_digest<S: Into<String>>(text: S) -> String {
  hex::encode(sha2::Sha256::digest(text.into().as_bytes()))
}