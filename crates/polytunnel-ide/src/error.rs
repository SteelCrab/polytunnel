//! Error types for polytunnel-ide

use thiserror::Error;

/// Result type alias for IDE integration operations
pub type Result<T> = std::result::Result<T, IdeError>;

/// IDE integration errors
#[derive(Debug, Error)]
pub enum IdeError {
    /// Build system error (e.g. javac not found, compilation failure)
    #[error("Build error: {0}")]
    Build(#[from] polytunnel_build::BuildError),

    /// Core configuration error
    #[error("Core error: {0}")]
    Core(#[from] polytunnel_core::CoreError),

    /// IO error while reading or writing IDE configuration files
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
