pub mod dto;

pub use dto::*;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub fn verify_signature(secret_key: &str, payload: &[u8], signature: &str) -> bool {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload);
    let payload_sig = base64::encode(mac.finalize().into_bytes());

    payload_sig == signature
}
