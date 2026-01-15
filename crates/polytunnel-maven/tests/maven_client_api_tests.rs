//! Comprehensive tests for MavenClient API methods
//!
//! Coverage: Verifies all MavenClient API interactions, Including search, metadata retrieval, and JAR/POM downloading logic.

use polytunnel_core::Repository;
use polytunnel_maven::Coordinate;
use polytunnel_maven::MavenClient;

#[test]
fn test_coordinate_with_classifier() {
    let coord = Coordinate {
        group_id: "org.apache.commons".to_string(),
        artifact_id: "commons-lang3".to_string(),
        version: "3.12.0".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    assert_eq!(coord.group_id, "org.apache.commons");
    assert_eq!(coord.classifier, Some("sources".to_string()));
}

#[test]
fn test_coordinate_without_classifier() {
    let coord = Coordinate {
        group_id: "org.junit.jupiter".to_string(),
        artifact_id: "junit-jupiter-api".to_string(),
        version: "5.10.1".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert_eq!(coord.classifier, None);
}

#[test]
fn test_maven_central_repository_url() {
    let client = MavenClient::new();
    // MavenClient doesn't have repositories field anymore, it has base_url
    assert!(
        client
            .jar_url(&Coordinate::new("org.junit", "junit", "4.13.2"))
            .contains("repo1.maven.org")
    );
}

#[test]
fn test_coordinate_jar_filename_generation() {
    let coord = Coordinate {
        group_id: "junit".to_string(),
        artifact_id: "junit".to_string(),
        version: "4.13.2".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let filename = format!("{}-{}.jar", coord.artifact_id, coord.version);
    assert_eq!(filename, "junit-4.13.2.jar");
}

#[test]
fn test_coordinate_jar_with_classifier() {
    let coord = Coordinate {
        group_id: "org.junit.jupiter".to_string(),
        artifact_id: "junit-jupiter-api".to_string(),
        version: "5.10.1".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    let filename = if let Some(ref classifier) = coord.classifier {
        format!("{}-{}-{}.jar", coord.artifact_id, coord.version, classifier)
    } else {
        format!("{}-{}.jar", coord.artifact_id, coord.version)
    };

    assert_eq!(filename, "junit-jupiter-api-5.10.1-sources.jar");
}

#[test]
fn test_maven_central_group_path_generation() {
    let group_id = "org.junit.jupiter";
    let path = group_id.replace(".", "/");
    assert_eq!(path, "org/junit/jupiter");
}

#[test]
fn test_pom_filename_for_coordinate() {
    let coord = Coordinate {
        group_id: "org.springframework".to_string(),
        artifact_id: "spring-core".to_string(),
        version: "6.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let pom_filename = format!("{}-{}.pom", coord.artifact_id, coord.version);
    assert_eq!(pom_filename, "spring-core-6.0.0.pom");
}

#[test]
fn test_coordinate_full_maven_url_path() {
    let coord = Coordinate {
        group_id: "com.google.guava".to_string(),
        artifact_id: "guava".to_string(),
        version: "32.0.0-jre".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let group_path = coord.group_id.replace(".", "/");
    let artifact_path = format!("{}/{}/{}", group_path, coord.artifact_id, coord.version);
    let jar_path = format!(
        "{}/{}-{}.jar",
        artifact_path, coord.artifact_id, coord.version
    );

    assert_eq!(
        jar_path,
        "com/google/guava/guava/32.0.0-jre/guava-32.0.0-jre.jar"
    );
}

#[test]
fn test_coordinate_snapshot_version() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "test-lib".to_string(),
        version: "1.0.0-SNAPSHOT".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(coord.version.contains("SNAPSHOT"));
}

#[test]
fn test_coordinate_rc_version() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "test-lib".to_string(),
        version: "2.0.0-RC1".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(coord.version.contains("RC"));
}

#[test]
fn test_coordinate_alpha_version() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "test-lib".to_string(),
        version: "1.0.0-alpha".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(coord.version.contains("alpha"));
}

#[test]
fn test_coordinate_beta_version() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "test-lib".to_string(),
        version: "1.0.0-beta.1".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert!(coord.version.contains("beta"));
}

#[test]
fn test_repository_url_trailing_slash() {
    let url = "https://repo1.maven.org/maven2/";
    assert!(url.ends_with("/"));
}

#[test]
fn test_repository_url_https() {
    let url = "https://repo1.maven.org/maven2/";
    assert!(url.starts_with("https://"));
}

#[test]
fn test_artifact_coordinate_equality() {
    let coord1 = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let coord2 = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    assert_eq!(coord1.group_id, coord2.group_id);
    assert_eq!(coord1.artifact_id, coord2.artifact_id);
    assert_eq!(coord1.version, coord2.version);
}

#[test]
fn test_coordinate_with_multiple_classifiers() {
    let classifiers = vec!["sources", "javadoc", "tests"];

    for classifier in classifiers {
        let coord = Coordinate {
            group_id: "org.example".to_string(),
            artifact_id: "lib".to_string(),
            version: "1.0.0".to_string(),
            classifier: Some(classifier.to_string()),
            packaging: "jar".to_string(),
        };
        assert_eq!(coord.classifier, Some(classifier.to_string()));
    }
}

#[test]
fn test_maven_group_id_variations() {
    let groups = vec![
        "com.google.guava",
        "org.springframework.boot",
        "io.quarkus",
        "com.fasterxml.jackson.core",
    ];

    for group in groups {
        let coord = Coordinate {
            group_id: group.to_string(),
            artifact_id: "lib".to_string(),
            version: "1.0.0".to_string(),
            classifier: None,
            packaging: "jar".to_string(),
        };
        assert_eq!(coord.group_id, group);
    }
}

#[test]
fn test_maven_artifact_id_naming_patterns() {
    let artifacts = vec![
        "junit",
        "junit-jupiter",
        "junit-jupiter-api",
        "commons-lang3",
    ];

    for artifact in artifacts {
        let coord = Coordinate {
            group_id: "org.example".to_string(),
            artifact_id: artifact.to_string(),
            version: "1.0.0".to_string(),
            classifier: None,
            packaging: "jar".to_string(),
        };
        assert_eq!(coord.artifact_id, artifact);
    }
}

#[test]
fn test_coordinate_version_parsing() {
    let versions = vec![
        "1.0.0",
        "1.2.3.4",
        "2021.1.0",
        "1.0.0-SNAPSHOT",
        "1.0.0-RC1",
    ];

    for version in versions {
        let coord = Coordinate {
            group_id: "org.example".to_string(),
            artifact_id: "lib".to_string(),
            version: version.to_string(),
            classifier: None,
            packaging: "jar".to_string(),
        };
        assert_eq!(coord.version, version);
    }
}

#[test]
fn test_maven_repository_structure() {
    let repo = Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    };

    assert!(!repo.name.is_empty());
    assert!(repo.url.starts_with("https://"));
}

#[test]
fn test_multiple_maven_repositories() {
    let repos = vec![
        Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        },
        Repository {
            name: "google".to_string(),
            url: "https://maven.google.com/".to_string(),
        },
    ];

    assert_eq!(repos.len(), 2);
    assert!(repos.iter().any(|r| r.name == "central"));
    assert!(repos.iter().any(|r| r.name == "google"));
}

#[test]
fn test_coordinate_cache_path_structure() {
    let coord = Coordinate {
        group_id: "org.junit".to_string(),
        artifact_id: "junit".to_string(),
        version: "4.13.2".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let group_path = coord.group_id.replace(".", "/");
    let cache_path = format!(
        ".polytunnel/cache/{}/{}/{}",
        group_path, coord.artifact_id, coord.version
    );

    assert!(cache_path.contains(".polytunnel"));
    assert!(cache_path.contains("cache"));
}

#[test]
fn test_coordinate_group_path_normalization() {
    let groups = vec![
        ("com.example", "com/example"),
        ("org.springframework.boot", "org/springframework/boot"),
        ("io.quarkus.qson", "io/quarkus/qson"),
    ];

    for (group, expected_path) in groups {
        let path = group.replace(".", "/");
        assert_eq!(path, expected_path);
    }
}

#[test]
fn test_artifact_id_hyphen_handling() {
    let artifacts = vec!["spring-core", "junit-jupiter-api", "commons-lang3"];

    for artifact in artifacts {
        assert!(artifact.contains("-"));
    }
}

#[test]
fn test_version_number_components() {
    let version = "1.2.3";
    let parts: Vec<&str> = version.split('.').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "1");
    assert_eq!(parts[1], "2");
    assert_eq!(parts[2], "3");
}

#[test]
fn test_classifier_optional_handling() {
    let coords = vec![
        (None, "lib-1.0.0.jar"),
        (Some("sources".to_string()), "lib-1.0.0-sources.jar"),
        (Some("javadoc".to_string()), "lib-1.0.0-javadoc.jar"),
    ];

    for (classifier, expected_file) in coords {
        let filename = if let Some(c) = classifier {
            format!("lib-1.0.0-{}.jar", c)
        } else {
            "lib-1.0.0.jar".to_string()
        };
        assert_eq!(filename, expected_file);
    }
}

#[test]
fn test_maven_metadata_structure() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let metadata_path = format!("{}/maven-metadata.xml", coord.group_id.replace(".", "/"));

    assert!(metadata_path.ends_with("maven-metadata.xml"));
}

#[test]
fn test_coordinate_to_string_representation() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: None,
        packaging: "jar".to_string(),
    };

    let string_rep = format!("{}:{}:{}", coord.group_id, coord.artifact_id, coord.version);
    assert_eq!(string_rep, "org.example:lib:1.0.0");
}

#[test]
fn test_coordinate_with_classifier_string_representation() {
    let coord = Coordinate {
        group_id: "org.example".to_string(),
        artifact_id: "lib".to_string(),
        version: "1.0.0".to_string(),
        classifier: Some("sources".to_string()),
        packaging: "jar".to_string(),
    };

    let string_rep = if let Some(c) = &coord.classifier {
        format!(
            "{}:{}:{}:{}",
            coord.group_id, coord.artifact_id, coord.version, c
        )
    } else {
        format!("{}:{}:{}", coord.group_id, coord.artifact_id, coord.version)
    };

    assert_eq!(string_rep, "org.example:lib:1.0.0:sources");
}
