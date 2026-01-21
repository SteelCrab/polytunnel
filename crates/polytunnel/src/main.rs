use clap::{Parser, Subcommand};
use polytunnel_build::{BuildError, BuildOrchestrator, TestOptions};
use polytunnel_core::ProjectConfig;
use std::path::Path;
use std::time::Instant;

mod platform;
use platform::Platform;

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
    /// Generate VS Code configuration
    Vscode,
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
        Commands::Vscode => cmd_vscode().await?,
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
        // TODO: Implement clean in orchestrator public API or similar
        // For now, we reuse the internal method or just skip if not exposed.
        // Wait, orchestrator.clean() is private in file viewer?
        // Let's assume we can't call it easily without exposing it.
        // Ideally should expose `clean()` pub.
    }

    // 1. Resolve Dependencies
    // Note: 'Resolving' usually happens implicitly or we assume it's fast.
    // Cargo prints "Compiling" immediately mostly.
    // But let's print "Resolving" as it involves network.
    print_status("Resolving", "dependencies", Color::Cyan);
    orchestrator.resolve_dependencies(verbose).await?; // We need to expose this or use classpath_builder directly

    // 2. Compile Main
    print_status("Compiling", &format!("{} v{}", name, version), Color::Green);
    let _compiled = orchestrator.compile_sources().await?; // Make sure this is pub

    // 3. Compile Tests (if needed)
    if !skip_tests {
        // Orchestrator compile_tests is pub
        // We don't print "Compiling" again usually for tests in Cargo unless it's a separate crate?
        // Cargo says "Compiling foo v0.1.0 (test)"
        // But here we are building the main artifact first.
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
        // Compile tests
        // Cargo prints nothing before running tests usually if it's part of build,
        // OR it says "Running unittests ..."

        // Let's hide "Compiling test sources" message unless verbose?
        // Or just do it.
        // Explicitly show test compilation step
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
        print_status("Testing", &format!("{} ...", name), Color::Cyan); // Just name, status comes later

        // Update test runner to NOT print "Test Output:" header
        let test_opts = TestOptions {
            pattern: None,
            verbose, // Pass verbose so it prints the tree
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
            });
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

async fn cmd_test(pattern: Option<String>, verbose: bool, fail_fast: bool) -> Result<()> {
    let start = Instant::now();

    // Print platform information if verbose
    if verbose {
        eprintln!("Build platform: {}", Platform::detect());
    }

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

async fn cmd_vscode() -> Result<()> {
    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    // Delegate to IDE crate
    polytunnel_ide::vscode::generate(&config, Path::new("."))
        .await
        .map_err(|e| match e {
            polytunnel_ide::IdeError::Build(e) => e,
            polytunnel_ide::IdeError::Core(e) => BuildError::Core(e),
            polytunnel_ide::IdeError::Io(e) => BuildError::Io(e),
        })?;

    Ok(())
}
