use core::fmt;
use std::{error::Error, fmt::Display};

use reqwest::StatusCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeatimeErrorKind {
    HttpError,
    ParseError,
    SerializationError,
    Other,
}

impl Display for TeatimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TeatimeErrorKind::HttpError => write!(f, "HTTP error"),
            TeatimeErrorKind::ParseError => write!(f, "Parsing error"),
            TeatimeErrorKind::SerializationError => write!(f, "Serialization error"),
            TeatimeErrorKind::Other => write!(f, "error"),
        }
    }
}

/// Represents some kind of error that can occur when interacting with the Gitea API.
/// This simply wraps a message and a status code.
#[derive(Debug, Clone)]
pub struct TeatimeError {
    pub message: String,
    pub kind: TeatimeErrorKind,
    pub status_code: reqwest::StatusCode,
}
impl Error for TeatimeError {}
impl Display for TeatimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// A type alias for a [std::result::Result] that uses [TeatimeError] as the error type.
/// We define this purely for convenience.
pub type Result<T> = std::result::Result<T, TeatimeError>;

/// Converts a [reqwest::Error] into a [TeatimeError].
/// This method exists for us to be able to directly call the unwrap operator (`?`) on the result
/// of a [reqwest::Result].
impl From<reqwest::Error> for TeatimeError {
    fn from(err: reqwest::Error) -> Self {
        let mut kind = TeatimeErrorKind::HttpError;
        if err.is_decode() {
            kind = TeatimeErrorKind::SerializationError;
        }
        TeatimeError {
            message: format!("{}", err),
            status_code: err.status().unwrap_or(StatusCode::BAD_REQUEST),
            kind,
        }
    }
}

impl From<serde_json::Error> for TeatimeError {
    fn from(err: serde_json::Error) -> Self {
        TeatimeError {
            message: format!("{}", err),
            status_code: StatusCode::BAD_REQUEST,
            kind: TeatimeErrorKind::ParseError,
        }
    }
}

impl From<Box<dyn Error>> for TeatimeError {
    fn from(err: Box<dyn Error>) -> Self {
        TeatimeError {
            message: format!("{}", err),
            status_code: StatusCode::BAD_REQUEST,
            kind: TeatimeErrorKind::Other,
        }
    }
}
