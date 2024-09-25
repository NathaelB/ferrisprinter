use std::sync::Arc;
use time::OffsetDateTime;
use tracing::info;

use crate::domain::token::models::refresh_token::FindRefreshTokenError;
use crate::{
    domain::token::{
        models::{
            refresh_token::{CreateRefreshTokenError, RefreshToken, RefreshTokenRow},
            token::{SerialNumber, Token},
        },
        ports::refresh_token::RefreshTokenRepository,
    },
    infrastructure::db::postgres::Postgres,
};

#[derive(Debug, Clone)]
pub struct PostgresRefreshTokenRepository {
    postgres: Arc<Postgres>,
}

impl PostgresRefreshTokenRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn create_refresh_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> Result<RefreshToken, CreateRefreshTokenError> {
        let uuid: uuid::Uuid = uuid::Uuid::new_v4();

        let refresh_token = RefreshToken::new(
            uuid,
            SerialNumber::new(serial_number).unwrap(),
            Token::new(token).unwrap(),
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
        );

        sqlx::query_as!(
            RefreshToken,
            r#"INSERT INTO refresh_tokens (id, serial_number, token, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"#,
            refresh_token.id,
            refresh_token.serial_number.as_str(),
            refresh_token.token.as_str(),
            refresh_token.created_at,
            refresh_token.updated_at,
        ).execute(&*self.postgres.get_pool())
        .await?;

        info!("Creation of a refresh token for the next serial_number: {}", serial_number);

        Ok(refresh_token)
    }

    async fn find_by_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<RefreshToken, FindRefreshTokenError> {
        let row = sqlx::query_as!(
            RefreshTokenRow,
            r#"SELECT id, serial_number, token, created_at, updated_at FROM refresh_tokens WHERE serial_number=$1"#,
            serial_number,
        ).fetch_one(&*self.postgres.get_pool()).await
            .map_err(|_| FindRefreshTokenError::NotFound { serial_number: SerialNumber::new(serial_number).unwrap() });

        let row = row.unwrap();

        let refresh_token = RefreshToken::new(
            row.id,
            SerialNumber::new(&row.serial_number).unwrap(),
            Token::new(&row.token).unwrap(),
            row.created_at,
            row.updated_at,
        );

        Ok(refresh_token)
    }
}
