use assert_cmd::Command;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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

fn write_minimal_source(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path.join("src/main/java"))?;
    fs::write(
        path.join("src/main/java/App.java"),
        "public class App { public static void main(String[] args) {} }",
    )?;
    Ok(())
}

#[cfg(unix)]
fn write_failing_javac_script(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let script = dir.join("javac");
    let mut file = fs::File::create(&script)?;
    file.write_all(b"#!/usr/bin/env sh\n")?;
    file.write_all(b"echo \"mock javac failure\" >&2\n")?;
    file.write_all(b"exit 1\n")?;

    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms)?;

    Ok(script)
}

#[cfg(unix)]
fn write_success_javac_script(java_home: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let bin_dir = java_home.join("bin");
    fs::create_dir_all(&bin_dir)?;
    let script = bin_dir.join("javac");
    let mut file = fs::File::create(&script)?;
    file.write_all(b"#!/usr/bin/env sh\n")?;
    file.write_all(b"exit 0\n")?;

    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms)?;

    Ok(script)
}

#[cfg(unix)]
fn write_failed_test_java_script(dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let script = dir.join("java");
    let mut file = fs::File::create(&script)?;
    file.write_all(b"#!/usr/bin/env sh\n")?;
    file.write_all(b"echo '1 tests found'\n")?;
    file.write_all(b"echo '0 tests successful'\n")?;
    file.write_all(b"echo '1 tests failed'\n")?;
    file.write_all(b"exit 0\n")?;

    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms)?;

    Ok(script)
}

#[test]
fn test_build_without_config_fails_with_clear_message() {
    let dir = tempdir().expect("tempdir");

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("IO error"));
}

#[test]
fn test_build_with_missing_source_directory_fails() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    fs::write(
        dir.path().join("polytunnel.toml"),
        r#"
[project]
name = "broken"
java_version = "17"

[build]
source_dirs = ["missing/src"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"
cache_dir = ".polytunnel/cache"
"#,
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Source directory not found"));

    Ok(())
}

#[test]
fn test_test_without_config_is_reported() {
    let dir = tempdir().expect("tempdir");

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("test")
        .assert()
        .failure()
        .stderr(predicates::str::contains("IO error"));
}

#[test]
fn test_invalid_toml_reports_parse_error() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join("polytunnel.toml"), "project = [broken")?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("build")
        .assert()
        .failure()
        .stderr(predicates::str::contains("TOML parse error"));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_build_reports_mocked_compiler_failure() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;
    write_minimal_source(dir.path())?;

    let mock_bin = tempdir()?;
    let _script = write_failing_javac_script(mock_bin.path())?;
    let path = format!(
        "{}:{}",
        mock_bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests"])
        .env("PATH", path)
        .assert()
        .failure()
        .stderr(predicates::str::contains("Compilation failed with"));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_test_command_reports_failed_tests() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    fs::create_dir_all(dir.path().join("src/main/java"))?;
    fs::create_dir_all(dir.path().join("src/test/java"))?;
    fs::create_dir_all(dir.path().join("target/test-classes"))?;
    fs::write(
        dir.path().join("target/test-classes/AppTest.class"),
        b"class",
    )?;

    fs::write(
        dir.path().join("polytunnel.toml"),
        r#"
[project]
name = "demo"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/junit-platform-console-standalone.jar"
test_output_dir = "target/test-classes"
cache_dir = ".polytunnel/cache"
"#,
    )?;

    let fake_java_bin = tempdir()?;
    let fake_java_home = tempdir()?;
    let _java = write_failed_test_java_script(fake_java_bin.path())?;
    let _javac = write_success_javac_script(fake_java_home.path())?;
    let path = format!(
        "{}:{}",
        fake_java_bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("test")
        .env("PATH", path)
        .env("JAVA_HOME", fake_java_home.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("1 test(s) failed"));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_vscode_maps_build_error_when_javac_missing() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("vscode")
        .env("PATH", "")
        .env_remove("JAVA_HOME")
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Java compiler (javac) not found in PATH",
        ));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_vscode_maps_io_error_when_project_path_is_directory() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;
    fs::create_dir_all(dir.path().join(".project"))?;

    let fake_java_home = tempdir()?;
    let _javac = write_success_javac_script(fake_java_home.path())?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("vscode")
        .env("PATH", "")
        .env("JAVA_HOME", fake_java_home.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains("IO error"));

    Ok(())
}

#[cfg(not(unix))]
#[test]
fn test_build_reports_compiler_failure_placeholder() {
    // Windows CI can run this behavior through environment-specific integration.
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage: pt"));
}
