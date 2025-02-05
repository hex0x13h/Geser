mod config;
mod server;
mod tls;
mod pages;
mod cache;
mod util;

use anyhow::Result;
use tracing_subscriber;
use config::Settings;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // 初始化日志（tracing）
    tracing_subscriber::fmt::init();

    // Loading Configuration
    let settings = Settings::new()?;
    tracing::info!("Loaded settings: {:?}", settings);

    // Start the Gemini server
    server::run_server(settings).await
}
