// #[derive(Error, Debug)]
// pub enum EchoError {
//     #[error("Http request failed: {0}")]
//     HttpError(#[from] reqwest::Error),
//
//     #[error("Failed to deserialize JSON response: {0}")]
//     JsonError(#[from] serde_json::Error),
//
//     #[error("Failed to parse text response: {0}")]
//     TextParseError(String),
//
//     #[error("Failed to parse binary response: {0}")]
//     BinaryParseError(String),
//
//     #[error("Unsupported response type: {0:?}")]
//     UnsupportedResponseType(ResponseType),
//
//     #[error("URL construction failed")]
//     UrlError,
// }
