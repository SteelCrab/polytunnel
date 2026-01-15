use clap::{Parser, Subcommand};
use polytunnel_build::{BuildOptions, BuildOrchestrator, TestOptions};
use polytunnel_core::{AppError, ProjectConfig, Result};
use std::path::Path;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "pt")]
#[command(about = "Fast Java dependency manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Project name
        #[arg(default_value = "my-java-app")]
        name: String,
    },
    /// Add a dependency
    Add {
        /// Dependency in format: groupId:artifactId:version
        dependency: String,
    },
    /// Remove a dependency
    Remove {
        /// Dependency in format: groupId:artifactId
        dependency: String,
    },
    /// Sync dependencies
    Sync,
    /// Show dependency tree
    Tree,
    /// Build the project
    Build {
        /// Clean build (remove existing outputs)
        #[arg(long)]
        clean: bool,

        /// Skip test compilation and execution
        #[arg(long)]
        skip_tests: bool,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run tests
    Test {
        /// Test class or pattern to run
        #[arg(value_name = "PATTERN")]
        pattern: Option<String>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Fail fast (stop on first failure)
        #[arg(long)]
        fail_fast: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
    }

    Ok(())
}

fn cmd_init(name: &str) -> Result<()> {
    let config_path = Path::new("polytunnel.toml");

    if config_path.exists() {
        println!("polytunnel.toml already exists");
        return Ok(());
    }

    let config = ProjectConfig::new(name);
    config.save(config_path)?;
    println!("Created polytunnel.toml for project: {}", name);
    Ok(())
}

async fn cmd_add(dependency: &str) -> Result<()> {
    println!("Adding: {}", dependency);
    // TODO: Implement in Phase 3
    Ok(())
}

fn cmd_remove(dependency: &str) -> Result<()> {
    println!("Removing: {}", dependency);
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_sync() -> Result<()> {
    println!("Syncing dependencies...");
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_tree() -> Result<()> {
    println!("Dependency tree:");
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_build(clean: bool, skip_tests: bool, verbose: bool) -> Result<()> {
    let start = Instant::now();

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Build options
    let options = BuildOptions {
        clean,
        skip_tests,
        verbose,
    };

    // Execute build
    if verbose {
        println!("Building {}...", orchestrator.config.project.name);
    }

    let result = orchestrator.build(&options).await?;

    // Report results
    println!("\n{}", "=".repeat(60));
    println!("Build Summary:");
    println!("{}", "=".repeat(60));
    println!("Compiled: {} files", result.compiled_files);
    println!("Time: {:.2}s", result.duration.as_secs_f64());

    if !skip_tests && result.test_result.is_some() {
        let test_result = result.test_result.unwrap();
        println!("\n{}", "-".repeat(60));
        println!("Test Summary:");
        println!("{}", "-".repeat(60));
        println!("Total: {}", test_result.total);
        println!("Passed: {}", test_result.passed);
        println!("Failed: {}", test_result.failed);
        println!("Skipped: {}", test_result.skipped);

        if test_result.failed > 0 {
            println!("\n{}", "-".repeat(60));
            println!("Failures:");
            for failure in test_result.failures {
                println!("\n  {} > {}", failure.class_name, failure.test_name);
                println!("    {}", failure.message);
                if verbose {
                    println!("\n{}", failure.stacktrace);
                }
            }

            return Err(AppError::TestExecutionFailed {
                message: format!("{} test(s) failed", test_result.failed),
            });
        }
    }

    println!("{}", "=".repeat(60));
    Ok(())
}

async fn cmd_test(pattern: Option<String>, verbose: bool, fail_fast: bool) -> Result<()> {
    let start = Instant::now();

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Ensure test sources are compiled first
    if verbose {
        println!("Compiling test sources...");
    }
    orchestrator.compile_tests().await?;

    // Run tests
    let options = TestOptions {
        pattern,
        verbose,
        fail_fast,
    };

    if verbose {
        println!("Running tests...");
    }
    let result = orchestrator.run_tests(&options).await?;

    // Report results
    println!("\n{}", "=".repeat(60));
    println!("Test Results:");
    println!("{}", "=".repeat(60));
    println!("Total: {}", result.total);
    println!(
        "Passed: {} ({}%)",
        result.passed,
        if result.total > 0 {
            (result.passed as f64 / result.total as f64 * 100.0) as u32
        } else {
            0
        }
    );
    println!("Failed: {}", result.failed);
    println!("Skipped: {}", result.skipped);
    println!("Time: {:.2}s", start.elapsed().as_secs_f64());

    if result.failed > 0 {
        println!("\n{}", "-".repeat(60));
        println!("Failures:");
        for failure in &result.failures {
            println!("\n  {} > {}", failure.class_name, failure.test_name);
            println!("    {}", failure.message);
            if verbose {
                println!("\n{}", failure.stacktrace);
            }
        }

        println!("{}", "=".repeat(60));
        return Err(AppError::TestExecutionFailed {
            message: format!("{} test(s) failed", result.failed),
        });
    }

    println!("{}", "=".repeat(60));
    Ok(())
}
