//! Core error types for polytunnel
//!
//! Configuration and basic IO errors used across all crates.

use thiserror::Error;

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core errors for configuration and basic IO
#[derive(Debug, Error)]
pub enum CoreError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parse error
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// TOML serialize error
    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// Config file not found
    #[error("Config not found: {path}")]
    ConfigNotFound {
        /// Path where the config file was expected
        path: String,
    },

    /// Invalid Maven coordinate format
    #[error("Invalid coordinate: {message}")]
    InvalidCoordinate {
        /// Description of what's wrong with the coordinate
        message: String,
    },

    /// Dependency already exists in config
    #[error("Dependency already exists: {coordinate}")]
    DuplicateDependency {
        /// The duplicate coordinate key
        coordinate: String,
    },

    /// TOML edit error
    #[error("TOML edit error: {0}")]
    TomlEdit(#[from] toml_edit::TomlError),
}
