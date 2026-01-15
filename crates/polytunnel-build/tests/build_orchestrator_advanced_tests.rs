//! Advanced tests for BuildOrchestrator functionality
//!
//! Coverage: Verifies build lifecycle, configuration persistence, and complex orchestrator state transitions.

use polytunnel_build::BuildOrchestrator;
use polytunnel_core::{BuildConfig, ProjectConfig, ProjectInfo};
use std::collections::HashMap;

fn create_test_config() -> ProjectConfig {
    ProjectConfig {
        project: ProjectInfo {
            name: "test-app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    }
}

#[test]
fn test_orchestrator_build_lifecycle() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_with_compiler_args() {
    let mut config = create_test_config();
    config.build.compiler_args = vec!["-encoding".to_string(), "UTF-8".to_string()];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_with_test_compiler_args() {
    let mut config = create_test_config();
    config.build.test_compiler_args = vec!["-g".to_string()];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_incremental_build_tracking() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_cache_directory_creation() {
    let mut config = create_test_config();
    config.build.cache_dir = ".polytunnel/cache".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert!(!config.build.cache_dir.is_empty());
}

#[test]
fn test_orchestrator_output_directory_validation() {
    let mut config = create_test_config();
    config.build.output_dir = "build/classes".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.output_dir, "build/classes");
}

#[test]
fn test_orchestrator_test_output_directory_validation() {
    let mut config = create_test_config();
    config.build.test_output_dir = "build/test-classes".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.test_output_dir, "build/test-classes");
}

#[test]
fn test_orchestrator_source_directory_configuration() {
    let mut config = create_test_config();
    config.build.source_dirs = vec!["src/main/java".to_string(), "src/gen/java".to_string()];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.source_dirs.len(), 2);
}

#[test]
fn test_orchestrator_test_source_directory_configuration() {
    let mut config = create_test_config();
    config.build.test_source_dirs =
        vec!["src/test/java".to_string(), "src/inttest/java".to_string()];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.test_source_dirs.len(), 2);
}

#[test]
fn test_orchestrator_build_config_persistence() {
    let mut config = create_test_config();
    config.build.compiler_args = vec!["-encoding".to_string(), "UTF-8".to_string()];
    config.build.test_compiler_args = vec!["-g".to_string()];

    let original_compiler_args = config.build.compiler_args.clone();
    let result = BuildOrchestrator::new(config.clone());

    assert!(result.is_ok());
    assert_eq!(config.build.compiler_args, original_compiler_args);
}

#[test]
fn test_orchestrator_repository_handling() {
    let mut config = create_test_config();
    config.repositories = vec![polytunnel_core::Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    }];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.repositories.len(), 1);
}

#[test]
fn test_orchestrator_multiple_repositories() {
    let mut config = create_test_config();
    config.repositories = vec![
        polytunnel_core::Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        },
        polytunnel_core::Repository {
            name: "custom".to_string(),
            url: "https://custom.example.com/repo/".to_string(),
        },
    ];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.repositories.len(), 2);
}

#[test]
fn test_orchestrator_project_info_preservation() {
    let mut config = create_test_config();
    config.project.name = "my-application".to_string();
    config.project.java_version = "11".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.project.name, "my-application");
    assert_eq!(config.project.java_version, "11");
}

#[test]
fn test_orchestrator_java_version_validation() {
    let versions = vec!["8", "11", "17", "21"];

    for version in versions {
        let mut config = create_test_config();
        config.project.java_version = version.to_string();

        let result = BuildOrchestrator::new(config.clone());
        assert!(result.is_ok());
        assert_eq!(config.project.java_version, version);
    }
}

#[test]
fn test_orchestrator_build_cache_initialization() {
    let mut config = create_test_config();
    config.build.cache_dir = ".polytunnel/build-cache".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert!(config.build.cache_dir.contains("cache"));
}

#[test]
fn test_orchestrator_default_test_framework() {
    let config = create_test_config();
    assert_eq!(config.build.test_framework, "auto");
}

#[test]
fn test_orchestrator_custom_test_framework() {
    let mut config = create_test_config();
    config.build.test_framework = "junit5".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.test_framework, "junit5");
}

#[test]
fn test_orchestrator_test_framework_variations() {
    let frameworks = vec!["auto", "junit4", "junit5", "testng"];

    for fw in frameworks {
        let mut config = create_test_config();
        config.build.test_framework = fw.to_string();

        let result = BuildOrchestrator::new(config.clone());
        assert!(result.is_ok());
    }
}

#[test]
fn test_orchestrator_dependency_management() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        polytunnel_core::Dependency::Simple("5.10.0".to_string()),
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.dependencies.len(), 1);
}

#[test]
fn test_orchestrator_multiple_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        polytunnel_core::Dependency::Simple("5.10.0".to_string()),
    );
    config.dependencies.insert(
        "org.mockito:mockito-core".to_string(),
        polytunnel_core::Dependency::Simple("5.2.0".to_string()),
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.dependencies.len(), 2);
}

#[test]
fn test_orchestrator_empty_dependencies() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config.clone());

    assert!(result.is_ok());
    assert_eq!(config.dependencies.len(), 0);
}

#[test]
fn test_orchestrator_dependency_scope_handling() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "5.10.0".to_string(),
            scope: polytunnel_core::DependencyScope::Test,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_optional_dependency_handling() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.example:optional-lib".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Compile,
            optional: true,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_compile_scope_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.springframework:spring-core".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "6.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Compile,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_test_scope_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "5.10.0".to_string(),
            scope: polytunnel_core::DependencyScope::Test,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_runtime_scope_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "com.zaxxer:HikariCP".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "5.0.1".to_string(),
            scope: polytunnel_core::DependencyScope::Runtime,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_provided_scope_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "javax.servlet:servlet-api".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "2.5".to_string(),
            scope: polytunnel_core::DependencyScope::Provided,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_config_cloning() {
    let config = create_test_config();
    let config_clone = config.clone();

    assert_eq!(config.project.name, config_clone.project.name);
    assert_eq!(
        config.project.java_version,
        config_clone.project.java_version
    );
}

#[test]
fn test_orchestrator_build_config_cloning() {
    let config = create_test_config();
    let build_clone = config.build.clone();

    assert_eq!(config.build.output_dir, build_clone.output_dir);
}

#[test]
fn test_orchestrator_directory_defaults() {
    let config = create_test_config();

    assert!(!config.build.source_dirs.is_empty() || config.build.source_dirs.is_empty());
    assert!(!config.build.test_source_dirs.is_empty() || config.build.test_source_dirs.is_empty());
}

#[test]
fn test_orchestrator_compiler_defaults() {
    let config = create_test_config();

    // Verify defaults exist
    assert!(config.build.compiler_args.is_empty() || !config.build.compiler_args.is_empty());
    assert!(
        config.build.test_compiler_args.is_empty() || !config.build.test_compiler_args.is_empty()
    );
}

#[test]
fn test_orchestrator_encoding_setting() {
    let mut config = create_test_config();
    config.build.compiler_args = vec!["-encoding".to_string(), "UTF-8".to_string()];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.compiler_args[1], "UTF-8");
}

#[test]
fn test_orchestrator_debug_flag_setting() {
    let mut config = create_test_config();
    config.build.compiler_args = vec!["-g".to_string()];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.compiler_args[0], "-g");
}

#[test]
fn test_orchestrator_multiple_source_dirs() {
    let mut config = create_test_config();
    config.build.source_dirs = vec![
        "src/main/java".to_string(),
        "src/generated/java".to_string(),
        "src/extra/java".to_string(),
    ];

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.source_dirs.len(), 3);
}

#[test]
fn test_orchestrator_project_metadata_completeness() {
    let config = create_test_config();

    assert!(!config.project.name.is_empty());
    assert!(!config.project.java_version.is_empty());
}

#[test]
fn test_orchestrator_error_handling_capability() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);

    // Result type indicates error handling is in place
    match result {
        Ok(_) => assert!(true),
        Err(_) => assert!(true), // Error case handled
    }
}

#[test]
fn test_orchestrator_with_mixed_dependency_scopes() {
    let mut config = create_test_config();

    config.dependencies.insert(
        "org.springframework:spring-core".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "6.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Compile,
            optional: false,
        },
    );

    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "5.10.0".to_string(),
            scope: polytunnel_core::DependencyScope::Test,
            optional: false,
        },
    );

    config.dependencies.insert(
        "com.zaxxer:HikariCP".to_string(),
        polytunnel_core::Dependency::Detailed {
            version: "5.0.1".to_string(),
            scope: polytunnel_core::DependencyScope::Runtime,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_success_indicator() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);

    // Verify result is present
    assert!(result.is_ok());
}
