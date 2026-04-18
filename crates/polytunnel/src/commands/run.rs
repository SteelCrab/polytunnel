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
    do_run(main_class, args, verbose, Path::new("polytunnel.toml")).await
}

pub(crate) async fn do_run(
    main_class: &str,
    args: &[String],
    verbose: bool,
    config_path: &Path,
) -> Result<()> {
    if !config_path.exists() {
        bail!("polytunnel.toml not found. Run `pt init` first.");
    }

    if main_class.trim().is_empty() {
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

    let classpaths = orchestrator.get_resolved_classpath();
    let mut classpath_entries = classpaths.runtime_classpath.clone();
    classpath_entries.push(output_dir);
    let classpath_str = format_classpath(&classpath_entries);

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

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}
