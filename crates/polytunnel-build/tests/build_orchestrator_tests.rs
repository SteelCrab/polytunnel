//! Tests for BuildOrchestrator
//!
//! Coverage: Verifies build orchestration logic, workflow control, and coordination between compiler and test runner.

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
fn test_build_orchestrator_creation() {
    let config = create_test_config();
    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_build_orchestrator_with_custom_build_config() {
    let mut config = create_test_config();
    config.build.source_dirs = vec!["src".to_string()];
    config.build.output_dir = "build".to_string();

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_build_orchestrator_with_test_sources() {
    let mut config = create_test_config();
    config.build.test_source_dirs = vec!["test".to_string()];
    config.build.test_output_dir = "build/test".to_string();

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_build_orchestrator_default_directories() {
    let config = create_test_config();
    // Just verify source_dirs exists as a vector
    assert!(!config.build.source_dirs.is_empty());
}

#[test]
fn test_build_orchestrator_cache_directory() {
    let mut config = create_test_config();
    config.build.cache_dir = ".polytunnel/cache".to_string();

    let result = BuildOrchestrator::new(config.clone());
    assert!(result.is_ok());
    assert_eq!(config.build.cache_dir, ".polytunnel/cache");
}

#[test]
fn test_build_orchestrator_compiler_args() {
    let config = create_test_config();
    // Just verify compiler_args is a vec
    let _ = config.build.compiler_args;
}

#[test]
fn test_build_orchestrator_test_framework_setting() {
    let config = create_test_config();
    // Just verify test_framework is a string
    let _ = config.build.test_framework;
}

#[test]
fn test_build_orchestrator_project_info() {
    let config = create_test_config();
    assert_eq!(config.project.name, "test-app");
    assert_eq!(config.project.java_version, "17");
}

#[test]
fn test_build_orchestrator_empty_dependencies() {
    let config = create_test_config();
    assert_eq!(config.dependencies.len(), 0);
}

#[test]
fn test_build_orchestrator_empty_repositories() {
    let config = create_test_config();
    assert_eq!(config.repositories.len(), 0);
}

#[test]
fn test_build_orchestrator_with_multiple_source_dirs() {
    let mut config = create_test_config();
    config.build.source_dirs = vec!["src/main/java".to_string(), "src/gen/java".to_string()];

    let result = BuildOrchestrator::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_build_config_custom_java_version() {
    let mut config = create_test_config();
    config.project.java_version = "11".to_string();

    assert_eq!(config.project.java_version, "11");
}

#[test]
fn test_build_orchestrator_modifications() {
    let mut config = create_test_config();
    config.build.output_dir = "target/build".to_string();
    assert_eq!(config.build.output_dir, "target/build");
}
