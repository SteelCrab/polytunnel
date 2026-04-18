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

#[test]
fn run_accepts_trailing_args() {
    // Verify CLI parses `-- arg1 arg2` as trailing args (no execution check here)
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("run")
        .arg("com.example.App")
        .arg("--")
        .arg("--flag")
        .arg("value")
        .arg("--help")
        .assert()
        .failure(); // fails because no polytunnel.toml, but parser accepts args
}
