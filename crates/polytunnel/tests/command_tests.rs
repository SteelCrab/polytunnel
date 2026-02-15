//! Targeted CLI command behavior checks.

use assert_cmd::Command;

#[test]
fn test_add_command_prints_action() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .args(["add", "com.example:demo:1.0.0"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Adding"));
}

#[test]
fn test_remove_command_prints_action() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .args(["remove", "com.example:demo"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Removing"));
}

#[test]
fn test_sync_command_prints_action() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("sync")
        .assert()
        .success()
        .stdout(predicates::str::contains("Syncing"));
}

#[test]
fn test_tree_command_prints_action() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("tree")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dependency tree"));
}

#[test]
fn test_unknown_subcommand_returns_error() {
    Command::new(env!("CARGO_BIN_EXE_pt"))
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicates::str::contains("error"));
}
