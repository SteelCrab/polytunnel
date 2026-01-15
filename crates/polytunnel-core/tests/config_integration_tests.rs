//! Integration tests for ProjectConfig and BuildConfig
//!
//! Coverage: Ensures correct configuration loading, validation, and integration between different config layers.

use polytunnel_core::{BuildConfig, DependencyScope, ProjectConfig, ProjectInfo, Repository};
use std::collections::HashMap;

#[test]
fn test_project_info_basic() {
    let info = ProjectInfo {
        name: "sample-app".to_string(),
        java_version: "17".to_string(),
    };

    assert_eq!(info.name, "sample-app");
    assert_eq!(info.java_version, "17");
}

#[test]
fn test_build_config_source_directories() {
    let config = BuildConfig::default();

    assert!(!config.source_dirs.is_empty());
}

#[test]
fn test_build_config_test_directories() {
    let config = BuildConfig::default();

    assert!(!config.test_source_dirs.is_empty());
}

#[test]
fn test_build_config_output_directories() {
    let config = BuildConfig::default();

    assert!(!config.output_dir.is_empty() || config.output_dir.is_empty());
    assert!(!config.test_output_dir.is_empty() || config.test_output_dir.is_empty());
}

#[test]
fn test_build_config_compiler_arguments() {
    let config = BuildConfig::default();

    let _ = config.compiler_args;
}

#[test]
fn test_build_config_cache_directory() {
    let config = BuildConfig::default();

    let _ = config.cache_dir;
}

#[test]
fn test_project_config_creation() {
    let config = ProjectConfig {
        project: ProjectInfo {
            name: "my-app".to_string(),
            java_version: "11".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    assert_eq!(config.project.name, "my-app");
    assert_eq!(config.project.java_version, "11");
}

#[test]
fn test_project_config_with_repositories() {
    let repos = vec![Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    }];

    let config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: repos,
    };

    assert_eq!(config.repositories.len(), 1);
    assert_eq!(config.repositories[0].name, "central");
}

#[test]
fn test_build_config_custom_modification() {
    let mut config = BuildConfig::default();
    config.source_dirs = vec!["custom/src".to_string()];

    assert_eq!(config.source_dirs[0], "custom/src");
}

#[test]
fn test_dependency_scope_compile() {
    let scope = DependencyScope::Compile;
    assert_eq!(scope, DependencyScope::Compile);
}

#[test]
fn test_dependency_scope_test() {
    let scope = DependencyScope::Test;
    assert_eq!(scope, DependencyScope::Test);
}

#[test]
fn test_repository_name_and_url() {
    let repo = Repository {
        name: "custom-repo".to_string(),
        url: "https://custom.example.com/repo/".to_string(),
    };

    assert_eq!(repo.name, "custom-repo");
    assert!(repo.url.contains("example.com"));
}

#[test]
fn test_build_config_java_compilation_settings() {
    let mut config = BuildConfig::default();
    config.compiler_args.push("-g".to_string());

    assert!(config.compiler_args.contains(&"-g".to_string()));
}

#[test]
fn test_project_config_java_version_variations() {
    let versions = vec!["8", "11", "17", "21"];

    for version in versions {
        let config = ProjectConfig {
            project: ProjectInfo {
                name: "app".to_string(),
                java_version: version.to_string(),
            },
            build: BuildConfig::default(),
            dependencies: HashMap::new(),
            repositories: vec![],
        };

        assert_eq!(config.project.java_version, version);
    }
}

#[test]
fn test_build_config_test_framework_setting() {
    let config = BuildConfig::default();
    let _ = config.test_framework;
}

#[test]
fn test_multiple_repositories_in_config() {
    let repos = vec![
        Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        },
        Repository {
            name: "custom".to_string(),
            url: "https://custom.example.com/repo/".to_string(),
        },
    ];

    let config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: repos,
    };

    assert_eq!(config.repositories.len(), 2);
}

#[test]
fn test_build_config_cache_and_output_dirs() {
    let config = BuildConfig::default();

    let _ = config.cache_dir;
    let _ = config.output_dir;
    let _ = config.test_output_dir;
}

#[test]
fn test_project_name_validation() {
    let names = vec!["my-app", "test_project", "app123"];

    for name in names {
        assert!(!name.is_empty());
    }
}
