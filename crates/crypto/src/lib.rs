use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, Generate, KeyInit},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Key = chacha20poly1305::Key;
pub type Nonce = chacha20poly1305::Nonce;

pub mod helper {
    use super::{Generate, Key, Nonce, Result};
    pub fn gen_nonce() -> Nonce {
        Nonce::generate()
    }

    pub fn gen_key() -> Key {
        Key::generate()
    }

    pub fn key_from(bytes: Vec<u8>) -> Result<Key> {
        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "key must be exactly 32 bytes".to_string())?;

        Ok(Key::from(array))
    }
}

pub fn encrypt(key: &Key, plaintext: Vec<u8>) -> Result<(Nonce, Vec<u8>)> {
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = helper::gen_nonce();

    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())?;

    Ok((nonce, ciphertext))
}

pub fn decrypt(key: &Key, ciphertext: Vec<u8>, nonce: &Nonce) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key);

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::result::Result;

    #[test]
    fn encryption_decryption() -> Result<(), Box<dyn std::error::Error>> {
        let key = helper::gen_key();
        let plaintext = "I alone am the honored one.\n - Saturo Gojo.";

        let (nonce, ciphertext) = encrypt(&key, plaintext.into())?;

        let transformed = decrypt(&key, ciphertext, &nonce)?;

        assert_eq!(plaintext.as_bytes(), transformed);

        Ok(())
    }

    #[test]
    fn helper_generates_expected_sizes() {
        let key = helper::gen_key();
        let nonce = helper::gen_nonce();

        assert_eq!(key.as_slice().len(), 32);
        assert_eq!(nonce.as_slice().len(), 12);
    }

    #[test]
    fn key_from_validates_length() {
        let key = helper::key_from(vec![7u8; 32]).expect("expected valid key");
        assert_eq!(key.as_slice(), vec![7u8; 32]);

        let err = helper::key_from(vec![1u8; 31]).unwrap_err();
        assert_eq!(err.to_string(), "key must be exactly 32 bytes");
    }

    #[test]
    fn decrypt_fails_with_wrong_key_or_tampered_ciphertext() {
        let key = helper::gen_key();
        let other_key = helper::gen_key();
        let plaintext = b"secret".to_vec();

        let (nonce, mut ciphertext) = encrypt(&key, plaintext.clone()).expect("encrypt");

        assert!(decrypt(&other_key, ciphertext.clone(), &nonce).is_err());

        if let Some(byte) = ciphertext.get_mut(0) {
            *byte ^= 0b1010_1010;
        }

        assert!(decrypt(&key, ciphertext, &nonce).is_err());
    }
}
