use std::future::Future;

use crate::domain::token::models::refresh_token::{
    CreateRefreshTokenError, FindRefreshTokenError, RefreshToken, RefreshTokenError,
};

use super::provider_token_service::ProviderType;

pub trait RefreshTokenService: Clone + Send + Sync + 'static {
    /// Asynchronously creates a new [RefreshToken].
    fn create_refresh_token(
        &self,
        username: String,
        password: String,
        serial_number: &str,
        provider_type: ProviderType,
    ) -> impl Future<Output = Result<RefreshToken, CreateRefreshTokenError>> + Send;
    fn find_by_serial_number(
        &self,
        serial_number: &str,
    ) -> impl Future<Output = Result<RefreshToken, FindRefreshTokenError>> + Send;
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
    fn find_by_serial_number(
        &self,
        serial_number: &str,
    ) -> impl Future<Output = Result<RefreshToken, FindRefreshTokenError>> + Send;
    fn delete_by_serial_number(
        &self,
        serial_number: &str,
    ) -> impl Future<Output = Result<(), RefreshTokenError>> + Send;
}
