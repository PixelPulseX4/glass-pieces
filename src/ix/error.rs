#[derive(Debug)]
pub enum IxLoadError {
    CipherInitError(super::cipher::CipherInitError),
    CipherDecryptError(super::cipher::CipherDecryptError),
    DiscoverFileError(super::discover_input::DiscoverInputError),
    IoError(tokio::io::Error),
    IxNotUtf8(std::str::Utf8Error),
    IxBadJson(serde_json::Error),
}

impl std::fmt::Display for IxLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CipherInitError(e) => write!(f, "Erreur d'init du cipher: {}", e),
            Self::CipherDecryptError(e) => write!(f, "Erreur de decrypt cipher: {}", e),
            Self::DiscoverFileError(e) => write!(f, "Échec du recherche de l'input: {}", e),
            Self::IoError(e) => write!(f, "Erreur async i/o: {}", e),
            Self::IxNotUtf8(e) => write!(f, "L'ix est pas du UTF-8 correct: {}", e),
            Self::IxBadJson(e) => write!(f, "L'ix est pas du bon json: {}", e),
        }
    }
}

impl std::error::Error for IxLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CipherInitError(e) => Some(e),
            Self::CipherDecryptError(e) => Some(e),
            Self::DiscoverFileError(e) => Some(e),
            Self::IoError(e) => Some(e),
            Self::IxNotUtf8(e) => Some(e),
            Self::IxBadJson(e) => Some(e),
        }
    }
}

impl From<tokio::io::Error> for IxLoadError {
    fn from(value: tokio::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<super::discover_input::DiscoverInputError> for IxLoadError {
    fn from(value: super::discover_input::DiscoverInputError) -> Self {
        Self::DiscoverFileError(value)
    }
}

impl From<super::cipher::CipherInitError> for IxLoadError {
    fn from(value: super::cipher::CipherInitError) -> Self {
        Self::CipherInitError(value)
    }
}

impl From<super::cipher::CipherDecryptError> for IxLoadError {
    fn from(value: super::cipher::CipherDecryptError) -> Self {
        Self::CipherDecryptError(value)
    }
}

impl From<std::str::Utf8Error> for IxLoadError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::IxNotUtf8(value)
    }
}

impl From<serde_json::Error> for IxLoadError {
    fn from(value: serde_json::Error) -> Self {
        Self::IxBadJson(value)
    }
}
