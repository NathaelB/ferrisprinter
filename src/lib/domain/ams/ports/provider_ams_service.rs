use std::future::Future;

use crate::domain::ams::models::AmsError;

pub trait ProviderAmsService: Send + Sync + Clone {
    fn refresh_rfid(
        &self,
        tray_id: String,
        device_id: String,
        username: String,
        password: String,
    ) -> impl Future<Output = Result<(), AmsError>> + Send;
}
