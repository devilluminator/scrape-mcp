use std::sync::Arc;

use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

use crate::config::Config;
use crate::server::ScraperServer;

pub fn router(config: Arc<Config>) -> Router {
    let factory_cfg = Arc::clone(&config);
    let service = StreamableHttpService::new(
        move || -> std::io::Result<ScraperServer> {
            Ok(ScraperServer::new(Arc::clone(&factory_cfg)))
        },
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );

    Router::new().nest_service(&config.mount, service)
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let config = Arc::new(config);
    let app = router(Arc::clone(&config));
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;

    println!(
        "MCP server listening on http://{}{}",
        config.bind, config.mount
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
