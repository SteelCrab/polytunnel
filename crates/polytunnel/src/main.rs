//! Polytunnel CLI - Fast Java dependency manager

use clap::Parser;
use color_eyre::eyre::Result;

mod cli;
mod commands;
mod platform;

use cli::{Cli, Commands};
use commands::*;

/// Main entry point - just installs error handler and delegates to run()
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run().await
}

/// Application logic separated for testability
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => cmd_init(&name)?,
        Commands::Add { dependency } => cmd_add(&dependency).await?,
        Commands::Remove { dependency } => cmd_remove(&dependency)?,
        Commands::Sync => cmd_sync().await?,
        Commands::Tree => cmd_tree().await?,
        Commands::Build {
            clean,
            skip_tests,
            verbose,
        } => cmd_build(clean, skip_tests, verbose).await?,
        Commands::Test {
            pattern,
            verbose,
            fail_fast,
        } => cmd_test(pattern, verbose, fail_fast).await?,
        Commands::Vscode => cmd_vscode().await?,
    }

    Ok(())
}
