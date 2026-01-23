//! Integration tests for ClasspathBuilder
//!
//! Tests classpath building, caching, and download functionality.

use polytunnel_build::ClasspathBuilder;
use polytunnel_core::{BuildConfig, Dependency, DependencyScope, ProjectConfig, ProjectInfo};
use std::collections::HashMap;
use tempfile::TempDir;

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

fn create_config_with_dependency() -> ProjectConfig {
    let mut config = create_test_config();
    config.dependencies.insert(
        "org.slf4j:slf4j-api".to_string(),
        Dependency::Simple("2.0.9".to_string()),
    );
    config
}

#[test]
fn test_classpath_builder_creation() {
    let config = create_test_config();
    let builder = ClasspathBuilder::new(config);

    // Builder should be created successfully
    let cached = builder.get_cached_classpath();
    assert!(cached.compile_classpath.is_empty());
    assert!(cached.test_classpath.is_empty());
    assert!(cached.runtime_classpath.is_empty());
}

#[test]
fn test_classpath_builder_with_dependencies() {
    let config = create_config_with_dependency();
    let builder = ClasspathBuilder::new(config);

    // Initial cache should be empty before build_classpath is called
    let cached = builder.get_cached_classpath();
    assert!(cached.compile_classpath.is_empty());
}

#[tokio::test]
async fn test_build_classpath_creates_cache_directory() {
    let config = create_test_config();
    let mut builder = ClasspathBuilder::new(config);

    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");

    // Cache directory should not exist initially
    assert!(!cache_dir.exists());

    // Building classpath with empty dependencies should succeed
    let result = builder
        .build_classpath(cache_dir.to_str().unwrap(), false)
        .await;
    assert!(result.is_ok());

    // Cache directory should be created
    assert!(cache_dir.exists());
}

#[tokio::test]
async fn test_build_classpath_with_verbose_mode() {
    let config = create_test_config();
    let mut builder = ClasspathBuilder::new(config);

    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");

    // Test with verbose = true
    let result = builder
        .build_classpath(cache_dir.to_str().unwrap(), true)
        .await;
    assert!(result.is_ok());

    // Test with verbose = false
    let result = builder
        .build_classpath(cache_dir.to_str().unwrap(), false)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_classpath_result_structure() {
    let config = create_test_config();
    let mut builder = ClasspathBuilder::new(config);

    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");

    let result = builder
        .build_classpath(cache_dir.to_str().unwrap(), false)
        .await;
    assert!(result.is_ok());

    let classpath_result = result.unwrap();
    // With no dependencies, all classpaths should be empty
    assert!(classpath_result.compile_classpath.is_empty());
    assert!(classpath_result.test_classpath.is_empty());
    assert!(classpath_result.runtime_classpath.is_empty());
}

#[tokio::test]
async fn test_cached_classpath_after_build() {
    let config = create_test_config();
    let mut builder = ClasspathBuilder::new(config);

    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");

    // Build classpath
    let _ = builder
        .build_classpath(cache_dir.to_str().unwrap(), false)
        .await;

    // Get cached result should return the same structure
    let cached = builder.get_cached_classpath();
    assert!(cached.compile_classpath.is_empty());
}

#[test]
fn test_classpath_builder_clone() {
    let config = create_test_config();
    let builder = ClasspathBuilder::new(config);

    // ClasspathBuilder should be cloneable
    let cloned = builder.clone();

    let cached = cloned.get_cached_classpath();
    assert!(cached.compile_classpath.is_empty());
}

#[tokio::test]
async fn test_build_classpath_with_different_scopes() {
    let mut config = create_test_config();

    // Add dependencies with different scopes (using invalid coordinates for unit test)
    config.dependencies.insert(
        "compile-lib:compile-lib".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Compile,
            optional: false,
        },
    );
    config.dependencies.insert(
        "test-lib:test-lib".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: DependencyScope::Test,
            optional: false,
        },
    );

    let builder = ClasspathBuilder::new(config);

    // Builder should be created with different scope dependencies
    let cached = builder.get_cached_classpath();
    assert!(cached.compile_classpath.is_empty());
}

#[test]
fn test_classpath_result_debug() {
    use polytunnel_build::classpath::ClasspathResult;
    use std::path::PathBuf;

    let result = ClasspathResult {
        compile_classpath: vec![PathBuf::from("/path/to/lib.jar")],
        test_classpath: vec![PathBuf::from("/path/to/test-lib.jar")],
        runtime_classpath: vec![],
    };

    // Should implement Debug
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("compile_classpath"));
    assert!(debug_str.contains("test_classpath"));
}

#[test]
fn test_classpath_result_clone() {
    use polytunnel_build::classpath::ClasspathResult;
    use std::path::PathBuf;

    let result = ClasspathResult {
        compile_classpath: vec![PathBuf::from("/path/to/lib.jar")],
        test_classpath: vec![],
        runtime_classpath: vec![],
    };

    let cloned = result.clone();
    assert_eq!(cloned.compile_classpath.len(), 1);
}
