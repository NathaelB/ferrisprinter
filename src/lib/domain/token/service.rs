use std::sync::Arc;

use crate::application::providers::token_provider_manager::TokenProviderManager;

use super::{
    models::refresh_token::{CreateRefreshTokenError, FindRefreshTokenError, RefreshToken},
    ports::{
        provider_token_service::{ProviderTokenService, ProviderType},
        refresh_token::{RefreshTokenRepository, RefreshTokenService},
    },
};

#[derive(Debug, Clone)]
pub struct RefreshTokenServiceImpl<R, P>
where
    R: RefreshTokenRepository,
    P: ProviderTokenService,
{
    refresh_token_repository: R,
    token_provider_manager: Arc<TokenProviderManager<P>>,
}

impl<R, P> RefreshTokenServiceImpl<R, P>
where
    R: RefreshTokenRepository,
    P: ProviderTokenService,
{
    pub fn new(
        refresh_token_repository: R,
        token_provider_manager: Arc<TokenProviderManager<P>>,
    ) -> Self {
        Self {
            refresh_token_repository,
            token_provider_manager,
        }
    }
}

impl<R, P> RefreshTokenService for RefreshTokenServiceImpl<R, P>
where
    R: RefreshTokenRepository,
    P: ProviderTokenService,
{
    async fn create_refresh_token(
        &self,
        username: String,
        password: String,
        serial_number: &str,
        provider_type: ProviderType,
    ) -> Result<RefreshToken, CreateRefreshTokenError> {
        let provider = self
            .token_provider_manager
            .get_provider(&provider_type)
            .ok_or(CreateRefreshTokenError::ProviderNotFound)?;

        let tokens = provider.authenticate(username, password).await.unwrap();
        self.refresh_token_repository
            .create_refresh_token(tokens.refresh_token.as_str(), serial_number)
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
