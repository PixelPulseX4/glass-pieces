mod cipher;
mod discover_input;
mod error;

pub use error::IxLoadError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AngelInstruction {
    pub git_url: String,
    pub git_ssh_priv_key: String,
    pub git_known_hosts: String,
    pub entrypoint: String,
}

impl AngelInstruction {
    pub async fn load_owned() -> Result<Self, IxLoadError> {
        let cipher = cipher::Cipher::new()?;
        let path = discover_input::discover_input()?;
        let enc_input = tokio::fs::read(path).await?;
        let json_bytes = cipher.decrypt(enc_input)?;
        let json = std::str::from_utf8(&json_bytes)?;
        Ok(serde_json::from_str(json)?)
    }
}
