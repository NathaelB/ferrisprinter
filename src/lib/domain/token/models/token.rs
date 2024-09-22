use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SerialNumber(String);

#[derive(Clone, Debug, Error)]
#[error("Serial number cannot be empty")]
pub struct SerialNumberEmptyError;

impl SerialNumber {
    pub fn new(value: &str) -> Result<SerialNumber, SerialNumberEmptyError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            Err(SerialNumberEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token(String);

#[derive(Clone, Debug, Error)]
#[error("Token cannot be empty")]
pub struct TokenEmptyError;

impl Token {
    pub fn new(value: &str) -> Result<Token, TokenEmptyError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            Err(TokenEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
