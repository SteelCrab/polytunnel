mod common;

use assert_cmd::Command;
use common::{TestProject, java_toolchain_available};
use std::error::Error;
use tempfile::tempdir;

#[test]
fn run_help_exits_zero() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .args(["run", "--help"])
        .assert()
        .success();
}

#[test]
fn run_requires_main_class() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("run")
        .assert()
        .failure();
}

#[test]
fn run_fails_without_config() {
    let dir = tempdir().unwrap();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["run", "com.example.App"])
        .assert()
        .failure();
}

#[test]
fn run_fails_with_empty_main_class() {
    TestProject::new().pt(&["run", ""]).assert().failure();
}

#[test]
fn run_cli_succeeds_when_main_exits_zero() -> Result<(), Box<dyn Error>> {
    if !java_toolchain_available() {
        eprintln!("Skipping: javac/java not available");
        return Ok(());
    }

    TestProject::new_named("cli-run-demo")
        .with_main("com.example.Hello", r#"System.out.println("ok");"#)
        .pt(&["run", "com.example.Hello"])
        .assert()
        .success();
    Ok(())
}

#[test]
fn run_cli_propagates_nonzero_exit_code() -> Result<(), Box<dyn Error>> {
    if !java_toolchain_available() {
        eprintln!("Skipping: javac/java not available");
        return Ok(());
    }

    TestProject::new_named("cli-run-demo")
        .with_main("com.example.Hello", "System.exit(42);")
        .pt(&["run", "com.example.Hello"])
        .assert()
        .code(42);
    Ok(())
}

#[test]
fn run_accepts_trailing_args() {
    // Prove clap accepted `-- --flag value --help` as trailing args by asserting
    // the missing-config error (reached do_run), not a clap parse error.
    let dir = tempdir().unwrap();

    let assert = Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["run", "com.example.App", "--", "--flag", "value", "--help"])
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("polytunnel.toml not found"),
        "expected missing-config error, got stderr: {stderr}"
    );
}
