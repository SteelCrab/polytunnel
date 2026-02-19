use super::utils::print_status;
use color_eyre::eyre::Result;
use colored::Color;
use polytunnel_build::BuildOrchestrator;
use polytunnel_core::ProjectConfig;
use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

pub async fn cmd_sync(verbose: bool) -> Result<()> {
    do_sync(Path::new("polytunnel.toml"), verbose).await
}

pub(crate) async fn do_sync(config_path: &Path, verbose: bool) -> Result<()> {
    let start = Instant::now();

    let config = ProjectConfig::load(config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config)?;

    print_status("Resolving", "dependencies", Color::Cyan);
    orchestrator.resolve_dependencies(verbose).await?;

    let duration = start.elapsed();
    let duration_str = format_duration(&duration);

    let classpath = orchestrator.get_resolved_classpath();
    let unique_jars: HashSet<_> = classpath
        .compile_classpath
        .iter()
        .chain(classpath.test_classpath.iter())
        .chain(classpath.runtime_classpath.iter())
        .collect();

    print_status(
        "Synced",
        &format!("{} dependencies in {}", unique_jars.len(), duration_str),
        Color::Green,
    );

    Ok(())
}

pub(crate) fn format_duration(duration: &Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{}s", duration.as_secs())
    } else {
        format!("{}ms", duration.as_millis())
    }
}
