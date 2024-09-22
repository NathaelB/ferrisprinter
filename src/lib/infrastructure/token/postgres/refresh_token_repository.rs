use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    domain::token::{
        models::{
            refresh_token::{CreateRefreshTokenError, RefreshToken},
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
            chrono::Utc::now(),
            chrono::Utc::now(),
        );

        let created_at = chrono_to_offset(refresh_token.created_at);
        let updated_at = chrono_to_offset(refresh_token.updated_at);

        let _result = sqlx::query_as!(
            RefreshToken,
            r#"INSERT INTO refresh_tokens (id, serial_number, token, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"#,
            refresh_token.id,
            refresh_token.serial_number.as_str(),
            refresh_token.token.as_str(),
            created_at,
            updated_at,
        ).execute(&*self.postgres.get_pool())
        .await
        .map_err(|_| CreateRefreshTokenError::DatabaseError);

        Ok(refresh_token)
    }
}

fn chrono_to_offset(dt: chrono::DateTime<chrono::Utc>) -> OffsetDateTime {
    let naive = dt.naive_utc();
    OffsetDateTime::from_unix_timestamp(naive.and_utc().timestamp()).unwrap()
}
