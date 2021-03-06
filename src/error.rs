#![allow(unreachable_patterns)]

use async_std::future::TimeoutError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use url::ParseError;

#[derive(Debug, Deserialize)]
pub struct LunoError {
    pub error: String,
    pub error_code: String,
    pub error_action: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Error {
    UrlParseError(String),
    SurfError(String),
    TimeoutError(String),
    ApiError(LunoError),
    InvalidCurrencyPair(String),
    InvalidOrderType(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UrlParseError(message) => write!(f, "URL parse error: {}", message),
            Error::SurfError(message) => write!(f, "Surf Error: {}", message),
            Error::TimeoutError(message) => write!(f, "Timeout Error: {}", message),
            Error::ApiError(error) => write!(f, "{}: {}", error.error_code, error.error),
            Error::InvalidCurrencyPair(str) => {
                write!(f, "Cannot convert {} to any currency pair", str)
            }
            Error::InvalidOrderType(str) => {
                write!(f, "Cannot convert {} to any order type", str)
            }
            _ => write!(f, "Unable to process request at this time"),
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::UrlParseError(err.to_string())
    }
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Self {
        Error::SurfError(err.to_string())
    }
}

impl From<TimeoutError> for Error {
    fn from(err: TimeoutError) -> Self {
        Error::TimeoutError(err.to_string())
    }
}
