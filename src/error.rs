use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error (status {0}): {1}")]
    Api(u16, String),

    #[error("Invalid or missing API key")]
    InvalidApiKey,

    #[error("Server error (500)")]
    ServerError,

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid game ID: {0}")]
    InvalidGameId(String),

    #[error("Invalid data source: {0}")]
    InvalidDataSource(String),
}
