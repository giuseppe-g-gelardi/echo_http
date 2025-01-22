use thiserror::Error;

#[derive(Error, Debug)]
pub enum EchoError {
    #[error("Http request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to deserialize JSON response: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL construction failed")]
    UrlError,
}
