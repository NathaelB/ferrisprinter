use std::collections::HashMap;

use crate::domain::{
    ams::ports::provider_ams_service::ProviderAmsService,
    token::ports::provider_token_service::ProviderType,
};

#[derive(Debug, Clone)]
pub struct AmsProviderManager<P> {
    providers: HashMap<ProviderType, P>,
}

impl<P> AmsProviderManager<P>
where
    P: ProviderAmsService,
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

impl<P> Default for AmsProviderManager<P>
where
    P: ProviderAmsService,
{
    fn default() -> Self {
        Self::new()
    }
}
