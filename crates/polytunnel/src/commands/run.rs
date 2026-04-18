use super::utils::print_status;
use crate::platform::Platform;
use color_eyre::eyre::{Result, bail};
use colored::Color;
use polytunnel_build::{BuildOptions, BuildOrchestrator, format_classpath};
use polytunnel_core::ProjectConfig;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

pub async fn cmd_run(main_class: &str, args: &[String], verbose: bool) -> Result<()> {
    let exit_code = do_run(main_class, args, verbose, Path::new("polytunnel.toml")).await?;
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}

pub(crate) async fn do_run(
    main_class: &str,
    args: &[String],
    verbose: bool,
    config_path: &Path,
) -> Result<i32> {
    if !config_path.exists() {
        bail!("polytunnel.toml not found. Run `pt init` first.");
    }

    let main_class = main_class.trim();
    if main_class.is_empty() {
        bail!("Main class must not be empty.");
    }

    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    let config = ProjectConfig::load(config_path)?;
    let output_dir = PathBuf::from(&config.build.output_dir);
    let mut orchestrator = BuildOrchestrator::new(config)?;

    print_status("Compiling", &format!("{} (run)", main_class), Color::Green);
    orchestrator
        .build(&BuildOptions {
            clean: false,
            skip_tests: true,
            verbose,
        })
        .await?;

    let classpath_str = build_run_classpath(&orchestrator, &output_dir);

    print_status("Running", main_class, Color::Green);

    let mut cmd = Command::new("java");
    cmd.arg("-cp")
        .arg(&classpath_str)
        .arg(main_class)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Failed to execute java: {}", e))?;

    Ok(status.code().unwrap_or(1))
}

fn build_run_classpath(orchestrator: &BuildOrchestrator, output_dir: &Path) -> String {
    let classpaths = orchestrator.get_resolved_classpath();
    // Project classes must precede dependencies so local overrides win against
    // identically-named classes that may exist in resolved jars.
    let mut entries = vec![output_dir.to_path_buf()];
    entries.extend(classpaths.runtime_classpath.iter().cloned());
    format_classpath(&entries)
}
