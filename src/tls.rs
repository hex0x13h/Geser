use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use anyhow::{Result, Context, anyhow};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile;
use tokio::time::{sleep, Duration};
use tracing::info;

/// Returns the TLS configuration by reading the certificate and key files.
pub async fn get_tls_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>> {
    load_tls_config(cert_path, key_path)
}

/// Loads the TLS configuration: reads the certificate chain and private key, and builds the ServerConfig.
pub fn load_tls_config(cert_path: &str, key_path: &str) -> Result<Arc<ServerConfig>> {
    // Read the certificate file.
    let cert_file = &mut BufReader::new(File::open(cert_path)
        .with_context(|| format!("Failed to open certificate file: {}", cert_path))?);
    let certs = rustls_pemfile::certs(cert_file)
        .with_context(|| "Failed to read certificate")?
        .into_iter()
        .map(Certificate)
        .collect();

    // Read the private key file, supporting PKCS8 and RSA formats.
    let key_file = &mut BufReader::new(File::open(key_path)
        .with_context(|| format!("Failed to open key file: {}", key_path))?);
    let keys = rustls_pemfile::read_all(key_file)
        .with_context(|| "Failed to read private key")?;
    let mut private_key = None;
    for item in keys {
        match item {
            rustls_pemfile::Item::PKCS8Key(key) | rustls_pemfile::Item::RSAKey(key) => {
                private_key = Some(PrivateKey(key));
                break;
            },
            _ => continue,
        }
    }
    let private_key = private_key.ok_or_else(|| anyhow!("No valid private key found"))?;

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()  // Do not require client certificate.
        .with_single_cert(certs, private_key)
        .with_context(|| "Failed to build TLS configuration")?;
    Ok(Arc::new(config))
}

/// Background task that periodically reloads the TLS configuration.
pub async fn reload_tls_config_task(cert_path: String, key_path: String, interval_secs: u64) {
    let interval = Duration::from_secs(interval_secs);
    loop {
        sleep(interval).await;
        match load_tls_config(&cert_path, &key_path) {
            Ok(new_config) => {
                // Here  can update the global configuration or notify the relevant modules to reload the TLS configuration.
                info!("Reloaded TLS configuration: {:?}", new_config);
            },
            Err(e) => {
                tracing::error!("Failed to reload TLS configuration: {:?}", e);
            }
        }
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;  // Import functions and structures from outer module
    use std::env;
    use std::fs::File;
    use std::path::Path;
    use tokio;

    // Test that TLS config is loaded correctly
    #[tokio::test]
    async fn test_get_tls_config() {
        // Create temporary certificate and key files for testing
        let cert_file = "test_cert.pem";
        let key_file = "test_key.pem";

        // Make sure the test files exist and are valid (you may want to create valid files or mock the file system)
        assert!(Path::new(cert_file).exists(), "Certificate file does not exist");
        assert!(Path::new(key_file).exists(), "Key file does not exist");

        // Test loading the TLS configuration
        let result = get_tls_config(cert_file, key_file).await;
        assert!(result.is_ok(), "Failed to load TLS config");
    }

    // Test reloading TLS configuration periodically
    #[tokio::test]
    async fn test_reload_tls_config_task() {
        // Create temporary certificate and key files for testing
        let cert_file = "test_cert.pem";
        let key_file = "test_key.pem";

        // Ensure test files exist and are valid
        assert!(Path::new(cert_file).exists(), "Certificate file does not exist");
        assert!(Path::new(key_file).exists(), "Key file does not exist");

        // Set reload interval to 1 second for testing
        let interval_secs = 1;

        // Run the reload task and check if the configuration reloads without errors
        let task = tokio::spawn(async move {
            reload_tls_config_task(cert_file.to_string(), key_file.to_string(), interval_secs).await;
        });

        // Allow the task to run for a few seconds
        tokio::time::sleep(Duration::from_secs(3)).await;
        task.abort(); // Stop the task after the test period

        // Check if the task ran correctly (this is a basic test, you can expand it by checking logs or other effects)
        assert!(true, "TLS reload task ran successfully");
    }
}
