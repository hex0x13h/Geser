use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as AsyncBufReader};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use anyhow::{Result, anyhow};
use url::Url;

use crate::tls::{get_tls_config, reload_tls_config_task};
use crate::pages;
use crate::config::Settings;
use crate::cache::Cache;
use crate::util::sanitize_path;


/// Starts the Gemini Server, binds to the listening address, and handles incoming connections.
pub async fn run_server(settings: Settings) -> Result<()> {
    // Start the TLS hot reload task (periodically reload certificates)
    let tls_reload_interval = settings.tls_reload_interval_secs;
    tokio::spawn(reload_tls_config_task(
        settings.cert_path.clone(),
        settings.key_path.clone(),
        tls_reload_interval,
    ));

    // Get the initial TLS configuration
    let tls_config = get_tls_config(&settings.cert_path, &settings.key_path).await?;
    let acceptor = TlsAcceptor::from(tls_config);

    // Bind listening address
    let listener = TcpListener::bind(&settings.address).await
        .map_err(|e| anyhow!("Failed to bind to address {}: {:?}", settings.address, e))?;
    tracing::info!("Gemini Server started, listening on: {}", settings.address);

    // Create a global cache (for static files and Markdown pages)
    let cache = Cache::new();

    loop {
        let (stream, peer) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let pages_dir = settings.pages_dir.clone();
        let cache = cache.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(acceptor, stream, peer, pages_dir, cache).await {
                tracing::error!("Error handling connection {}: {:?}", peer, e);
            }
        });
    }
}

/// Handles a single connection: performs TLS handshake, reads the request line,
/// sanitizes the requested path, and returns either a Markdown page or a static file.
async fn handle_connection(
    acceptor: TlsAcceptor,
    stream: tokio::net::TcpStream,
    peer: SocketAddr,
    pages_dir: String,
    cache: Cache,
) -> Result<()> {
    tracing::info!("Handling connection from {}", peer);
    let tls_stream = acceptor.accept(stream).await
        .map_err(|e| anyhow!("TLS handshake with {} failed: {:?}", peer, e))?;

    let (reader, mut writer) = tokio::io::split(tls_stream);
    let mut buf_reader = AsyncBufReader::new(reader);
    let mut request_line = String::new();

    let bytes_read = buf_reader.read_line(&mut request_line).await?;
    if bytes_read == 0 {
        tracing::info!("Connection {} closed", peer);
        return Ok(());
    }
    tracing::info!("Received request from {}: {}", peer, request_line.trim_end());

    let req_line = request_line.trim();
    let req_url = Url::parse(req_line)
        .map_err(|e| anyhow!("Failed to parse URL {}: {:?}", req_line, e))?;
    let path = req_url.path();

    // Perform security checks on URL paths to prevent directory traversal
    let safe_path = sanitize_path(path)?;

    if safe_path.ends_with(".jpg") || safe_path.ends_with(".jpeg") ||
       safe_path.ends_with(".png") || safe_path.ends_with(".gif") {
        // Static image resource request
        match pages::serve_static_file(&pages_dir, &safe_path, cache).await {
            Ok((data, mime)) => {
                let header = format!("20 {}\r\n", mime);
                writer.write_all(header.as_bytes()).await?;
                writer.write_all(&data).await?;
            },
            Err(e) => {
                tracing::error!("Error serving static resource {}: {:?}", safe_path, e);
                writer.write_all(b"51 Not Found\r\n").await?;
            }
        }
    } else {
        // Markdown page request
        match pages::serve_markdown(&pages_dir, &safe_path, cache).await {
            Ok(content) => {
                writer.write_all(b"20 text/gemini\r\n").await?;
                writer.write_all(content.as_bytes()).await?;
            },
            Err(e) => {
                tracing::error!("Error serving page {}: {:?}", safe_path, e);
                writer.write_all(b"51 Not Found\r\n").await?;
            }
        }
    }
    writer.flush().await?;
    Ok(())
}
