use std::fs;
use std::sync::OnceLock;
use std::path::PathBuf;
use serde::Deserialize;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub secretkey: String,
    pub use_ws_for_learn: bool,
    pub use_ws_for_predit: bool,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn load(path: &str) -> Result<(), ConfigError> {
        let data = fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(e.to_string()))?;

        let parsed: Config = toml::from_str(&data)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        CONFIG.set(parsed)
            .map_err(|_| ConfigError::AlreadyInitialized)?;

        Ok(())
    }

    /// Auto-load configuration from various sources with priority:
    /// 1. Provided config_path (from --config flag)
    /// 2. SML_CONFIG_FILE environment variable
    /// 3. Default locations: ./rc.conf, ~/.config/sym-rest-rc/config.toml, ~/.sym-rest-rc/config.toml
    pub fn auto_load(config_path: Option<String>) -> Result<String, ConfigError> {
        // Priority 1: Explicit config path from flag
        if let Some(path) = config_path {
            Self::load(&path)?;
            return Ok(path);
        }

        // Priority 2: Environment variable
        if let Ok(env_path) = std::env::var("SML_CONFIG_FILE") {
            Self::load(&env_path)?;
            return Ok(env_path);
        }

        // Priority 3: Default locations
        let default_paths = Self::get_default_config_paths();
        for path in &default_paths {
            if path.exists() {
                let path_str = path.to_string_lossy().to_string();
                Self::load(&path_str)?;
                return Ok(path_str);
            }
        }

        // No config found
        Err(ConfigError::NotFound(format!(
            "No configuration file found. Tried:\n  \
            - --config flag\n  \
            - SML_CONFIG_FILE environment variable\n  \
            - Default locations: {}",
            default_paths.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )))
    }

    /// Get list of default configuration file paths
    fn get_default_config_paths() -> Vec<PathBuf> {
        let mut paths = vec![
            PathBuf::from("./rc.conf"),
        ];

        // Add home directory paths
        if let Some(home) = std::env::var_os("HOME") {
            let home_path = PathBuf::from(home);
            paths.push(home_path.join(".config/sym-rest-rc/config.toml"));
            paths.push(home_path.join(".sym-rest-rc/config.toml"));
        }

        paths
    }

    /// Get the host from the loaded configuration
    pub fn host() -> &'static str {
        &Self::get().host
    }

    /// Get the port from the loaded configuration
    pub fn port() -> u16 {
        Self::get().port
    }

    /// Get the user from the loaded configuration
    pub fn user() -> &'static str {
        &Self::get().user
    }

    /// Get the secret key from the loaded configuration
    /// Checks SML_SK environment variable first, falls back to config file
    pub fn secretkey() -> String {
        std::env::var("SML_SK")
            .unwrap_or_else(|_| Self::get().secretkey.clone())
    }

    /// Check if WebSocket should be used for learn operations
    pub fn use_ws_for_learn() -> bool {
        Self::get().use_ws_for_learn
    }

    /// Check if WebSocket should be used for predict operations
    pub fn use_ws_for_predit() -> bool {
        Self::get().use_ws_for_predit
    }

    /// Get reference to the loaded configuration
    fn get() -> &'static Config {
        CONFIG.get().expect("Config not loaded")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(String),

    #[error("Failed to parse config: {0}")]
    ParseError(String),

    #[error("Config already initialized")]
    AlreadyInitialized,

    #[error("{0}")]
    NotFound(String),
}
