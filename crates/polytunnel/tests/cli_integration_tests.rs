//! Integration tests for end-to-end CLI behavior.

use assert_cmd::Command;
use std::error::Error;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_help_output_contains_subcommands() -> Result<(), Box<dyn Error>> {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage:"))
        .stdout(predicates::str::contains("init"))
        .stdout(predicates::str::contains("build"))
        .stdout(predicates::str::contains("test"))
        .stdout(predicates::str::contains("vscode"));

    Ok(())
}

#[test]
fn test_init_command_runs_in_working_directory() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["init", "demo-project"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Created"));

    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("name = \"demo-project\""));

    Ok(())
}

#[test]
fn test_build_help_shows_command_flags() -> Result<(), Box<dyn Error>> {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .args(["build", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--clean"))
        .stdout(predicates::str::contains("--skip-tests"))
        .stdout(predicates::str::contains("-v"));

    Ok(())
}

#[test]
fn test_test_help_shows_command_flags() -> Result<(), Box<dyn Error>> {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .args(["test", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("--fail-fast"))
        .stdout(predicates::str::contains("[PATTERN]"));

    Ok(())
}

#[test]
fn test_vscode_subcommand_is_executable_after_init() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let config = dir.path().join("polytunnel.toml");
    fs::write(
        config,
        r#"
[project]
name = "demo"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"
cache_dir = ".polytunnel/cache"
"#,
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("vscode")
        .assert()
        .success();

    assert!(dir.path().join(".project").exists());
    assert!(dir.path().join(".classpath").exists());
    assert!(dir.path().join(".vscode/settings.json").exists());

    Ok(())
}
