//! Error types for polytunnel-maven
//!
//! Maven-specific errors for repository access, POM parsing, and artifact resolution.

use thiserror::Error;

/// Result type alias for Maven operations
pub type Result<T> = std::result::Result<T, MavenError>;

/// Maven-specific errors
#[derive(Debug, Error)]
pub enum MavenError {
    /// IO error during Maven operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// HTTP status error
    #[error("HTTP status {status} for {url}")]
    HttpStatus {
        /// HTTP status code received
        status: u16,
        /// URL that returned the error status
        url: String,
    },

    /// Failed to parse JSON response
    #[error("JSON parse error: {message}")]
    JsonParse {
        /// Description of the JSON parse error
        message: String,
    },

    /// Response body is not valid UTF-8 text
    #[error("Invalid UTF-8 response: {message}")]
    InvalidUtf8 {
        /// Description of the encoding error
        message: String,
    },

    /// XML parsing error
    #[error("XML parse error: {message}")]
    XmlParse {
        /// Description of the XML parse error
        message: String,
    },

    /// Invalid Maven coordinate format
    #[error("Invalid coordinate format: {input}")]
    InvalidCoordinate {
        /// The malformed coordinate string
        input: String,
    },

    /// Artifact not found in repositories
    #[error("Artifact not found: {coordinate}")]
    ArtifactNotFound {
        /// Coordinate of the missing artifact
        coordinate: String,
    },

    /// POM file not found
    #[error("POM not found: {path}")]
    PomNotFound {
        /// Repository path where the POM was expected
        path: String,
    },

    /// Core configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] polytunnel_core::CoreError),
}
