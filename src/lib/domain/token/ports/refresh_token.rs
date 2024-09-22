use std::future::Future;

use crate::domain::token::models::refresh_token::{CreateRefreshTokenError, RefreshToken};

pub trait RefreshTokenService: Clone + Send + Sync + 'static {
    /// Asynchronously creates a new [RefreshToken].
    fn create_refresh_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> impl Future<Output = Result<RefreshToken, CreateRefreshTokenError>> + Send;
}

pub trait RefreshTokenRepository: Send + Sync + Clone + 'static {
    /// Asynchronously creates a new [RefreshToken].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateRefreshTokenError::Duplicate] if a token with the same [SerialNumber] already exists.
    fn create_refresh_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> impl Future<Output = Result<RefreshToken, CreateRefreshTokenError>> + Send;
}
