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
    ConfigNotFound { path: String },
}

// Keep AppError as alias for backward compatibility during migration
#[deprecated(note = "Use crate-specific error types instead")]
pub type AppError = CoreError;
