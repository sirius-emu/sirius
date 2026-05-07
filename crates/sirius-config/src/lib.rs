//! Configuration loading and validation for Sirius.
//!
//! Configuration is layered: a base file is loaded first, then an
//! environment-specific file is merged on top, and finally environment
//! variables are applied to overrides. The result is validated before the
//! server starts. A missing or malformed config is a hard failure, not something
//! to paper over with defaults at runtime.
//!
//! # Loading order
//!
//! 1. `config/default.toml` - always loaded, must exist.
//! 2. `config/{environment}.toml` - merged on top; missing is fine.
//! 3. `SIRIUS_*` environment variables - highest precedence.
//!
//! # Environment variable mapping
//!
//! Variable names follow the pattern `SIRIUS_{SECTION}_{KEY}`, with double underscores
//! separating nested keys. Examples:
//!
//! ```text
//! SIRIUS_SERVER__PORT=3000
//! SIRIUS_DATABASE__URL=postgres://user:pass@localhost/sirius
//! ```
//!
//! # Usage
//! ```no_run
//! use sirius_config::Config;
//!
//! let config = Config::load("production").expect("failed to load config");
//! println!("listening on port {}", config.server.port);
//! ```

mod database;
mod error;
mod network;
mod server;

pub use database::DatabaseConfig;
pub use error::ConfigError;
pub use network::NetworkConfig;
pub use server::ServerConfig;

use config::{Config as RawConfig, Environment, File, FileFormat};
use serde::Deserialize;

/// The root configuration structure.
///
/// All fields are required unless explicitly marked optional. The server
/// will not start if any required field is missing after all layers are
/// merged and environment variables are applied.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub network: NetworkConfig,
    pub database: DatabaseConfig,
}

impl Config {
    /// Loads and validates configuration for the given environment.
    ///
    /// `environment` is typically `development`, `production` or `test`. It determines
    /// which optional override file is loaded on top of `config/default.toml`.
    ///
    /// Returns an error if any required field is missing, if a value fails validation, or if a file
    /// cannot be parsed.
    pub fn load(environment: &str) -> Result<Self, ConfigError> {
        let raw = RawConfig::builder()
            .add_source(File::new("config/default", FileFormat::Toml))
            .add_source(
                File::new(&format!("config/{environment}"), FileFormat::Toml)
                    .required(false),
            )
            .add_source(
                Environment::with_prefix("SIRIUS")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()
            .map_err(|e| ConfigError::Load(e.to_string()))?;

        let config: Self = raw
            .try_deserialize()
            .map_err(|e| ConfigError::Deserialize(e.to_string()))?;

        config.validate()?;

        Ok(config)
    }

    /// Validates all config sections.
    ///
    /// Called automatically by `load`. Exposed publicly so callers can re-validate
    /// after programmatic modification in tests.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.server.validate()?;
        self.network.validate()?;
        self.database.validate()?;
        Ok(())
    }
}
