use super::token::{SerialNumber, Token};
use derive_more::From;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessToken {
    pub id: uuid::Uuid,
    pub serial_number: SerialNumber,
    pub token: Token,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

impl AccessToken {
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
pub struct AccessTokenRow {
    pub id: uuid::Uuid,
    pub serial_number: String,
    pub token: String,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Error)]
pub enum AccessTokenError {
    #[error("Token with serial number {serial_number} not found")]
    NotFound { serial_number: SerialNumber },
    #[error("Error creating access token")]
    CreateError,
}
