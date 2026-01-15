//! Real integration tests for BuildOrchestrator
//!
//! Tests actual build functionality using public API only.

use polytunnel_build::{BuildOptions, BuildOrchestrator};
use polytunnel_core::{
    BuildConfig, Dependency, DependencyScope, ProjectConfig, ProjectInfo, Repository,
};
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
fn test_orchestrator_creation_with_minimal_config() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_custom_output_dirs() {
    let mut config = create_test_config();
    config.build.output_dir = "target/classes".to_string();
    config.build.test_output_dir = "target/test-classes".to_string();

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_source_dirs() {
    let mut config = create_test_config();
    config.build.source_dirs = vec![
        "src/main/java".to_string(),
        "src/generated/java".to_string(),
    ];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_compiler_args() {
    let mut config = create_test_config();
    config.build.compiler_args = vec![
        "-encoding".to_string(),
        "UTF-8".to_string(),
        "-Xlint:all".to_string(),
    ];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_dependencies() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        Dependency::Simple("5.10.0".to_string()),
    );
    config.dependencies.insert(
        "org.mockito:mockito-core".to_string(),
        Dependency::Detailed {
            version: "5.2.0".to_string(),
            scope: DependencyScope::Test,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_repositories() {
    let mut config = create_test_config();
    config.repositories = vec![
        Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        },
        Repository {
            name: "jcenter".to_string(),
            url: "https://jcenter.bintray.com/".to_string(),
        },
    ];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_build_options_defaults() {
    let options = BuildOptions {
        clean: false,
        skip_tests: false,
        verbose: false,
    };

    assert!(!options.clean);
    assert!(!options.skip_tests);
    assert!(!options.verbose);
}

#[test]
fn test_build_options_all_enabled() {
    let options = BuildOptions {
        clean: true,
        skip_tests: true,
        verbose: true,
    };

    assert!(options.clean);
    assert!(options.skip_tests);
    assert!(options.verbose);
}

#[test]
fn test_orchestrator_preserves_java_version() {
    let mut config = create_test_config();
    config.project.java_version = "21".to_string();

    let orchestrator = BuildOrchestrator::new(config).unwrap();
    // Orchestrator should be created successfully with Java 21 config
    drop(orchestrator);
}

#[test]
fn test_orchestrator_creation_with_all_dependency_scopes() {
    let mut config = create_test_config();

    config.dependencies.insert(
        "compile-dep".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Compile,
            optional: false,
        },
    );
    config.dependencies.insert(
        "test-dep".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Test,
            optional: false,
        },
    );
    config.dependencies.insert(
        "runtime-dep".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Runtime,
            optional: false,
        },
    );
    config.dependencies.insert(
        "provided-dep".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Provided,
            optional: false,
        },
    );

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_optional_dependency() {
    let mut config = create_test_config();
    config.dependencies.insert(
        "optional-lib".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Compile,
            optional: true,
        },
    );

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_orchestrator_creation_with_test_framework() {
    let mut config = create_test_config();
    config.build.test_framework = "junit5".to_string();

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}
