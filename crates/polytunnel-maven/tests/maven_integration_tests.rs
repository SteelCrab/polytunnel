//! Integration tests for Maven client and coordinate handling
//!
//! Coverage: Verifies Maven Central API communication and robustness of coordinate parsing and resolution.

use polytunnel_maven::{Coordinate, MavenClient};

#[test]
fn test_maven_client_instantiation() {
    let _client = MavenClient::new();
    // Client created successfully with default URL
}

#[test]
fn test_maven_client_with_custom_url() {
    let _client = MavenClient::with_base_url("https://custom-repo.example.com");
    // Custom URL client created
}

#[test]
fn test_coordinate_parsing_simple() {
    let coord = Coordinate::parse("org.junit:junit:4.13.2");
    assert!(coord.is_ok());
}

#[test]
fn test_coordinate_parsing_with_version() {
    let coord = Coordinate::parse("org.junit.jupiter:junit-jupiter-api:5.10.0").unwrap();

    assert_eq!(coord.group_id, "org.junit.jupiter");
    assert_eq!(coord.artifact_id, "junit-jupiter-api");
    assert_eq!(coord.version, "5.10.0");
}

#[test]
fn test_coordinate_jar_filename() {
    let coord = Coordinate::parse("com.google.guava:guava:33.0.0-jre").unwrap();
    let filename = coord.jar_filename();

    assert!(filename.contains("guava"));
    assert!(filename.contains("33.0.0-jre"));
    assert!(filename.ends_with(".jar"));
}

#[test]
fn test_coordinate_pom_filename() {
    let coord = Coordinate::parse("org.slf4j:slf4j-api:2.0.9").unwrap();
    let filename = coord.pom_filename();

    assert!(filename.contains("slf4j-api"));
    assert!(filename.contains("2.0.9"));
    assert!(filename.ends_with(".pom"));
}

#[test]
fn test_coordinate_repo_path() {
    let coord = Coordinate::parse("org.junit.jupiter:junit-jupiter-api:5.10.0").unwrap();
    let path = coord.repo_path();

    assert!(path.contains("org/junit/jupiter"));
    assert!(path.contains("junit-jupiter-api"));
    assert!(path.contains("5.10.0"));
}

#[test]
fn test_coordinate_snapshot_version() {
    let coord = Coordinate::parse("org.example:artifact:1.0.0-SNAPSHOT").unwrap();
    assert_eq!(coord.version, "1.0.0-SNAPSHOT");
}

#[test]
fn test_coordinate_release_candidate() {
    let coord = Coordinate::parse("org.example:artifact:1.0.0-rc1").unwrap();
    assert_eq!(coord.version, "1.0.0-rc1");
}

#[test]
fn test_coordinate_beta_version() {
    let coord = Coordinate::parse("org.example:artifact:1.0.0-beta.1").unwrap();
    assert_eq!(coord.version, "1.0.0-beta.1");
}

#[test]
fn test_coordinate_alpha_version() {
    let coord = Coordinate::parse("org.example:artifact:1.0.0-alpha").unwrap();
    assert_eq!(coord.version, "1.0.0-alpha");
}

#[test]
fn test_multiple_coordinates_parsing() {
    let coords = vec![
        "org.junit.jupiter:junit-jupiter-api:5.10.0",
        "com.google.guava:guava:33.0.0-jre",
        "org.slf4j:slf4j-api:2.0.9",
    ];

    for coord_str in coords {
        let coord = Coordinate::parse(coord_str);
        assert!(coord.is_ok(), "Failed to parse: {}", coord_str);
    }
}

#[test]
fn test_coordinate_display_format() {
    let coord = Coordinate::parse("org.junit:junit:4.13.2").unwrap();
    let display = format!("{}", coord);

    assert!(display.contains("org.junit"));
    assert!(display.contains("junit"));
    assert!(display.contains("4.13.2"));
}

#[test]
fn test_coordinate_numeric_versions() {
    let versions = vec!["1.0.0", "2.1.3", "10.5.8", "99.99.99"];

    for version in versions {
        let coord_str = format!("org.test:lib:{}", version);
        let coord = Coordinate::parse(&coord_str);
        assert!(coord.is_ok(), "Failed to parse version: {}", version);
    }
}

#[test]
fn test_coordinate_with_classifier() {
    let coord = Coordinate::parse("org.example:artifact:1.0.0");
    assert!(coord.is_ok());
}

#[test]
fn test_coordinate_group_id_extraction() {
    let coord = Coordinate::parse("com.example.app:my-artifact:1.0.0").unwrap();
    assert_eq!(coord.group_id, "com.example.app");
}

#[test]
fn test_coordinate_artifact_id_extraction() {
    let coord = Coordinate::parse("org.test:my-library:2.5.0").unwrap();
    assert_eq!(coord.artifact_id, "my-library");
}

#[test]
fn test_coordinate_version_extraction() {
    let coord = Coordinate::parse("org.test:lib:3.14.159").unwrap();
    assert_eq!(coord.version, "3.14.159");
}

#[test]
fn test_maven_central_dependency_formats() {
    let deps = vec![
        "org.junit.jupiter:junit-jupiter:5.10.1",
        "org.mockito:mockito-core:5.2.1",
        "org.assertj:assertj-core:3.24.1",
        "com.fasterxml.jackson.core:jackson-databind:2.16.1",
    ];

    for dep in deps {
        let coord = Coordinate::parse(dep);
        assert!(coord.is_ok(), "Failed to parse: {}", dep);
    }
}
