use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use ferrisprinter::{
    application::http::{HttpServer, HttpServerConfig},
    domain::token::service::RefreshTokenServiceImpl,
    env::Env,
    infrastructure::{
        db::postgres::Postgres,
        token::postgres::refresh_token_repository::PostgresRefreshTokenRepository,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let postgres = Postgres::new(Arc::clone(&env)).await?;

    let postgres = Arc::new(postgres);
    let server_config = HttpServerConfig { port: &env.port };

    let refresh_token_repository = PostgresRefreshTokenRepository::new(Arc::clone(&postgres));

    let refresh_token_service = RefreshTokenServiceImpl::new(refresh_token_repository);

    let http_server = HttpServer::new(Arc::new(refresh_token_service), server_config).await?;

    http_server.run().await
}
