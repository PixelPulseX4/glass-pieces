mod error;

use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
pub use error::{CipherDecryptError, CipherInitError};

pub struct Cipher {
    cipher: Aes256Gcm,
}

impl Cipher {
    pub fn new() -> Result<Self, CipherInitError> {
        let key_hex = std::env::var("MYKEY")?;
        let key_heap = hex::decode(&key_hex)?;
        let key: [u8; 32] = key_heap
            .try_into()
            .map_err(|_| CipherInitError::InvalidKey)?;

        let cipher = Aes256Gcm::new(&key.into());

        Ok(Self { cipher })
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>, CipherDecryptError> {
        if data.len() <= 12 {
            return Err(CipherDecryptError::MalformedInput);
        }

        let nonce = &data[..12];
        let ciphertext = &data[12..];

        Ok(self.cipher.decrypt(nonce.into(), ciphertext)?)
    }
}
