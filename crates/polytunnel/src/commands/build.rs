use super::utils::{check_test_failures, print_status, print_test_result};
use crate::platform::Platform;
use color_eyre::eyre::Result;
use colored::*;
use polytunnel_build::{BuildOptions, BuildOrchestrator};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

pub async fn cmd_build(clean: bool, skip_tests: bool, verbose: bool) -> Result<()> {
    let start = Instant::now();

    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();
    let mut orchestrator = BuildOrchestrator::new(config)?;

    let options = BuildOptions {
        clean,
        skip_tests,
        verbose,
    };

    print_status(
        "Compiling",
        &format!("{} v{}", name, env!("CARGO_PKG_VERSION")),
        Color::Green,
    );
    let result = orchestrator.build(&options).await?;

    if let Some(ref test_result) = result.test_result {
        let test_duration = result.duration.as_secs_f64();
        print_test_result(test_result, test_duration);
        check_test_failures(test_result)?;
    }

    let total = start.elapsed();
    let duration_str = if total.as_secs() > 0 {
        format!("{}s", total.as_secs())
    } else {
        format!("{}ms", total.as_millis())
    };

    println!(
        "\n{} in {}\n",
        "BUILD SUCCESSFUL".green().bold(),
        duration_str
    );

    Ok(())
}
