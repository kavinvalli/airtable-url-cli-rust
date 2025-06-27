use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::RequestError(e) => write!(f, "Request error: {}", e),
            ClientError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl StdError for ClientError {}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::RequestError(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::ParseError(err)
    }
}
