use thiserror::Error;

use derive_more::From;
use time::OffsetDateTime;

use super::token::{SerialNumber, Token};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub serial_number: SerialNumber,
    pub token: Token,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl RefreshToken {
    pub fn new(
        id: uuid::Uuid,
        serial_number: SerialNumber,
        token: Token,
        created_at: time::OffsetDateTime,
        updated_at: time::OffsetDateTime,
    ) -> Self {
        Self {
            id,
            serial_number,
            token,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct RefreshTokenRow {
    pub id: uuid::Uuid,
    pub serial_number: String,
    pub token: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From)]
pub struct CreateRefreshTokenRequest {
    username: String,
    password: String,
    serial_number: SerialNumber,
}

impl CreateRefreshTokenRequest {
    pub fn new(username: String, password: String, serial_number: SerialNumber) -> Self {
        Self {
            username,
            password,
            serial_number,
        }
    }

    pub fn serial_number(&self) -> &SerialNumber {
        &self.serial_number
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Error)]
pub enum CreateRefreshTokenError {
    #[error("Token with serial number {name} already exists")]
    Duplicate { name: SerialNumber },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Token provider not found")]
    ProviderNotFound,
}

#[derive(Debug, Error)]
pub enum FindRefreshTokenError {
    #[error("Token with serial number {serial_number} not found")]
    NotFound { serial_number: SerialNumber },
}

#[derive(Debug, Error)]
pub enum RefreshTokenError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
