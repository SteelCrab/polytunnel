//! Build system for Polytunnel
//!
//! Provides compilation and test execution for Java projects using javac.
//! Supports JUnit 5, JUnit 4, and TestNG test frameworks with automatic detection.

mod classpath;
mod compiler;
pub mod error;
mod incremental;
mod orchestrator;
mod test_runner;

// Re-exports for convenience
pub use classpath::{ClasspathBuilder, ClasspathResult};
pub use compiler::{CompilationResult, JavaCompiler};
pub use error::{BuildError, Result};
pub use incremental::{BuildCache, BuildCacheEntry};
pub use orchestrator::{BuildOptions, BuildOrchestrator, BuildResult, TestOptions};
pub use test_runner::{TestFramework, TestResult, TestRunner};

/// Format classpath paths with OS-specific separator.
///
/// Uses `;` on Windows and `:` on Unix-like systems.
pub fn format_classpath(paths: &[std::path::PathBuf]) -> String {
    let separator = if cfg!(windows) { ";" } else { ":" };
    paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join(separator)
}
