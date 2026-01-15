//! Error types for polytunnel-core

use thiserror::Error;

/// Result type alias using AppError
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Config not found: {path}")]
    ConfigNotFound { path: String },

    #[error("Invalid dependency format: {input}")]
    InvalidDependency { input: String },

    #[error("Dependency not found: {group_id}:{artifact_id}")]
    DependencyNotFound {
        group_id: String,
        artifact_id: String,
    },

    #[error("Compilation failed: {message}")]
    CompilationFailed { message: String },

    #[error("Test execution failed: {message}")]
    TestExecutionFailed { message: String },

    #[error("Java compiler not found in PATH")]
    JavacNotFound,

    #[error("Test framework not detected. Available: {available}")]
    TestFrameworkNotDetected { available: String },

    #[error("Source directory not found: {path}")]
    SourceDirNotFound { path: String },
}
