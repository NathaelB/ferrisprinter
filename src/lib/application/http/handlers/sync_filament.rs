use std::sync::Arc;

use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::domain::{
    ams::ports::ams_service::AmsService, token::ports::provider_token_service::ProviderType,
};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SyncFilamentHttpRequestBody {
    tray_id: String,
    device_id: String,
}

pub async fn sync_filament<A: AmsService>(
    Extension(ams_service): Extension<Arc<A>>,
    Json(body): Json<SyncFilamentHttpRequestBody>,
) -> Result<ApiSuccess<String>, ApiError> {
    let tray_id = body.tray_id;
    let device_id = body.device_id;

    let _ = ams_service
        .refresh_rfid(tray_id, ProviderType::BambuLab, device_id)
        .await;

    Ok(ApiSuccess::new(StatusCode::ACCEPTED, "Success".to_string()))
}
