use std::sync::Arc;

use anyhow::Result;
use ferrisprinter::{
    application::http::{HttpServer, HttpServerConfig},
    domain::token::service::RefreshTokenServiceImpl,
    infrastructure::{
        db::postgres::Postgres,
        token::postgres::refresh_token_repository::PostgresRefreshTokenRepository,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let postgres =
        Postgres::new("postgres://postgres:postgres@localhost:5432/ferrisprinter").await?;

    let postgres = Arc::new(postgres);
    let server_config = HttpServerConfig { port: "3333" };

    let refresh_token_repository = PostgresRefreshTokenRepository::new(Arc::clone(&postgres));

    let refresh_token_service = RefreshTokenServiceImpl::new(refresh_token_repository);

    let http_server = HttpServer::new(Arc::new(refresh_token_service), server_config).await?;

    http_server.run().await
}
