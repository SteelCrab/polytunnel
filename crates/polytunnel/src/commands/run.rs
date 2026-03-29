use super::utils::print_status;
use crate::platform::Platform;
use color_eyre::eyre::Result;
use colored::*;
use polytunnel_build::{BuildOrchestrator, format_classpath};
use polytunnel_core::ProjectConfig;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

pub async fn cmd_run(main_class: &str, args: Vec<String>, verbose: bool) -> Result<()> {
    let start = Instant::now();

    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();
    let output_dir = config.build.output_dir.clone();
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Resolve dependencies
    orchestrator.resolve_dependencies(verbose).await?;

    // Compile main sources
    print_status(
        "Compiling",
        &format!("{} v{}", name, env!("CARGO_PKG_VERSION")),
        Color::Green,
    );
    orchestrator.compile_sources()?;

    // Build runtime classpath (compile + runtime scope, no test/provided)
    let classpaths = orchestrator.get_resolved_classpath();
    let mut classpath = classpaths.runtime_classpath;
    classpath.push(PathBuf::from(&output_dir));
    let classpath_str = format_classpath(&classpath);

    if verbose {
        eprintln!("Classpath: {}", classpath_str);
        eprintln!("Main class: {}", main_class);
        if !args.is_empty() {
            eprintln!("Arguments: {:?}", args);
        }
    }

    // Execute Java
    print_status("Running", main_class, Color::Green);

    let status = Command::new("java")
        .arg("-cp")
        .arg(&classpath_str)
        .arg(main_class)
        .args(&args)
        .status()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                polytunnel_build::BuildError::JavaNotFound
            } else {
                polytunnel_build::BuildError::JavaExecutionFailed {
                    message: e.to_string(),
                }
            }
        })?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    let total = start.elapsed();
    let duration_str = if total.as_secs() > 0 {
        format!("{}s", total.as_secs())
    } else {
        format!("{}ms", total.as_millis())
    };

    println!(
        "\n{} in {}\n",
        "RUN SUCCESSFUL".green().bold(),
        duration_str
    );

    Ok(())
}
