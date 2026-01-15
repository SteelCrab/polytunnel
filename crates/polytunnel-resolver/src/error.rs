//! Error types for polytunnel-resolver
//!
//! Resolver-specific errors for dependency resolution.

use thiserror::Error;

/// Result type alias for resolver operations
pub type Result<T> = std::result::Result<T, ResolverError>;

/// Resolver-specific errors
#[derive(Debug, Error)]
pub enum ResolverError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Maven operation failed
    #[error("Maven error: {0}")]
    Maven(#[from] polytunnel_maven::MavenError),

    /// Circular dependency detected
    #[error("Circular dependency detected: {path}")]
    CircularDependency { path: String },

    /// Dependency not found
    #[error("Dependency not found: {coordinate}")]
    DependencyNotFound { coordinate: String },

    /// Version conflict
    #[error("Version conflict for {artifact}: {versions:?}")]
    VersionConflict {
        artifact: String,
        versions: Vec<String>,
    },

    /// Core configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] polytunnel_core::CoreError),
}
