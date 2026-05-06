//! Configuration error type.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    /// A config file could not be read or parsed.
    #[error("failed to load configuration: {0}")]
    Load(String),

    /// The raw config could not be deserialized into the expected structure.
    ///
    /// Usually means a required field is missing or a value has the wrong type.
    #[error("failed to deserialize configuration: {0}")]
    Deserialize(String),

    /// A field passed deserialization but failed semantic validation.
    ///
    /// Examples: port 0, empty database URL, `max_connections` less than 1.
    #[error("invalid value for '{field}': {reason}")]
    InvalidValue { field: &'static str, reason: String },
}
