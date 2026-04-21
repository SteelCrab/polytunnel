use assert_cmd::Command;
use std::error::Error;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_minimal_project(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path.join("src/main/java"))?;
    fs::create_dir_all(path.join("src/test/java"))?;

    fs::write(
        path.join("polytunnel.toml"),
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

    Ok(())
}

#[test]
fn run_help_exits_zero() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("run").arg("--help").assert().success();
}

#[test]
fn run_requires_main_class() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("run").assert().failure();
}

#[test]
fn run_fails_without_config() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .arg("run")
        .arg("com.example.App")
        .assert()
        .failure();
}

#[test]
fn run_fails_with_empty_main_class() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .arg("run")
        .arg("")
        .assert()
        .failure();

    Ok(())
}

fn java_toolchain_available() -> bool {
    std::process::Command::new("javac")
        .arg("--version")
        .output()
        .is_ok()
        && std::process::Command::new("java")
            .arg("--version")
            .output()
            .is_ok()
}

fn write_run_fixture(dir: &Path, main_body: &str) -> Result<(), Box<dyn Error>> {
    let src_dir = dir.join("src/main/java/com/example");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(dir.join("src/test/java"))?;

    fs::write(
        src_dir.join("Hello.java"),
        format!(
            "package com.example; public class Hello {{ public static void main(String[] args) {{ {} }} }}",
            main_body
        ),
    )?;

    fs::write(
        dir.join("polytunnel.toml"),
        format!(
            r#"[project]
name = "cli-run-demo"
java_version = "17"

[build]
source_dirs = ["{src}"]
test_source_dirs = ["{test_src}"]
output_dir = "{out}"
test_output_dir = "{test_out}"
cache_dir = "{cache}"
"#,
            src = dir.join("src/main/java").display(),
            test_src = dir.join("src/test/java").display(),
            out = dir.join("target/classes").display(),
            test_out = dir.join("target/test-classes").display(),
            cache = dir.join(".polytunnel/cache").display(),
        ),
    )?;
    Ok(())
}

#[test]
fn run_cli_succeeds_when_main_exits_zero() -> Result<(), Box<dyn Error>> {
    if !java_toolchain_available() {
        eprintln!("Skipping: javac/java not available");
        return Ok(());
    }

    let dir = tempdir()?;
    write_run_fixture(dir.path(), r#"System.out.println("ok");"#)?;

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("run")
        .arg("com.example.Hello")
        .output()?;

    assert!(
        output.status.success(),
        "expected exit 0, got {:?}. stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
fn run_cli_propagates_nonzero_exit_code() -> Result<(), Box<dyn Error>> {
    if !java_toolchain_available() {
        eprintln!("Skipping: javac/java not available");
        return Ok(());
    }

    let dir = tempdir()?;
    write_run_fixture(dir.path(), "System.exit(42);")?;

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("run")
        .arg("com.example.Hello")
        .output()?;

    assert_eq!(
        output.status.code(),
        Some(42),
        "expected cmd_run to propagate exit code 42 via std::process::exit. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

#[test]
fn run_accepts_trailing_args() -> Result<(), Box<dyn Error>> {
    // Prove clap accepted `-- --flag value --help` and execution reached do_run
    // by asserting the missing-config error (not a clap parse error).
    let dir = tempdir()?;

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("run")
        .arg("com.example.App")
        .arg("--")
        .arg("--flag")
        .arg("value")
        .arg("--help")
        .output()?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("polytunnel.toml not found"),
        "expected missing-config error, got stderr: {stderr}"
    );
    Ok(())
}
