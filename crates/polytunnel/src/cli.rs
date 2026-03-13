//! CLI argument parsing definitions

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pt")]
#[command(about = "Fast Java dependency manager", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    Sync {
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show dependency tree
    Tree {
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
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
