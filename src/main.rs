mod config;
mod server;
mod tls;
mod pages;
mod cache;
mod util;

use anyhow::Result;
use tracing_subscriber;
use config::Settings;
use tokio;

// Main function
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // Initialize logging (tracing)
    tracing_subscriber::fmt::init();

    // Load configuration
    let settings = Settings::new()?;
    tracing::info!("Loaded settings: {:?}", settings);

    // Start the Gemini server
    server::run_server(settings).await
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;  // Import items from the outer module
    use tokio;     // Import the tokio runtime
    
    // Test configuration loading functionality
    #[tokio::test]
    async fn test_settings_loading() {
        let settings = Settings::new();
        assert!(settings.is_ok(), "Configuration loading should succeed");
    }

    // Note: Since `tracing_subscriber` initialization doesn't return a value, we can't directly test the log initialization.
    #[tokio::test]
    async fn test_server_run() {
        let settings = Settings::new().unwrap(); // Assuming that loading the configuration is successful
        let result = server::run_server(settings).await;
        assert!(result.is_ok(), "The server should run without errors");
    }
}
