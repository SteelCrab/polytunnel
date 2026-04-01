use polytunnel_core::{parse_remove_coordinate, remove_dependency_from_file};
use std::io::Write;
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
