use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, Generate, KeyInit},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod helper {
    use super::{Generate, Key, Nonce};
    pub fn gen_nonce() -> Nonce {
        Nonce::generate()
    }

    pub fn gen_key() -> Key {
        Key::generate()
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
}
