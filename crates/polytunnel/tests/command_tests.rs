//! Targeted CLI command behavior checks.

use assert_cmd::Command;
use std::fs;

#[test]
fn test_add_command_prints_action() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("polytunnel.toml"),
        "[project]\nname = \"demo\"\njava_version = \"17\"\n",
    )
    .unwrap();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["add", "com.example:demo:1.0.0"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Added"));
}

#[test]
fn test_remove_command_prints_action() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("polytunnel.toml"),
        r#"[project]
name = "demo"
java_version = "17"

[dependencies]
"com.example:demo" = "1.0.0"
"#,
    )
    .unwrap();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["remove", "com.example:demo"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Removed"));
}

#[test]
fn test_sync_command_fails_without_config() {
    let dir = tempfile::tempdir().unwrap();
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("sync")
        .assert()
        .failure();
}

#[test]
fn test_tree_command_fails_without_config() {
    let dir = tempfile::tempdir().unwrap();
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .arg("tree")
        .assert()
        .failure();
}

#[test]
fn test_add_command_with_scope() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("polytunnel.toml"),
        "[project]\nname = \"demo\"\njava_version = \"17\"\n",
    )
    .unwrap();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["add", "com.example:lib:1.0.0", "--scope", "test"])
        .assert()
        .success()
        .stdout(predicates::str::contains("scope: test"));
}

#[test]
fn test_add_command_with_invalid_scope() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("polytunnel.toml"),
        "[project]\nname = \"demo\"\njava_version = \"17\"\n",
    )
    .unwrap();

    Command::new(env!("CARGO_BIN_EXE_pt"))
        .current_dir(dir.path())
        .args(["add", "com.example:lib:1.0.0", "--scope", "invalid"])
        .assert()
        .failure();
}

#[test]
fn test_unknown_subcommand_returns_error() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}
