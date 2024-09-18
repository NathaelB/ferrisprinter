use std::sync::Arc;

use ferrisprinter::{application::context::AppContext, interface::http_server::HttpServeApp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let context = Arc::new(AppContext::new());

    let server = HttpServeApp::new(context, "127.0.0.1:8888");

    server.run().await?;

    Ok(())
}
