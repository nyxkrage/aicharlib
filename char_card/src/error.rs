#[derive(thiserror::Error, Debug)]
pub enum CharCardError {
    #[error("No character data found in card")]
    NoCharacterFound,
    #[error("Character data is not valid base64")]
    InvalidBase64Data(#[from] base64::DecodeError),
    #[error("Character data is not valid UTF-8")]
    InvalidUtf8Data(#[from] std::str::Utf8Error),
    #[error("Character data is not valid JSON")]
    InvalidJsonData(#[from] serde_json::Error),
    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, CharCardError>;
