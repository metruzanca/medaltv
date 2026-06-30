use thiserror::Error;

#[derive(Error, Debug)]
pub enum MedalError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON decode failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("user not found: {0}")]
    UserNotFound(String),

    #[error("authentication failed")]
    AuthFailed,
}
