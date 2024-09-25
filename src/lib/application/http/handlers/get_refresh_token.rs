use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::{
    application::http::AppState, domain::token::ports::refresh_token::RefreshTokenService,
};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRefreshTokenResponseData {
    pub refresh_token: String,
}

pub async fn get_refresh_token<R: RefreshTokenService>(
    State(state): State<AppState<R>>,
    Path(token_id): Path<String>,
) -> Result<ApiSuccess<GetRefreshTokenResponseData>, ApiError> {
    let refresh_token = state
        .refresh_token_service
        .find_by_serial_number(&token_id)
        .await;

    match refresh_token {
        Ok(refresh_token) => {
            let response_data = GetRefreshTokenResponseData {
                refresh_token: refresh_token.token.as_str().to_string(),
            };

            Ok(ApiSuccess::new(StatusCode::ACCEPTED, response_data))
        }
        Err(_) => Err(ApiError::InternalServerError(
            "Internal Server Error".to_string(),
        )),
    }
}
