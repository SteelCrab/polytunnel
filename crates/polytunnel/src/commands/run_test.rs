use super::utils::{check_test_failures, print_status, print_test_result};
use crate::platform::Platform;
use color_eyre::eyre::Result;
use colored::*;
use polytunnel_build::{BuildOrchestrator, TestOptions};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

pub async fn cmd_test(pattern: Option<String>, verbose: bool, fail_fast: bool) -> Result<()> {
    let start = Instant::now();

    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();

    let mut orchestrator = BuildOrchestrator::new(config)?;

    print_status(
        "Compiling",
        &format!("{} v{} (test)", name, env!("CARGO_PKG_VERSION")),
        Color::Green,
    );
    orchestrator.compile_tests()?;

    let options = TestOptions {
        pattern,
        verbose,
        fail_fast,
    };

    print_status("Running", "tests", Color::Green);
    let result = orchestrator.run_tests(&options).await?;

    let duration_secs = start.elapsed().as_secs_f64();
    print_test_result(&result, duration_secs);
    check_test_failures(&result)?;

    Ok(())
}
