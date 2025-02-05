use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub address: String,
    pub cert_path: String,
    pub key_path: String,
    pub pages_dir: String,
    pub tls_reload_interval_secs: u64,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("GEMINI").separator("_"))
            .build()?;
        config.try_deserialize::<Settings>().map_err(|e| e.into())
    }
}
