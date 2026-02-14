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
fn test_init_creates_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .arg("init")
        .arg("test-project")
        .assert()
        .success();

    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("name = \"test-project\""));

    Ok(())
}

#[test]
fn test_init_ignores_existing() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    // Create initial config
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .arg("init")
        .arg("initial-project")
        .assert()
        .success();

    // Try to init again
    let mut cmd2 = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd2.current_dir(dir.path())
        .arg("init")
        .arg("new-project")
        .assert()
        .success();

    // Verify content hasn't changed
    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("name = \"initial-project\""));
    assert!(!content.contains("name = \"new-project\""));

    Ok(())
}

#[test]
fn test_build_command_compiles_sources() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    let source = dir.path().join("src/main/java/App.java");
    fs::write(
        source,
        r#"
public class App {
    public static void main(String[] args) {
        System.out.println("hello");
    }
}
"#,
    )?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .args(["build", "--skip-tests"])
        .assert()
        .success()
        .stdout(predicates::str::contains("BUILD SUCCESSFUL"));

    assert!(dir.path().join("target/classes/App.class").exists());

    Ok(())
}

#[test]
fn test_build_command_respects_clean_flag() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    // Pre-create output directory so we can verify `--clean` clears it before compilation.
    fs::create_dir_all(dir.path().join("target/classes"))?;
    fs::write(dir.path().join("target/classes/stale.txt"), b"stale")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .args(["build", "--skip-tests", "--clean", "--verbose"])
        .assert()
        .success();

    assert!(!dir.path().join("target/classes").exists());

    Ok(())
}

#[test]
fn test_test_command_runs_without_framework_dependency() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    fs::write(
        dir.path().join("src/test/java/AppTest.java"),
        "class AppTest {}\n",
    )?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path())
        .arg("test")
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicates::str::contains("test result: ok."));

    Ok(())
}

#[test]
fn test_vscode_command_generates_project_files() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.current_dir(dir.path()).arg("vscode").assert().success();

    assert!(dir.path().join(".project").exists());
    assert!(dir.path().join(".classpath").exists());
    assert!(dir.path().join(".vscode/settings.json").exists());

    Ok(())
}

#[test]
fn test_add_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("add")
        .arg("com.example:lib:1.0.0")
        .assert()
        .success()
        .stdout(predicates::str::contains("Adding"));
    Ok(())
}

#[test]
fn test_remove_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("remove")
        .arg("com.example:lib")
        .assert()
        .success()
        .stdout(predicates::str::contains("Removing"));
    Ok(())
}

#[test]
fn test_sync_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("sync")
        .assert()
        .success()
        .stdout(predicates::str::contains("Syncing"));
    Ok(())
}

#[test]
fn test_tree_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
    cmd.arg("tree")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dependency tree"));
    Ok(())
}
