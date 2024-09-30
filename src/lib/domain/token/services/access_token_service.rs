use crate::domain::token::{
    models::access_token::{AccessToken, AccessTokenError},
    ports::access_token::{AccessTokenRepository, AccessTokenService},
};

#[derive(Debug, Clone)]
pub struct AccessTokenServiceImpl<R>
where
    R: AccessTokenRepository,
{
    access_token_repository: R,
}

impl<R> AccessTokenServiceImpl<R>
where
    R: AccessTokenRepository,
{
    pub fn new(access_token_repository: R) -> Self {
        Self {
            access_token_repository,
        }
    }
}

impl<R> AccessTokenService for AccessTokenServiceImpl<R>
where
    R: AccessTokenRepository,
{
    async fn create_access_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> Result<AccessToken, AccessTokenError> {
        self.access_token_repository
            .create_access_token(token, serial_number)
            .await
    }

    async fn delete_by_serial_number(&self, serial_number: &str) -> Result<(), AccessTokenError> {
        self.access_token_repository
            .delete_by_serial_number(serial_number)
            .await
    }
}
