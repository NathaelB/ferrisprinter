use std::sync::Arc;

use time::OffsetDateTime;
use tracing::info;
use uuid::Uuid;

use crate::{
    domain::token::{
        models::{
            access_token::{AccessToken, AccessTokenError, AccessTokenRow},
            token::{SerialNumber, Token},
        },
        ports::access_token::AccessTokenRepository,
    },
    infrastructure::db::postgres::Postgres,
};

#[derive(Debug, Clone)]
pub struct PostgresAccessTokenRepository {
    postgres: Arc<Postgres>,
}

impl PostgresAccessTokenRepository {
    pub fn new(postgres: Arc<Postgres>) -> Self {
        Self { postgres }
    }
}

impl AccessTokenRepository for PostgresAccessTokenRepository {
    async fn create_access_token(
        &self,
        token: &str,
        serial_number: &str,
    ) -> Result<AccessToken, AccessTokenError> {
        let uuid: Uuid = Uuid::new_v4();

        let access_token = AccessToken::new(
            uuid,
            SerialNumber::new(serial_number).unwrap(),
            Token::new(token).unwrap(),
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc(),
        );

        sqlx::query_as!(
          AccessToken,
          r#"INSERT INTO access_tokens (id, serial_number, token, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"#,
          access_token.id,
          access_token.serial_number.as_str(),
          access_token.token.as_str(),
          access_token.created_at,
          access_token.updated_at,
      ).execute(&*self.postgres.get_pool())
      .await.map_err(|_| AccessTokenError::CreateError)?;

        info!(
            "Creation of a access token for the next serial_number: {}",
            serial_number
        );

        Ok(access_token)
    }

    async fn find_by_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<AccessToken, AccessTokenError> {
        let row = sqlx::query_as!(
            AccessTokenRow,
            r#"SELECT id, serial_number, token, created_at, updated_at FROM access_tokens WHERE serial_number = $1"#,
            serial_number
        )
        .fetch_one(&*self.postgres.get_pool())
        .await
        .map_err(|_| AccessTokenError::NotFound {
            serial_number: SerialNumber::new(serial_number).unwrap(),
        });

        let row = row.unwrap();

        let access_token = AccessToken::new(
            row.id,
            SerialNumber::new(row.serial_number.as_str()).unwrap(),
            Token::new(row.token.as_str()).unwrap(),
            row.created_at,
            row.updated_at,
        );

        Ok(access_token)
    }

    async fn delete_by_serial_number(&self, serial_number: &str) -> Result<(), AccessTokenError> {
        sqlx::query!(
            r#"DELETE FROM access_tokens WHERE serial_number = $1"#,
            serial_number
        )
        .execute(&*self.postgres.get_pool())
        .await
        .map_err(|e| AccessTokenError::DatabaseError(e))?;

        Ok(())
    }
}
