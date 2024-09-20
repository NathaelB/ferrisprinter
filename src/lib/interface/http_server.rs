use std::sync::Arc;

use actix_web::{get, App, HttpServer, Responder};

use crate::application::context::AppContext;

pub struct HttpServeApp {
    pub context: Arc<AppContext>,
    address: String,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

impl HttpServeApp {
    pub fn new(context: Arc<AppContext>, address: &str) -> Self {
        Self {
            context,
            address: address.to_string(),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let _context = Arc::clone(&self.context);

        HttpServer::new(move || App::new().service(index))
            .bind(&self.address)?
            .run()
            .await
    }
}
