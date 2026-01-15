//! Error types for polytunnel-build
//!
//! Build-specific errors for compilation, testing, and source management.

use thiserror::Error;

/// Result type alias for build operations
pub type Result<T> = std::result::Result<T, BuildError>;

/// Build-specific errors
#[derive(Debug, Error)]
pub enum BuildError {
    /// IO error during build operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Java compiler (javac) not found in PATH
    #[error("Java compiler (javac) not found in PATH")]
    JavacNotFound,

    /// Compilation failed with error message
    #[error("Compilation failed: {message}")]
    CompilationFailed { message: String },

    /// Test execution failed
    #[error("Test execution failed: {message}")]
    TestExecutionFailed { message: String },

    /// Source directory not found
    #[error("Source directory not found: {path}")]
    SourceDirNotFound { path: String },

    /// Test framework could not be detected
    #[error("Test framework not detected. Available: {available}")]
    TestFrameworkNotDetected { available: String },

    /// Invalid dependency format
    #[error("Invalid dependency format: {input}")]
    InvalidDependency { input: String },

    /// Core configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] polytunnel_core::CoreError),
}
