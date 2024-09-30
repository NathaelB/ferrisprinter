use std::future::Future;

use crate::domain::token::models::access_token::{AccessToken, AccessTokenError};

pub trait AccessTokenService: Clone + Send + Sync + 'static {
    fn create_access_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> impl Future<Output = Result<AccessToken, AccessTokenError>> + Send;
}
pub trait AccessTokenRepository: Send + Sync + Clone + 'static {
    fn create_access_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> impl Future<Output = Result<AccessToken, AccessTokenError>> + Send;
}
