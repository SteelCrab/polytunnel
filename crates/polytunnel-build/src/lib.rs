//! Build system for Polytunnel
//!
//! Provides compilation and test execution for Java projects using javac.
//! Supports JUnit 5, JUnit 4, and TestNG test frameworks with automatic detection.

pub mod classpath;
pub mod compiler;
pub mod incremental;
pub mod orchestrator;
pub mod test_runner;

// Re-exports for convenience
pub use classpath::ClasspathBuilder;
pub use compiler::{CompilationResult, JavaCompiler};
pub use incremental::BuildCache;
pub use orchestrator::{BuildOptions, BuildOrchestrator, BuildResult, TestOptions};
pub use test_runner::{TestFramework, TestResult, TestRunner};
