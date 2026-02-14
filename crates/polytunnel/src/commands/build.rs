use super::utils::print_status;
use crate::platform::Platform;
use color_eyre::eyre::Result;
use colored::*;
use polytunnel_build::{BuildError, BuildOrchestrator, TestOptions};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

pub async fn cmd_build(clean: bool, skip_tests: bool, verbose: bool) -> Result<()> {
    let start = Instant::now();

    // Print platform information if verbose
    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();
    let version = "0.1.0"; // Placeholder

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Build options
    let _options = polytunnel_build::BuildOptions {
        clean,
        skip_tests,
        verbose,
    };

    // 0. Clean (if requested)
    if clean {
        orchestrator.clean()?;
    }

    // 1. Resolve Dependencies
    print_status("Resolving", "dependencies", Color::Cyan);
    orchestrator.resolve_dependencies(verbose).await?;

    // 2. Compile Main
    print_status("Compiling", &format!("{} v{}", name, version), Color::Green);
    let _compiled = orchestrator.compile_sources().await?;

    // 3. Compile Tests (if needed)
    if !skip_tests {
        // Orchestrator compile_tests is pub
    }

    let duration_secs = start.elapsed().as_secs_f64();
    print_status(
        "Finished",
        &format!(
            "dev [unoptimized + debuginfo] target(s) in {:.2}s",
            duration_secs
        ),
        Color::Green,
    );

    // 4. Run Tests
    if !skip_tests {
        print_status(
            "Compiling",
            &format!("{} v{} (test)", name, version),
            Color::Green,
        );
        orchestrator.compile_tests().await?;

        println!(
            "\n     Running unittests ({})",
            "target/test-classes".white()
        );
        print_status("Testing", &format!("{} ...", name), Color::Cyan);

        let test_opts = TestOptions {
            pattern: None,
            verbose,
            fail_fast: false,
        };

        let test_start = Instant::now();
        let test_result = orchestrator.run_tests(&test_opts).await?;
        let test_duration = test_start.elapsed().as_secs_f64();

        let status_color = if test_result.failed > 0 {
            Color::Red
        } else {
            Color::Green
        };
        let status_text = if test_result.failed > 0 {
            "FAILED"
        } else {
            "ok"
        };

        println!(
            "\ntest result: {}. {} passed; {} failed; {} ignored; 0 measured; 0 filtered out; finished in {:.2}s\n",
            status_text.color(status_color),
            test_result.passed,
            test_result.failed,
            test_result.skipped,
            test_duration
        );

        if test_result.failed > 0 {
            return Err(BuildError::TestExecutionFailed {
                message: format!("{} test(s) failed", test_result.failed),
            }
            .into());
        }
    }

    let total_duration = start.elapsed();
    let duration_str = if total_duration.as_secs() > 0 {
        format!("{}s", total_duration.as_secs())
    } else {
        format!("{}ms", total_duration.as_millis())
    };

    println!(
        "\n{} in {}\n",
        "BUILD SUCCESSFUL".green().bold(),
        duration_str
    );

    Ok(())
}
