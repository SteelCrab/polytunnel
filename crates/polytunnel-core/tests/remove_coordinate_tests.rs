use polytunnel_core::{
    CoreError, finalize_backup_write, parse_remove_coordinate, remove_dependency_from_file,
};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use tempfile::NamedTempFile;

#[test]
fn valid_coordinate() {
    let result = parse_remove_coordinate("com.google.guava:guava").unwrap();
    assert_eq!(result, "com.google.guava:guava");
}

#[test]
fn single_part_fails() {
    let result = parse_remove_coordinate("guava");
    assert!(result.is_err());
}

#[test]
fn three_parts_with_version_fails() {
    let result = parse_remove_coordinate("com.google.guava:guava:33.0.0");
    assert!(result.is_err());
}

#[test]
fn empty_parts_fails() {
    let result = parse_remove_coordinate(":guava");
    assert!(result.is_err());

    let result = parse_remove_coordinate("com.google.guava:");
    assert!(result.is_err());
}

#[test]
fn remove_cleans_up_backup_on_success() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    writeln!(
        file,
        r#"[project]
name = "test"

[dependencies]
"com.google.guava:guava" = "33.0.0"
"#
    )
    .unwrap();

    let path = file.path().to_path_buf();
    let backup_path = path.with_extension("toml.bak");

    remove_dependency_from_file(&path, "com.google.guava:guava").unwrap();

    assert!(!backup_path.exists(), "backup should be removed on success");

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(!content.contains("guava"));
}

#[test]
fn remove_preserves_file_on_not_found_error() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    let original = r#"[project]
name = "test"

[dependencies]
"com.google.guava:guava" = "33.0.0"
"#;
    write!(file, "{original}").unwrap();

    let path = file.path().to_path_buf();

    let result = remove_dependency_from_file(&path, "org.apache:commons");
    assert!(result.is_err());

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(
        content.contains("guava"),
        "original content should be intact"
    );
}

#[test]
fn remove_restores_from_backup_on_write_failure() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    let original = r#"[project]
name = "test"

[dependencies]
"com.google.guava:guava" = "33.0.0"
"#;
    write!(file, "{original}").unwrap();

    let path = file.path().to_path_buf();
    let backup_path = path.with_extension("toml.bak");

    // Make file read-only so write fails after backup is created
    let perms = std::fs::Permissions::from_mode(0o444);
    std::fs::set_permissions(&path, perms).unwrap();

    let result = remove_dependency_from_file(&path, "com.google.guava:guava");
    assert!(result.is_err(), "write to read-only file should fail");

    // Restore write permission for cleanup
    let perms = std::fs::Permissions::from_mode(0o644);
    let _ = std::fs::set_permissions(&path, perms);

    assert!(
        !backup_path.exists(),
        "backup should be cleaned up after rollback"
    );
}

#[test]
fn remove_does_not_clobber_existing_backup() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    writeln!(
        file,
        r#"[project]
name = "test"

[dependencies]
"com.google.guava:guava" = "33.0.0"
"#
    )
    .unwrap();

    let path = file.path().to_path_buf();
    let existing_backup = path.with_extension("toml.bak");

    // Create a pre-existing backup with unrelated content
    std::fs::write(&existing_backup, "do not touch").unwrap();

    remove_dependency_from_file(&path, "com.google.guava:guava").unwrap();

    // Pre-existing backup must survive
    let backup_content = std::fs::read_to_string(&existing_backup).unwrap();
    assert_eq!(backup_content, "do not touch");

    // Clean up
    let _ = std::fs::remove_file(&existing_backup);
}

#[test]
fn remove_preserves_multiple_existing_backups() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    writeln!(
        file,
        r#"[project]
name = "test"

[dependencies]
"com.google.guava:guava" = "33.0.0"
"#
    )
    .unwrap();

    let path = file.path().to_path_buf();
    let bak = path.with_extension("toml.bak");
    let bak1 = std::path::PathBuf::from(format!("{}.1", bak.to_string_lossy()));

    // Two pre-existing backups force unique_backup_path to iterate past .bak.1
    std::fs::write(&bak, "first").unwrap();
    std::fs::write(&bak1, "second").unwrap();

    remove_dependency_from_file(&path, "com.google.guava:guava").unwrap();

    assert_eq!(std::fs::read_to_string(&bak).unwrap(), "first");
    assert_eq!(std::fs::read_to_string(&bak1).unwrap(), "second");

    let _ = std::fs::remove_file(&bak);
    let _ = std::fs::remove_file(&bak1);
}

#[test]
fn finalize_returns_write_error_when_rollback_succeeds() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    writeln!(file, "partially-modified content to be rolled back").unwrap();
    let path = file.path().to_path_buf();

    let backup = path.with_extension("toml.bak");
    let original = "original content preserved in backup";
    std::fs::write(&backup, original).unwrap();

    let injected = std::io::Error::other("simulated write failure");
    let result = finalize_backup_write(&path, &backup, Err(injected));

    let err = result.expect_err("original write error must propagate");
    assert!(!matches!(err, CoreError::RollbackFailed { .. }));

    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        original,
        "path must be restored from backup"
    );
    assert!(
        !backup.exists(),
        "backup should be cleaned up after successful rollback"
    );
}

#[test]
fn finalize_removes_backup_on_write_success() {
    let mut file = NamedTempFile::with_suffix(".toml").unwrap();
    writeln!(file, "already-written successful content").unwrap();
    let path = file.path().to_path_buf();

    let backup = path.with_extension("toml.bak");
    std::fs::write(&backup, "stale backup").unwrap();

    finalize_backup_write(&path, &backup, Ok(())).unwrap();

    assert!(
        !backup.exists(),
        "backup should be removed after successful write"
    );
}
