use std::future::Future;

use crate::domain::{ams::models::AmsError, token::ports::provider_token_service::ProviderType};

pub trait AmsService: Send + Sync + Clone + 'static {
    fn refresh_rfid(
        &self,
        tray_id: String,
        provider: ProviderType,
        device_id: String,
    ) -> impl Future<Output = Result<(), AmsError>> + Send;
}
