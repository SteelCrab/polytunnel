use clap::{Parser, Subcommand};
use polytunnel_build::{BuildError, BuildOptions, BuildOrchestrator, TestOptions};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

type Result<T> = std::result::Result<T, BuildError>;

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

use colored::*;

// Helper for formatted status output
fn print_status(status: &str, message: &str, color: Color) {
    println!("{:>12} {}", status.color(color).bold(), message);
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
        print_status("Ignored", "polytunnel.toml already exists", Color::Yellow);
        return Ok(());
    }

    let config = ProjectConfig::new(name);
    config.save(config_path)?;
    print_status(
        "Created",
        &format!("polytunnel.toml for project: {}", name),
        Color::Green,
    );
    Ok(())
}

async fn cmd_add(dependency: &str) -> Result<()> {
    print_status("Adding", dependency, Color::Green);
    // TODO: Implement in Phase 3
    Ok(())
}

fn cmd_remove(dependency: &str) -> Result<()> {
    print_status("Removing", dependency, Color::Red);
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_sync() -> Result<()> {
    print_status("Syncing", "dependencies...", Color::Cyan);
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_tree() -> Result<()> {
    println!("Dependency tree:");
    // TODO: Implement in Phase 3
    Ok(())
}

async fn cmd_build(clean: bool, skip_tests: bool, verbose: bool) -> Result<()> {
    let _start = Instant::now();

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();
    let version = "0.1.0"; // Placeholder for now

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // Build options
    let options = BuildOptions {
        clean,
        skip_tests,
        verbose,
    };

    // Execute build
    print_status("Building", &format!("{} v{}", name, version), Color::Green);

    // We want to capture the orchestrator's progress in a cleaner way.
    // Ideally, orchestrator should take a callback or return richer status.
    // For now, we rely on its verbose flag for details, but we can print high-level steps here.

    // 1. Resolve
    print_status("Resolving", "dependencies", Color::Cyan);
    // The orchestrator calls classpath_builder which prints nothing by default unless verbose
    // We'll let verbose handle deep details

    let result = orchestrator.build(&options).await?;

    // Report results
    let duration_secs = result.duration.as_secs_f64();
    print_status(
        "Finished",
        &format!(
            "dev [unoptimized + debuginfo] target(s) in {:.2}s",
            duration_secs
        ),
        Color::Green,
    );

    if !skip_tests && result.test_result.is_some() {
        let test_result = result.test_result.unwrap();

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
            "\n     Running unittests ({})",
            "target/test-classes".white()
        );
        print_status(
            "Testing",
            &format!("{} ... {}", name, status_text.color(status_color)),
            Color::Cyan,
        );

        println!(
            "\ntest result: {}. {} passed; {} failed; {} ignored; 0 measured; 0 filtered out; finished in {:.2}s\n",
            status_text.color(status_color),
            test_result.passed,
            test_result.failed,
            test_result.skipped,
            duration_secs // Approximation
        );

        if test_result.failed > 0 {
            return Err(BuildError::TestExecutionFailed {
                message: format!("{} test(s) failed", test_result.failed),
            });
        }
    }

    Ok(())
}

async fn cmd_test(pattern: Option<String>, verbose: bool, fail_fast: bool) -> Result<()> {
    let start = Instant::now();

    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    let name = config.project.name.clone();

    // Create build orchestrator
    let mut orchestrator = BuildOrchestrator::new(config)?;

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
        });
    }

    Ok(())
}
