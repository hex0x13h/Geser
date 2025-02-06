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
    // Creates a new Settings instance by loading configuration from a file and environment variables
    pub fn new() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(false))  // Optionally load config from "config" file
            .add_source(config::Environment::with_prefix("GEMINI").separator("_")) // Load configuration from environment variables with "GEMINI" prefix
            .build()?;
        
        config.try_deserialize::<Settings>().map_err(|e| e.into()) // Deserialize config into Settings struct
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;  // Import Settings struct from outer scope
    use std::env;

    // Test loading configuration from file and environment variables
    #[test]
    fn test_settings_loading() {
        // Set environment variables to simulate the config loading
        env::set_var("GEMINI_ADDRESS", "0.0.0.0:1965");
        env::set_var("GEMINI_CERT_PATH", "cert.pem");
        env::set_var("GEMINI_KEY_PATH", "key.pem");
        env::set_var("GEMINI_PAGES_DIR", "pages");
        env::set_var("GEMINI_TLS_RELOAD_INTERVAL_SECS", "300");

        // Load settings
        let settings = Settings::new().unwrap();

        // Check if the environment variables were correctly loaded
        assert_eq!(settings.address, "0.0.0.0:1965");
        assert_eq!(settings.cert_path, "cert.pem");
        assert_eq!(settings.key_path, "key.pem");
        assert_eq!(settings.pages_dir, "pages");
        assert_eq!(settings.tls_reload_interval_secs, 300);
    }

    // Test loading settings from file (if the file exists)
    #[test]
    fn test_settings_from_file() {
        // Assume there is a config file named "config.toml" with relevant settings.
        // You would have to set up a mock config file for this test to be meaningful.
        // This test is for illustration purposes only and may not pass unless you have a file.

        // Normally you would load a config file like this:
        // let settings = Settings::new().unwrap();

        // Check if settings were loaded correctly from the config file
        // You can test individual values here
    }
}
