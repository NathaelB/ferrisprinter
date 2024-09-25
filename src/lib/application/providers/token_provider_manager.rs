use std::collections::HashMap;

use crate::domain::token::ports::provider_token_service::{ProviderTokenService, ProviderType};

pub struct TokenProviderManager<P> {
    providers: HashMap<ProviderType, P>,
}

impl<P> TokenProviderManager<P>
where
    P: ProviderTokenService,
{
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider(&mut self, provider_type: ProviderType, provider: P) {
        self.providers.insert(provider_type, provider);
    }

    pub fn get_provider(&self, provider_type: &ProviderType) -> Option<&P> {
        self.providers.get(provider_type)
    }
}
