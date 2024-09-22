use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::domain::token::models::refresh_token::CreateRefreshTokenRequest;
use crate::{
    application::http::AppState,
    domain::token::{
        models::{
            refresh_token::{CreateRefreshTokenError, RefreshToken},
            token::{SerialNumber, SerialNumberEmptyError, Token, TokenEmptyError},
        },
        ports::refresh_token::RefreshTokenService,
    },
};

pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<CreateRefreshTokenError> for ApiError {
    fn from(e: CreateRefreshTokenError) -> Self {
        match e {
            CreateRefreshTokenError::Duplicate { name } => Self::UnprocessableEntity(format!(
                "Refresh token with serial number {} already exists",
                name
            )),
            CreateRefreshTokenError::Unknown(cause) => {
                error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
            CreateRefreshTokenError::DatabaseError(cause) => {
                error!("{:?}", cause);
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateRefreshTokenHttpRequestBodyError> for ApiError {
    fn from(e: ParseCreateRefreshTokenHttpRequestBodyError) -> Self {
        let message = match e {
            ParseCreateRefreshTokenHttpRequestBodyError::SerialNumber(e) => e.to_string(),
            ParseCreateRefreshTokenHttpRequestBodyError::Token(e) => e.to_string(),
        };

        Self::UnprocessableEntity(message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateRefreshTokenRequestBody {
    token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateRefreshTokenResponseData {
    id: String,
}

impl From<&RefreshToken> for CreateRefreshTokenResponseData {
    fn from(refresh_token: &RefreshToken) -> Self {
        Self {
            id: refresh_token.id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateRefreshTokenHttpRequestBody {
    token: String,
    serial_number: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateRefreshTokenHttpRequestBodyError {
    #[error(transparent)]
    SerialNumber(#[from] SerialNumberEmptyError),
    #[error(transparent)]
    Token(#[from] TokenEmptyError),
}

impl CreateRefreshTokenHttpRequestBody {
    fn try_into_domain(
        self,
    ) -> Result<CreateRefreshTokenRequest, ParseCreateRefreshTokenHttpRequestBodyError> {
        let token = Token::new(&self.token)?;
        let serial_number = SerialNumber::new(&self.serial_number)?;

        Ok(CreateRefreshTokenRequest::new(token, serial_number))
    }
}

pub async fn create_refresh_token<R: RefreshTokenService>(
    State(state): State<AppState<R>>,
    Json(body): Json<CreateRefreshTokenHttpRequestBody>,
) -> Result<ApiSuccess<CreateRefreshTokenResponseData>, ApiError> {
    let domain_request = body.try_into_domain()?;

    state
        .refresh_token_service
        .create_refresh_token(
            domain_request.token().as_str(),
            domain_request.serial_number().as_str(),
        )
        .await
        .map_err(ApiError::from)
        .map(|ref refresh_token| ApiSuccess::new(StatusCode::CREATED, refresh_token.into()))
}
