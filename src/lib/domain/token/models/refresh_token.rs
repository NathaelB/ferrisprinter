use thiserror::Error;

use derive_more::From;

use super::token::{SerialNumber, Token};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshToken {
    pub id: uuid::Uuid,
    pub serial_number: SerialNumber,
    pub token: Token,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl RefreshToken {
    pub fn new(
        id: uuid::Uuid,
        serial_number: SerialNumber,
        token: Token,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
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
pub struct CreateRefreshTokenRequest {
    token: Token,
    serial_number: SerialNumber,
}

impl CreateRefreshTokenRequest {
    pub fn new(token: Token, serial_number: SerialNumber) -> Self {
        Self {
            token,
            serial_number,
        }
    }

    pub fn serial_number(&self) -> &SerialNumber {
        &self.serial_number
    }

    pub fn token(&self) -> &Token {
        &self.token
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
}
