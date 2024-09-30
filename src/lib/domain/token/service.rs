use std::sync::Arc;

use crate::application::providers::token_provider_manager::TokenProviderManager;

use super::{
    models::{
        access_token,
        refresh_token::{CreateRefreshTokenError, FindRefreshTokenError, RefreshToken},
    },
    ports::{
        access_token::AccessTokenService,
        provider_token_service::{ProviderTokenService, ProviderType},
        refresh_token::{RefreshTokenRepository, RefreshTokenService},
    },
};

#[derive(Debug, Clone)]
pub struct RefreshTokenServiceImpl<R, P, S>
where
    R: RefreshTokenRepository,
    S: AccessTokenService,
    P: ProviderTokenService,
{
    refresh_token_repository: R,
    access_token_service: Arc<S>,
    token_provider_manager: Arc<TokenProviderManager<P>>,
}

impl<R, P, S> RefreshTokenServiceImpl<R, P, S>
where
    R: RefreshTokenRepository,
    P: ProviderTokenService,
    S: AccessTokenService,
{
    pub fn new(
        refresh_token_repository: R,
        access_token_service: Arc<S>,
        token_provider_manager: Arc<TokenProviderManager<P>>,
    ) -> Self {
        Self {
            refresh_token_repository,
            access_token_service: Arc::clone(&access_token_service),
            token_provider_manager,
        }
    }
}

impl<R, P, S> RefreshTokenService for RefreshTokenServiceImpl<R, P, S>
where
    R: RefreshTokenRepository,
    P: ProviderTokenService,
    S: AccessTokenService,
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


        let _ = self.access_token_service.delete_by_serial_number(serial_number).await;
        let _ = self.refresh_token_repository.delete_by_serial_number(serial_number).await;

        let _ = self
            .access_token_service
            .create_access_token(tokens.access_token.as_str(), serial_number)
            .await;
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
