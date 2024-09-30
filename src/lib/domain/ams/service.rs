use std::sync::Arc;

use crate::{
    application::providers::ams_provider_manager::AmsProviderManager,
    domain::token::ports::provider_token_service::ProviderType,
};

use super::{
    models::AmsError,
    ports::{ams_service::AmsService, provider_ams_service::ProviderAmsService},
};

#[derive(Debug, Clone)]
pub struct AmsServiceImpl<P>
where
    //R: AmsRepository,
    P: ProviderAmsService,
{
    //ams_repository: R,
    ams_provider_manager: Arc<AmsProviderManager<P>>,
}

impl<P> AmsServiceImpl<P>
where
    P: ProviderAmsService,
{
    pub fn new(ams_provider_manager: Arc<AmsProviderManager<P>>) -> Self {
        AmsServiceImpl {
            //ams_repository,
            ams_provider_manager,
        }
    }
}

impl<P> AmsService for AmsServiceImpl<P>
where
    P: ProviderAmsService + 'static,
{
    async fn refresh_rfid(
        &self,
        tray_id: String,
        provider: ProviderType,
        device_id: String,
    ) -> Result<(), AmsError> {
        let provider = self.ams_provider_manager.get_provider(&provider);

        let provider = match provider {
            Some(provider) => provider,
            None => return Err(AmsError::ProviderNotFound),
        };

        provider.refresh_rfid(tray_id, device_id, "".to_string(), "".to_string()).await?;

        Ok(())
    }
}
