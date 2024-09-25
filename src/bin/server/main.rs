use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use ferrisprinter::{
    application::{http::{HttpServer, HttpServerConfig}, providers::token_provider_manager::{self, TokenProviderManager}},
    domain::token::{
        ports::provider_token_service::ProviderType, service::RefreshTokenServiceImpl
    },
    env::Env,
    infrastructure::{
        db::postgres::Postgres,
        token::{
            postgres::refresh_token_repository::PostgresRefreshTokenRepository,
            providers::bambulab_provider::BambuLabProviderTokenService,
        },
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
    let mut token_provider_manager = TokenProviderManager::new();
    let bambulab_provider = BambuLabProviderTokenService::new(
        "".to_string(),
        "https://bambulab.com/api/sign-in/form".to_string(),
    );

    token_provider_manager.register_provider(ProviderType::BambuLab, bambulab_provider);
    let token_provider_manager = Arc::new(token_provider_manager);

    let refresh_token_repository = PostgresRefreshTokenRepository::new(Arc::clone(&postgres));

    let refresh_token_service = RefreshTokenServiceImpl::new(refresh_token_repository, Arc::clone(&token_provider_manager));

    let http_server = HttpServer::new(Arc::new(refresh_token_service), server_config).await?;

    http_server.run().await
}
