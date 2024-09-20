use std::future::Future;

use super::models::refresh_token::{CreateRefreshTokenError, RefreshToken};

pub trait RefreshTokenService: Clone + Send + Sync {
    /// Asynchronously creates a new [RefreshToken].
    fn create_refresh_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<RefreshToken, CreateRefreshTokenError>> + Send;
}

pub trait RefreshTokenRepository: Send + Sync + Clone {
    /// Asynchronously creates a new [RefreshToken].
    /// 
    /// # Errors
    /// 
    /// - MUST return [CreateRefreshTokenError::Duplicate] if a token with the same [SerialNumber] already exists.
    fn create_refresh_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<RefreshToken, CreateRefreshTokenError>> + Send;
    
}
