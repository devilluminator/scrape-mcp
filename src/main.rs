use scrape_mcp::{config::Config, serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve::run(Config::from_env()).await
}
