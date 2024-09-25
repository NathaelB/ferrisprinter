use std::future::Future;

use crate::domain::token::models::token::{CreateTokensError, Tokens};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ProviderType {
    BambuLab,
}
pub trait ProviderTokenService: Send + Sync + Clone + 'static {
    fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<Tokens, CreateTokensError>> + Send;
}
