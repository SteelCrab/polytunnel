//! VS Code IDE integration for Polytunnel
//!
//! Generates Eclipse/VS Code project files (`.project`, `.classpath`) and
//! VS Code workspace settings from a resolved Polytunnel project.

#![warn(missing_docs)]

mod error;
pub mod vscode;

pub use error::{IdeError, Result};
