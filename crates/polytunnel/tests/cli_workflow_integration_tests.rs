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
fn test_init_then_build_and_test_flow() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

    fs::write(
        dir.path().join("src/main/java/App.java"),
        "public class App {} \n",
    )?;
    fs::write(
        dir.path().join("src/test/java/AppTest.java"),
        "class AppTest {}\n",
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("init")
        .arg("demo")
        .assert()
        .success()
        .stdout(predicates::str::contains("Ignored"));

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests"])
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

    assert!(dir.path().join("target/classes").exists());
    assert!(dir.path().join("target/test-classes").exists());

    Ok(())
}

#[test]
fn test_build_respects_custom_output_directories() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    fs::write(
        dir.path().join("polytunnel.toml"),
        r#"
[project]
name = "demo"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "out/classes"
test_output_dir = "out/test-classes"
cache_dir = ".polytunnel/cache"
"#,
    )?;

    fs::create_dir_all(dir.path().join("src/main/java"))?;
    fs::write(
        dir.path().join("src/main/java/App.java"),
        "public class App { }",
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests"])
        .assert()
        .success()
        .stdout(predicates::str::contains("BUILD SUCCESSFUL"));

    assert!(dir.path().join("out/classes").exists());

    Ok(())
}

#[test]
fn test_build_clean_removes_stale_artifacts() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;
    fs::create_dir_all(dir.path().join("target/classes"))?;
    fs::write(
        dir.path().join("target/classes/stale.class"),
        b"stale bytes",
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--clean", "--skip-tests"])
        .assert()
        .success()
        .stdout(predicates::str::contains("BUILD SUCCESSFUL"));

    assert!(!dir.path().join("target/classes/stale.class").exists());

    Ok(())
}

#[test]
fn test_build_verbose_mode_reports_platform() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;
    fs::write(
        dir.path().join("src/main/java/App.java"),
        "public class App {}",
    )?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["build", "--skip-tests", "--verbose"])
        .assert()
        .success()
        .stderr(predicates::str::contains("Build platform"))
        .stdout(predicates::str::contains("BUILD SUCCESSFUL"));

    Ok(())
}

#[test]
fn test_vscode_generates_ide_artifacts() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    write_minimal_project(dir.path())?;

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

#[test]
fn test_sync_and_tree_commands_require_config() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("sync")
        .assert()
        .failure();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("tree")
        .assert()
        .failure();

    Ok(())
}
