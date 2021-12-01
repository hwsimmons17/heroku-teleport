use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};
use sodalite::sign_attached_open;
use tweetnacl::sign_open;
use tweetnacl_sys::crypto_sign_ed25519_tweet_open;

pub struct VerifyReturn {
    pub success: bool,
    pub message: u8,
}

pub fn verify(message: &str, signature: &str) -> VerifyReturn {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let message_hashed = hasher.finalize();
    let hashed_message_bytes = &mut message_hashed.clone()[..];

    let mut split = message.split(",");
    let pubkey_key_value = split.next().unwrap();
    let mut pubkey_iterator = pubkey_key_value.split(":");
    let key = pubkey_iterator.next().unwrap();
    if key != "pubkey" {
        return VerifyReturn {
            success: false,
            message: 10,
        };
    }
    let value = pubkey_iterator.next().unwrap();
    let pubkey = &value.parse::<Pubkey>().unwrap().to_bytes();
    let signature_bytes = signature.as_bytes();

    // let result = sign_open(hashed_message_bytes, signature_bytes, pubkey);
    let mut mlen = hashed_message_bytes.len() as tweetnacl_sys::c_ulonglong;
    let x = unsafe {
        crypto_sign_ed25519_tweet_open(
            hashed_message_bytes.as_mut_ptr(),
            &mut mlen,
            signature_bytes.as_ptr(),
            signature_bytes.len() as tweetnacl_sys::c_ulonglong,
            pubkey.as_ptr(),
        )
    };
    if x == -1 {
        return VerifyReturn {
            success: false,
            message: 1,
        };
    }
    VerifyReturn {
        success: true,
        message: 0,
    }
}
