use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_init_creates_config() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    let mut cmd = Command::cargo_bin("pt")?;
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
    let mut cmd = Command::cargo_bin("pt")?;
    cmd.current_dir(dir.path())
        .arg("init")
        .arg("initial-project")
        .assert()
        .success();

    // Try to init again
    let mut cmd2 = Command::cargo_bin("pt")?;
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
fn test_add_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pt")?;
    cmd.arg("add")
        .arg("com.example:lib:1.0.0")
        .assert()
        .success()
        .stdout(predicates::str::contains("Adding"));
    Ok(())
}

#[test]
fn test_remove_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pt")?;
    cmd.arg("remove")
        .arg("com.example:lib")
        .assert()
        .success()
        .stdout(predicates::str::contains("Removing"));
    Ok(())
}

#[test]
fn test_sync_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pt")?;
    cmd.arg("sync")
        .assert()
        .success()
        .stdout(predicates::str::contains("Syncing"));
    Ok(())
}

#[test]
fn test_tree_command_runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pt")?;
    cmd.arg("tree")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dependency tree"));
    Ok(())
}
