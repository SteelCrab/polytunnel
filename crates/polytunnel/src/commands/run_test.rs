use super::utils::print_status;
use crate::platform::Platform;
use color_eyre::eyre::Result;
use colored::*;
use polytunnel_build::{BuildError, BuildOrchestrator, TestOptions};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

pub async fn cmd_test(pattern: Option<String>, verbose: bool, fail_fast: bool) -> Result<()> {
    let start = Instant::now();

    // Print platform information if verbose
    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Resolve dependencies first (required for classpath)
    orchestrator.resolve_dependencies(verbose).await?;

    print_status(
        "Compiling",
        &format!("{} v0.1.0 (test)", name),
        Color::Green,
    );
    orchestrator.compile_tests().await?;

    // Run tests
    let options = TestOptions {
        pattern,
        verbose,
        fail_fast,
    };

    print_status("Running", "tests", Color::Green);
    let result = orchestrator.run_tests(&options).await?;

    let duration_secs = start.elapsed().as_secs_f64();

    let status_color = if result.failed > 0 {
        Color::Red
    } else {
        Color::Green
    };
    let status_text = if result.failed > 0 { "FAILED" } else { "ok" };

    println!(
        "\ntest result: {}. {} passed; {} failed; {} ignored; 0 measured; 0 filtered out; finished in {:.2}s\n",
        status_text.color(status_color),
        result.passed,
        result.failed,
        result.skipped,
        duration_secs
    );

    if result.failed > 0 {
        return Err(BuildError::TestExecutionFailed {
            message: format!("{} test(s) failed", result.failed),
        }
        .into());
    }

    Ok(())
}
