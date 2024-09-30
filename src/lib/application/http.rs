use anyhow::Context;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use handlers::{
    create_refresh_token::create_refresh_token, get_refresh_token::get_refresh_token,
    sync_filament::sync_filament,
};
use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};

use crate::domain::{
    ams::ports::ams_service::AmsService, token::ports::refresh_token::RefreshTokenService,
};

mod handlers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
struct AppState<RefreshToken: RefreshTokenService, Ams: AmsService> {
    refresh_token_service: Arc<RefreshToken>,
    ams_service: Arc<Ams>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<'a, RefreshToken, Ams>(
        refresh_token_service: Arc<RefreshToken>,
        ams_service: Arc<Ams>,
        config: HttpServerConfig<'a>,
    ) -> anyhow::Result<Self>
    where
        RefreshToken: RefreshTokenService + Send + Sync + 'a,
        Ams: AmsService + Send + Sync + 'a,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            refresh_token_service: Arc::clone(&refresh_token_service),
            ams_service: Arc::clone(&ams_service),
        };

        let router = axum::Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
            .layer(Extension(Arc::clone(&state.ams_service)))
            .layer(Extension(Arc::clone(&state.refresh_token_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running http server")?;

        Ok(())
    }
}

fn api_routes<RefreshToken, A>() -> Router<AppState<RefreshToken, A>>
where
    RefreshToken: RefreshTokenService + Send + Sync + 'static,
    A: AmsService + Send + Sync + 'static,
{
    Router::new()
        .route("/tokens", post(create_refresh_token::<RefreshToken>))
        .route("/tokens/:token_id", get(get_refresh_token::<RefreshToken>))
        .route("/ams", post(sync_filament::<A>))
}
