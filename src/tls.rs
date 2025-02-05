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
