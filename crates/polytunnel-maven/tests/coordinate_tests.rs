//! Tests for coordinate module
//!
//! Coverage: Ensures correct parsing, validation, and string representation of Maven Group:Artifact:Version:Packaging coordinates.

use polytunnel_maven::Coordinate;

#[test]
fn test_parse_gav() {
    let coord = Coordinate::parse("org.slf4j:slf4j-api:2.0.9").unwrap();
    assert_eq!(coord.group_id, "org.slf4j");
    assert_eq!(coord.artifact_id, "slf4j-api");
    assert_eq!(coord.version, "2.0.9");
}

#[test]
fn test_parse_gavp() {
    let coord = Coordinate::parse("org.slf4j:slf4j-api:pom:2.0.9").unwrap();
    assert_eq!(coord.packaging, "pom");
    assert_eq!(coord.version, "2.0.9");
}

#[test]
fn test_parse_invalid() {
    let result = Coordinate::parse("invalid");
    assert!(result.is_err());
}

#[test]
fn test_repo_path() {
    let coord = Coordinate::new("org.slf4j", "slf4j-api", "2.0.9");
    assert_eq!(coord.repo_path(), "org/slf4j/slf4j-api/2.0.9");
}

#[test]
fn test_jar_filename() {
    let coord = Coordinate::new("org.slf4j", "slf4j-api", "2.0.9");
    assert_eq!(coord.jar_filename(), "slf4j-api-2.0.9.jar");
}

#[test]
fn test_pom_filename() {
    let coord = Coordinate::new("org.slf4j", "slf4j-api", "2.0.9");
    assert_eq!(coord.pom_filename(), "slf4j-api-2.0.9.pom");
}

#[test]
fn test_display() {
    let coord = Coordinate::new("org.slf4j", "slf4j-api", "2.0.9");
    assert_eq!(format!("{}", coord), "org.slf4j:slf4j-api:2.0.9");
}
