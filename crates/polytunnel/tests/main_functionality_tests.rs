//! High-level user flow coverage.

use assert_cmd::Command;
use std::error::Error;
use std::fs;
use tempfile::tempdir;

fn write_minimal_project(path: &std::path::Path) -> Result<(), Box<dyn Error>> {
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
fn test_version_is_available() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::contains("pt"));
}

#[test]
fn test_help_shows_expected_sections() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Fast Java dependency manager"))
        .stdout(predicates::str::contains("Usage: pt"));
}

#[test]
fn test_build_and_test_flow_executes() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    fs::write(
        dir.path().join("src/main/java/App.java"),
        "public class App { public static void main(String[] args) {} }\n",
    )?;

    fs::write(
        dir.path().join("src/test/java/AppTest.java"),
        "class AppTest {}\n",
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("build")
        .arg("--skip-tests")
        .assert()
        .success()
        .stdout(predicates::str::contains("BUILD SUCCESSFUL"));

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("test")
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicates::str::contains("test result: ok."));

    Ok(())
}
