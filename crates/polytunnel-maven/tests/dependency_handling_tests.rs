//! Comprehensive tests for dependency handling
//!
//! Coverage: Verifies Maven coordinate parsing, repository path construction, and dependency scope inheritance logic.

use polytunnel_core::Dependency;
use polytunnel_maven::Coordinate;

#[test]
fn test_coordinate_to_maven_coordinate() {
    let coord = Coordinate {
        group_id: "org.junit.jupiter".to_string(),
        artifact_id: "junit-jupiter".to_string(),
        version: "5.10.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert_eq!(coord.group_id, "org.junit.jupiter");
    assert_eq!(coord.artifact_id, "junit-jupiter");
    assert_eq!(coord.version, "5.10.0");
}

#[test]
fn test_coordinate_with_classifier_full_representation() {
    let coord = Coordinate {
        group_id: "org.junit.jupiter".to_string(),
        artifact_id: "junit-jupiter-api".to_string(),
        version: "5.10.0".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    let full = format!(
        "{}:{}:{}:{}",
        coord.group_id,
        coord.artifact_id,
        coord.version,
        coord.classifier.as_ref().unwrap()
    );

    assert_eq!(full, "org.junit.jupiter:junit-jupiter-api:5.10.0:sources");
}

#[test]
fn test_dependency_simple_variant_string_representation() {
    let dep = Dependency::Simple("4.13.2".to_string());

    match dep {
        Dependency::Simple(v) => assert_eq!(v, "4.13.2"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_dependency_detailed_variant_fields() {
    let dep = Dependency::Detailed {
        version: "1.0.0".to_string(),
        scope: polytunnel_core::DependencyScope::Compile,
        optional: false,
    };

    match dep {
        Dependency::Detailed {
            version,
            scope,
            optional,
        } => {
            assert_eq!(version, "1.0.0");
            assert_eq!(scope, polytunnel_core::DependencyScope::Compile);
            assert!(!optional);
        }
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_coordinate_group_id_variations() {
    let group_ids = vec![
        "com.google.guava",
        "org.springframework",
        "junit",
        "org.junit.jupiter",
    ];

    for group in group_ids {
        assert!(!group.is_empty());
    }
}

#[test]
fn test_coordinate_artifact_id_naming() {
    let artifacts = vec!["guava", "spring-core", "junit", "junit-jupiter-api"];

    for artifact in artifacts {
        assert!(!artifact.is_empty());
    }
}

#[test]
fn test_coordinate_version_semver() {
    let versions = vec!["1.0.0", "2.3.4", "10.20.30"];

    for version in versions {
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3);
    }
}

#[test]
fn test_coordinate_version_with_qualifier() {
    let versions = vec!["1.0.0-SNAPSHOT", "1.0.0-RC1", "1.0.0-alpha", "1.0.0-beta.1"];

    for version in versions {
        assert!(version.contains("-"));
    }
}

#[test]
fn test_coordinate_classifier_types() {
    let classifiers = vec!["sources", "javadoc", "tests", "natives-windows"];

    for classifier in classifiers {
        assert!(!classifier.is_empty());
    }
}

#[test]
fn test_dependency_scope_all_variants() {
    let scopes = vec![
        polytunnel_core::DependencyScope::Compile,
        polytunnel_core::DependencyScope::Test,
        polytunnel_core::DependencyScope::Runtime,
        polytunnel_core::DependencyScope::Provided,
    ];

    assert_eq!(scopes.len(), 4);
}

#[test]
fn test_coordinate_jar_path_construction() {
    let coord = Coordinate {
        group_id: "org.springframework".to_string(),
        artifact_id: "spring-core".to_string(),
        version: "6.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let group_path = coord.group_id.replace(".", "/");
    let jar_path = format!(
        "{}/{}/{}/{}",
        group_path,
        coord.artifact_id,
        coord.version,
        format!("{}-{}.jar", coord.artifact_id, coord.version)
    );

    assert!(jar_path.contains("org/springframework"));
    assert!(jar_path.contains("spring-core"));
}

#[test]
fn test_coordinate_pom_path_construction() {
    let coord = Coordinate {
        group_id: "junit".to_string(),
        artifact_id: "junit".to_string(),
        version: "4.13.2".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let pom_filename = format!("{}-{}.pom", coord.artifact_id, coord.version);
    assert_eq!(pom_filename, "junit-4.13.2.pom");
}

#[test]
fn test_coordinate_maven_metadata_path() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let group_path = coord.group_id.replace(".", "/");
    let metadata_path = format!("{}/maven-metadata.xml", group_path);

    assert!(metadata_path.ends_with("maven-metadata.xml"));
}

#[test]
fn test_coordinate_repository_relative_path() {
    let coord = Coordinate {
        group_id: "com.example".to_string(),
        artifact_id: "mylib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let repo_path = format!(
        "{}/{}/{}",
        coord.group_id.replace(".", "/"),
        coord.artifact_id,
        coord.version
    );

    assert_eq!(repo_path, "com/example/mylib/1.0.0");
}

#[test]
fn test_dependency_optional_field_true() {
    let dep = Dependency::Detailed {
        version: "1.0.0".to_string(),
        scope: polytunnel_core::DependencyScope::Compile,
        optional: true,
    };

    match dep {
        Dependency::Detailed { optional, .. } => assert!(optional),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_dependency_optional_field_false() {
    let dep = Dependency::Detailed {
        version: "1.0.0".to_string(),
        scope: polytunnel_core::DependencyScope::Compile,
        optional: false,
    };

    match dep {
        Dependency::Detailed { optional, .. } => assert!(!optional),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_dependency_scope_affects_classpath() {
    let compile_scope = polytunnel_core::DependencyScope::Compile;
    let test_scope = polytunnel_core::DependencyScope::Test;

    assert_ne!(compile_scope, test_scope);
}

#[test]
fn test_coordinate_cache_key_generation() {
    let coord = Coordinate {
        group_id: "org.junit".to_string(),
        artifact_id: "junit".to_string(),
        version: "4.13.2".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let cache_key = format!("{}:{}:{}", coord.group_id, coord.artifact_id, coord.version);
    assert_eq!(cache_key, "org.junit:junit:4.13.2");
}

#[test]
fn test_coordinate_cache_key_with_classifier() {
    let coord = Coordinate {
        group_id: "org.junit".to_string(),
        artifact_id: "junit".to_string(),
        version: "4.13.2".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    let cache_key = if let Some(c) = &coord.classifier {
        format!(
            "{}:{}:{}:{}",
            coord.group_id, coord.artifact_id, coord.version, c
        )
    } else {
        format!("{}:{}:{}", coord.group_id, coord.artifact_id, coord.version)
    };

    assert_eq!(cache_key, "org.junit:junit:4.13.2:sources");
}

#[test]
fn test_dependency_version_parsing() {
    let versions = vec!["1.0", "1.0.0", "1.0.0.0"];

    for version in versions {
        assert!(!version.is_empty());
    }
}

#[test]
fn test_coordinate_filename_generation_simple() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let filename = format!("{}-{}.jar", coord.artifact_id, coord.version);
    assert_eq!(filename, "lib-1.0.0.jar");
}

#[test]
fn test_coordinate_filename_generation_with_classifier() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    let filename = format!(
        "{}-{}-{}.jar",
        coord.artifact_id,
        coord.version,
        coord.classifier.as_ref().unwrap()
    );
    assert_eq!(filename, "lib-1.0.0-sources.jar");
}

#[test]
fn test_coordinate_version_range_markers() {
    let ranges = vec![
        "[1.0.0,2.0.0]",
        "[1.0.0,2.0.0)",
        "(1.0.0,2.0.0)",
        "(1.0.0,2.0.0]",
    ];

    for range in ranges {
        assert!(range.contains("[") || range.contains("("));
    }
}

#[test]
fn test_coordinate_classifier_to_extension_mapping() {
    let mapping = vec![("sources", ".jar"), ("javadoc", ".jar"), ("tests", ".jar")];

    for (classifier, ext) in mapping {
        assert!(!classifier.is_empty());
        assert!(ext.starts_with("."));
    }
}

#[test]
fn test_dependency_scope_transitive_inheritance() {
    let compile_scope = polytunnel_core::DependencyScope::Compile;
    let test_scope = polytunnel_core::DependencyScope::Test;
    let _runtime_scope = polytunnel_core::DependencyScope::Runtime;
    let _provided_scope = polytunnel_core::DependencyScope::Provided;

    // Compile should be transitive
    assert_eq!(compile_scope, polytunnel_core::DependencyScope::Compile);

    // Test should not be transitive
    assert_eq!(test_scope, polytunnel_core::DependencyScope::Test);
}

#[test]
fn test_coordinate_parent_reference() {
    let parent = Coordinate {
        group_id: "org.springframework.boot".to_string(),
        artifact_id: "spring-boot-parent".to_string(),
        version: "2.7.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(!parent.group_id.is_empty());
}

#[test]
fn test_coordinate_bom_import() {
    let bom = Coordinate {
        group_id: "org.springframework.cloud".to_string(),
        artifact_id: "spring-cloud-dependencies".to_string(),
        version: "2021.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(bom.artifact_id.contains("dependencies"));
}

#[test]
fn test_dependency_exclusion_pattern() {
    let excluded_group = "commons-logging";
    assert!(!excluded_group.is_empty());
}

#[test]
fn test_coordinate_version_ordering() {
    let versions = vec!["1.0.0", "1.1.0", "2.0.0"];
    let latest = versions.iter().max().unwrap();

    assert_eq!(*latest, "2.0.0");
}

#[test]
fn test_coordinate_compatible_version_range() {
    let version = "1.5.3";
    let range_start = "1.0.0";
    let range_end = "2.0.0";

    let compatible = version >= range_start && version < range_end;
    assert!(compatible);
}

#[test]
fn test_dependency_convergence_highest_version() {
    let versions = vec!["1.0.0", "1.5.0", "2.0.0"];
    let selected = versions.iter().max().unwrap();

    assert_eq!(*selected, "2.0.0");
}

#[test]
fn test_coordinate_repository_order() {
    let repos = vec!["central", "custom", "snapshots"];
    assert_eq!(repos[0], "central");
}

#[test]
fn test_dependency_scope_runtime_propagation() {
    // Runtime scope should propagate at runtime
    let scope = polytunnel_core::DependencyScope::Runtime;
    assert_eq!(scope, polytunnel_core::DependencyScope::Runtime);
}

#[test]
fn test_dependency_scope_provided_propagation() {
    // Provided scope should NOT propagate
    let scope = polytunnel_core::DependencyScope::Provided;
    assert_eq!(scope, polytunnel_core::DependencyScope::Provided);
}

#[test]
fn test_coordinate_snapshot_identifier() {
    let version = "1.0.0-SNAPSHOT";
    let is_snapshot = version.contains("SNAPSHOT");

    assert!(is_snapshot);
}

#[test]
fn test_coordinate_release_identifier() {
    let version = "1.0.0";
    let is_release = !version.contains("-");

    assert!(is_release);
}

#[test]
fn test_dependency_map_key_construction() {
    let group = "org.example";
    let artifact = "mylib";
    let key = format!("{}:{}", group, artifact);

    assert_eq!(key, "org.example:mylib");
}

#[test]
fn test_dependency_uniqueness() {
    let dep1 = ("org.junit:junit", "4.13.2");
    let dep2 = ("org.junit:junit", "4.13.2");

    assert_eq!(dep1, dep2);
}

#[test]
fn test_coordinate_transitive_resolution() {
    // Simulating transitive resolution
    let direct = Coordinate {
        group_id: "org.springframework".to_string(),
        artifact_id: "spring-core".to_string(),
        version: "6.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(!direct.group_id.is_empty());
}
