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
    CompilationFailed {
        /// Compiler error output
        message: String,
    },

    /// Test execution failed
    #[error("Test execution failed: {message}")]
    TestExecutionFailed {
        /// Description of the test execution failure
        message: String,
    },

    /// Source directory not found
    #[error("Source directory not found: {path}")]
    SourceDirNotFound {
        /// Path to the missing source directory
        path: String,
    },

    /// Test framework could not be detected
    #[error("Test framework not detected. Available: {available}")]
    TestFrameworkNotDetected {
        /// Comma-separated list of frameworks that were checked
        available: String,
    },

    /// Invalid dependency format
    #[error("Invalid dependency format: {input}")]
    InvalidDependency {
        /// The malformed dependency string
        input: String,
    },

    /// Maven error
    #[error("Maven error: {0}")]
    Maven(#[from] polytunnel_maven::MavenError),

    /// Resolver error
    #[error("Dependency resolution error: {0}")]
    Resolver(#[from] polytunnel_resolver::ResolverError),

    /// Core error
    #[error("Core error: {0}")]
    Core(#[from] polytunnel_core::CoreError),
}
