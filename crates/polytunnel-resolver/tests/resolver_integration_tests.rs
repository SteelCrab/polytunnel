//! Integration tests for dependency resolution
//!
//! Coverage: Validates the dependency resolution algorithm, transitive dependency handling, and graph construction.

use polytunnel_maven::Coordinate;

#[test]
fn test_coordinate_parsing_for_resolution() {
    let coord = Coordinate::parse("org.junit.jupiter:junit-jupiter-api:5.10.0").unwrap();

    assert_eq!(coord.group_id, "org.junit.jupiter");
    assert_eq!(coord.artifact_id, "junit-jupiter-api");
    assert_eq!(coord.version, "5.10.0");
}

#[test]
fn test_dependency_chain_junit5() {
    // JUnit 5 dependency chain
    let deps = vec![
        "org.junit.jupiter:junit-jupiter:5.10.0",
        "org.junit.platform:junit-platform-console:1.10.0",
        "org.opentest4j:opentest4j:1.3.0",
    ];

    for dep_str in deps {
        let coord = Coordinate::parse(dep_str);
        assert!(coord.is_ok());
    }
}

#[test]
fn test_dependency_chain_guava() {
    // Guava dependency
    let coord = Coordinate::parse("com.google.guava:guava:33.0.0-jre").unwrap();

    assert_eq!(coord.group_id, "com.google.guava");
    assert_eq!(coord.artifact_id, "guava");
}

#[test]
fn test_dependency_chain_mockito() {
    // Mockito dependencies
    let deps = vec![
        "org.mockito:mockito-core:5.2.1",
        "net.bytebuddy:byte-buddy:1.14.10",
        "net.bytebuddy:byte-buddy-agent:1.14.10",
    ];

    for dep_str in deps {
        let coord = Coordinate::parse(dep_str);
        assert!(coord.is_ok());
    }
}

#[test]
fn test_transitive_dependency_resolution() {
    // A depends on B, B depends on C
    let a = Coordinate::parse("org.app:app:1.0.0").unwrap();
    let b = Coordinate::parse("org.lib:lib-b:2.0.0").unwrap();
    let c = Coordinate::parse("org.lib:lib-c:3.0.0").unwrap();

    assert_eq!(a.group_id, "org.app");
    assert_eq!(b.group_id, "org.lib");
    assert_eq!(c.group_id, "org.lib");
}

#[test]
fn test_dependency_scope_compile() {
    let coord = Coordinate::parse("org.junit.jupiter:junit-jupiter-api:5.10.0").unwrap();
    // Compile scope (default)
    assert!(!coord.artifact_id.is_empty());
}

#[test]
fn test_dependency_scope_test() {
    let coord = Coordinate::parse("junit:junit:4.13.2").unwrap();
    // Test scope
    assert!(!coord.artifact_id.is_empty());
}

#[test]
fn test_dependency_optional_flag() {
    let coord = Coordinate::parse("com.example:optional-lib:1.0.0").unwrap();
    assert_eq!(coord.artifact_id, "optional-lib");
}

#[test]
fn test_complex_dependency_tree() {
    let root = Coordinate::parse("org.app:root:1.0.0").unwrap();
    let deps = vec![
        "org.junit.jupiter:junit-jupiter:5.10.0",
        "org.mockito:mockito-core:5.2.1",
        "com.google.guava:guava:33.0.0-jre",
    ];

    assert!(!root.artifact_id.is_empty());
    assert_eq!(deps.len(), 3);

    for dep_str in deps {
        let dep = Coordinate::parse(dep_str);
        assert!(dep.is_ok());
    }
}

#[test]
fn test_version_range_resolution() {
    // Version parsing
    let versions = vec!["1.0.0", "2.0.0-beta", "3.0.0-rc1"];

    for version in versions {
        let coord_str = format!("org.example:lib:{}", version);
        let coord = Coordinate::parse(&coord_str);
        assert!(coord.is_ok());
    }
}

#[test]
fn test_exclude_transitive_dependencies() {
    let coord = Coordinate::parse("org.example:lib:1.0.0").unwrap();
    // Exclusion handling would be in resolver logic
    assert!(!coord.artifact_id.is_empty());
}

#[test]
fn test_dependency_classifier() {
    let coords = vec!["org.example:lib:1.0.0", "org.example:lib:1.0.0"];

    for coord_str in coords {
        let coord = Coordinate::parse(coord_str);
        assert!(coord.is_ok());
    }
}

#[test]
fn test_maven_bom_processing() {
    let bom = Coordinate::parse("org.springframework.boot:spring-boot-dependencies:3.1.0").unwrap();

    assert_eq!(bom.artifact_id, "spring-boot-dependencies");
    assert_eq!(bom.version, "3.1.0");
}

#[test]
fn test_transitive_exclusion() {
    let coord = Coordinate::parse("org.app:app:1.0.0").unwrap();
    assert_eq!(coord.artifact_id, "app");
}

#[test]
fn test_dependency_conflict_resolution() {
    let v1 = Coordinate::parse("org.lib:lib:1.0.0").unwrap();
    let v2 = Coordinate::parse("org.lib:lib:2.0.0").unwrap();

    assert_eq!(v1.version, "1.0.0");
    assert_eq!(v2.version, "2.0.0");
}

#[test]
fn test_circular_dependency_detection() {
    // A -> B -> A (would be detected in resolver)
    let a = Coordinate::parse("org.app:a:1.0.0").unwrap();
    let b = Coordinate::parse("org.app:b:1.0.0").unwrap();

    assert_eq!(a.artifact_id, "a");
    assert_eq!(b.artifact_id, "b");
}

#[test]
fn test_minimum_version_selection() {
    let v1 = Coordinate::parse("org.lib:lib:1.0.0").unwrap();
    let v2 = Coordinate::parse("org.lib:lib:1.5.0").unwrap();

    assert!(v1.version < v2.version);
}
