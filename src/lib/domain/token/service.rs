use super::{
    models::refresh_token::{CreateRefreshTokenError, FindRefreshTokenError, RefreshToken},
    ports::refresh_token::{RefreshTokenRepository, RefreshTokenService},
};

#[derive(Debug, Clone)]
pub struct RefreshTokenServiceImpl<R>
where
    R: RefreshTokenRepository,
{
    refresh_token_repository: R,
}

impl<R> RefreshTokenServiceImpl<R>
where
    R: RefreshTokenRepository,
{
    pub fn new(refresh_token_repository: R) -> Self {
        Self {
            refresh_token_repository,
        }
    }
}

impl<R> RefreshTokenService for RefreshTokenServiceImpl<R>
where
    R: RefreshTokenRepository,
{
    async fn create_refresh_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> Result<RefreshToken, CreateRefreshTokenError> {
        self.refresh_token_repository
            .create_refresh_token(token, serial_number)
            .await
    }

    async fn find_by_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<RefreshToken, FindRefreshTokenError> {
        self.refresh_token_repository
            .find_by_serial_number(serial_number)
            .await
    }
}
