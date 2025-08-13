#[derive(Debug)]
pub enum CipherInitError {
    EnvError(std::env::VarError),
    HexError(hex::FromHexError),
    InvalidKey,
}

#[derive(Debug)]
pub enum CipherDecryptError {
    MalformedInput,
    AesGcmError(aes_gcm::Error),
}

impl std::fmt::Display for CipherInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvError(e) => write!(f, "Erreur ENV: {}", e),
            Self::HexError(e) => write!(f, "Erreur HEX: {}", e),
            Self::InvalidKey => write!(f, "Clef cipher invalide"),
        }
    }
}

impl std::error::Error for CipherInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::EnvError(e) => Some(e),
            Self::HexError(e) => Some(e),
            Self::InvalidKey => None,
        }
    }
}

impl std::fmt::Display for CipherDecryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedInput => write!(f, "L'input est malformé"),
            Self::AesGcmError(e) => write!(f, "Erreur de décryption: {}", e),
        }
    }
}

impl std::error::Error for CipherDecryptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MalformedInput => None,
            Self::AesGcmError(_e) => None,
        }
    }
}

impl From<aes_gcm::Error> for CipherDecryptError {
    fn from(value: aes_gcm::Error) -> Self {
        Self::AesGcmError(value)
    }
}

impl From<hex::FromHexError> for CipherInitError {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexError(value)
    }
}

impl From<std::env::VarError> for CipherInitError {
    fn from(value: std::env::VarError) -> Self {
        Self::EnvError(value)
    }
}
