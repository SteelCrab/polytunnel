//! Tests for parse_add_coordinate and add_dependency_to_file

use polytunnel_core::parse_add_coordinate;

#[test]
fn test_parse_valid_coordinate() {
    let (ga, ver) = parse_add_coordinate("com.google.guava:guava:33.0.0-jre").unwrap();
    assert_eq!(ga, "com.google.guava:guava");
    assert_eq!(ver, "33.0.0-jre");
}

#[test]
fn test_parse_coordinate_missing_version() {
    assert!(parse_add_coordinate("com.google.guava:guava").is_err());
}

#[test]
fn test_parse_coordinate_empty_parts() {
    assert!(parse_add_coordinate("::").is_err());
    assert!(parse_add_coordinate(":artifact:1.0").is_err());
    assert!(parse_add_coordinate("group::1.0").is_err());
    assert!(parse_add_coordinate("group:artifact:").is_err());
}

#[test]
fn test_parse_coordinate_single_part() {
    assert!(parse_add_coordinate("just-a-name").is_err());
}

#[test]
fn test_parse_coordinate_too_many_parts() {
    // 4+ parts should fail (we only accept groupId:artifactId:version)
    assert!(parse_add_coordinate("a:b:c:d").is_err());
}

// --- add_dependency_to_file tests ---

use polytunnel_core::{DependencyScope, add_dependency_to_file};
use std::fs;

#[test]
fn test_add_dependency_with_runtime_scope() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("polytunnel.toml");
    fs::write(&path, "[project]\nname = \"demo\"\njava_version = \"17\"\n").unwrap();

    add_dependency_to_file(
        &path,
        "com.example:lib",
        "2.0.0",
        Some(DependencyScope::Runtime),
    )
    .unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("com.example:lib"));
    assert!(content.contains("runtime"));
}

#[test]
fn test_add_dependency_with_test_scope() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("polytunnel.toml");
    fs::write(&path, "[project]\nname = \"demo\"\njava_version = \"17\"\n").unwrap();

    add_dependency_to_file(
        &path,
        "org.junit:junit",
        "5.10.0",
        Some(DependencyScope::Test),
    )
    .unwrap();

    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("org.junit:junit"));
    assert!(content.contains("test"));
}

#[test]
fn test_add_dependency_duplicate_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("polytunnel.toml");
    fs::write(
        &path,
        "[project]\nname = \"demo\"\njava_version = \"17\"\n\n[dependencies]\n\"com.example:lib\" = \"1.0.0\"\n",
    )
    .unwrap();

    let result = add_dependency_to_file(&path, "com.example:lib", "2.0.0", None);
    assert!(result.is_err());
}
