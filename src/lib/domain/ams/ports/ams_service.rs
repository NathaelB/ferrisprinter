use std::future::Future;

use crate::domain::ams::models::AmsError;

pub trait AmsService: Send + Sync {
  fn refresh_rfid(&self, tray_id: String) -> impl Future<Output = Result<(), AmsError>> + Send;
}