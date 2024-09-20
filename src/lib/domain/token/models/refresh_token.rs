use thiserror::Error;

use super::token::{SerialNumber, Token};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshToken {
    id: uuid::Uuid,
    serial_number: SerialNumber,
    token: Token,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
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

#[derive(Debug, Error)]
pub enum CreateRefreshTokenError {
    #[error("Token with serial number {name} already exists")]
    Duplicate { name: SerialNumber },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
