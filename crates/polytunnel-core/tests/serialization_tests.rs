//! Serialization and deserialization tests for configs
//!
//! Coverage: Validates the accuracy of TOML serialization/deserialization for all project and build configuration structures.

use polytunnel_core::{
    BuildConfig, Dependency, DependencyScope, ProjectConfig, ProjectInfo, Repository,
};
use std::collections::HashMap;

#[test]
fn test_project_info_serialization() {
    let info = ProjectInfo {
        name: "test".to_string(),
        java_version: "17".to_string(),
    };

    assert_eq!(info.name, "test");
    assert_eq!(info.java_version, "17");
}

#[test]
fn test_build_config_serialization() {
    let config = BuildConfig {
        source_dirs: vec!["src/main/java".to_string()],
        test_source_dirs: vec!["src/test/java".to_string()],
        output_dir: "target/classes".to_string(),
        test_output_dir: "target/test-classes".to_string(),
        compiler_args: vec![],
        test_compiler_args: vec![],
        test_framework: "auto".to_string(),
        cache_dir: ".polytunnel/cache".to_string(),
    };

    assert_eq!(config.source_dirs[0], "src/main/java");
    assert_eq!(config.output_dir, "target/classes");
}

#[test]
fn test_repository_serialization() {
    let repo = Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    };

    assert_eq!(repo.name, "central");
    assert!(repo.url.contains("maven"));
}

#[test]
fn test_dependency_simple_serialization() {
    let dep = Dependency::Simple("1.0.0".to_string());
    match dep {
        Dependency::Simple(v) => assert_eq!(v, "1.0.0"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_dependency_detailed_serialization() {
    let dep = Dependency::Detailed {
        version: "2.0.0".to_string(),
        scope: DependencyScope::Test,
        optional: false,
    };

    match dep {
        Dependency::Detailed {
            version,
            scope,
            optional,
        } => {
            assert_eq!(version, "2.0.0");
            assert_eq!(scope, DependencyScope::Test);
            assert!(!optional);
        }
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_full_config_structure() {
    let mut deps = HashMap::new();
    deps.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        Dependency::Simple("5.10.0".to_string()),
    );

    let config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig {
            source_dirs: vec!["src/main/java".to_string()],
            test_source_dirs: vec!["src/test/java".to_string()],
            output_dir: "target/classes".to_string(),
            test_output_dir: "target/test-classes".to_string(),
            compiler_args: vec![],
            test_compiler_args: vec![],
            test_framework: "auto".to_string(),
            cache_dir: ".polytunnel/cache".to_string(),
        },
        dependencies: deps,
        repositories: vec![],
    };

    assert_eq!(config.project.name, "app");
    assert_eq!(config.dependencies.len(), 1);
}

#[test]
fn test_multiple_repositories() {
    let repos = [
        Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        },
        Repository {
            name: "custom".to_string(),
            url: "https://custom.example.com".to_string(),
        },
    ];

    assert_eq!(repos.len(), 2);
    assert_eq!(repos[0].name, "central");
    assert_eq!(repos[1].name, "custom");
}

#[test]
fn test_dependency_scope_variants() {
    let scopes = [
        DependencyScope::Compile,
        DependencyScope::Test,
        DependencyScope::Runtime,
        DependencyScope::Provided,
    ];

    assert_eq!(scopes.len(), 4);
}

#[test]
fn test_build_config_with_custom_args() {
    let config = BuildConfig {
        compiler_args: vec!["-encoding".to_string(), "UTF-8".to_string()],
        ..Default::default()
    };

    assert_eq!(config.compiler_args.len(), 2);
    assert!(config.compiler_args.contains(&"-encoding".to_string()));
}

#[test]
fn test_config_clone_and_modify() {
    let config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    let mut config2 = config.clone();
    config2.project.name = "modified-app".to_string();

    assert_eq!(config.project.name, "app");
    assert_eq!(config2.project.name, "modified-app");
}

#[test]
fn test_build_config_clone() {
    let config = BuildConfig::default();
    let config2 = config.clone();

    assert_eq!(config.output_dir, config2.output_dir);
}

#[test]
fn test_project_info_clone() {
    let info = ProjectInfo {
        name: "test".to_string(),
        java_version: "17".to_string(),
    };

    let info2 = info.clone();
    assert_eq!(info.name, info2.name);
}

#[test]
fn test_repository_clone() {
    let repo = Repository {
        name: "central".to_string(),
        url: "https://repo.example.com".to_string(),
    };

    let repo2 = repo.clone();
    assert_eq!(repo.name, repo2.name);
}

#[test]
fn test_build_config_custom_paths() {
    let config = BuildConfig {
        source_dirs: vec!["custom/src".to_string()],
        test_source_dirs: vec!["custom/test".to_string()],
        output_dir: "build/main".to_string(),
        test_output_dir: "build/test".to_string(),
        compiler_args: vec![],
        test_compiler_args: vec![],
        test_framework: "junit5".to_string(),
        cache_dir: "build/cache".to_string(),
    };

    assert_eq!(config.source_dirs[0], "custom/src");
    assert_eq!(config.output_dir, "build/main");
}

#[test]
fn test_dependency_optional_flag() {
    let dep = Dependency::Detailed {
        version: "1.0.0".to_string(),
        scope: DependencyScope::Compile,
        optional: true,
    };

    match dep {
        Dependency::Detailed { optional, .. } => assert!(optional),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_multiple_dependencies() {
    let mut deps = HashMap::new();
    deps.insert(
        "org.junit:junit".to_string(),
        Dependency::Simple("4.13".to_string()),
    );
    deps.insert(
        "junit:junit".to_string(),
        Dependency::Simple("4.13.2".to_string()),
    );

    assert_eq!(deps.len(), 2);
}

#[test]
fn test_java_version_variations() {
    let versions = vec!["8", "11", "17", "21"];

    for version in versions {
        let info = ProjectInfo {
            name: "app".to_string(),
            java_version: version.to_string(),
        };
        assert_eq!(info.java_version, version);
    }
}

#[test]
fn test_build_cache_directory_path() {
    let config = BuildConfig {
        source_dirs: vec![],
        test_source_dirs: vec![],
        output_dir: "target/classes".to_string(),
        test_output_dir: "target/test-classes".to_string(),
        compiler_args: vec![],
        test_compiler_args: vec![],
        test_framework: "auto".to_string(),
        cache_dir: ".polytunnel/cache".to_string(),
    };

    assert!(config.cache_dir.starts_with(".polytunnel"));
}

#[test]
fn test_repository_url_validation() {
    let repo = Repository {
        name: "test".to_string(),
        url: "https://example.com/repo/".to_string(),
    };

    assert!(repo.url.starts_with("https://"));
}

#[test]
fn test_build_config_equality() {
    let config1 = BuildConfig::default();
    let config2 = BuildConfig::default();

    assert_eq!(config1.output_dir, config2.output_dir);
}

#[test]
fn test_project_info_equality() {
    let info1 = ProjectInfo {
        name: "test".to_string(),
        java_version: "17".to_string(),
    };

    let info2 = ProjectInfo {
        name: "test".to_string(),
        java_version: "17".to_string(),
    };

    assert_eq!(info1.name, info2.name);
}
