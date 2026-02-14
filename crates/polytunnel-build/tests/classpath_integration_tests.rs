use polytunnel_build::ClasspathBuilder;
use polytunnel_core::{BuildConfig, Dependency, ProjectConfig, ProjectInfo};
use polytunnel_maven::Coordinate;
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

fn config_with_invalid_dependency() -> ProjectConfig {
    ProjectConfig {
        project: ProjectInfo {
            name: "classpath-invalid".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: {
            let mut deps = HashMap::new();
            deps.insert(
                "not-a-coordinate".to_string(),
                Dependency::Simple("1.0.0".to_string()),
            );
            deps
        },
        repositories: vec![],
    }
}

fn config_with_scoped_dependencies() -> ProjectConfig {
    let mut dependencies = HashMap::new();

    dependencies.insert(
        "com.example:compile-lib:1.0.0".to_string(),
        Dependency::Simple("1.0.0".to_string()),
    );

    dependencies.insert(
        "com.example:provided-lib:1.0.0".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Provided,
            optional: false,
        },
    );

    dependencies.insert(
        "com.example:runtime-lib:1.0.0".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Runtime,
            optional: false,
        },
    );

    dependencies.insert(
        "com.example:test-lib:1.0.0".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Test,
            optional: false,
        },
    );

    dependencies.insert(
        "com.example:transitive-lib:1.0.0".to_string(),
        Dependency::Detailed {
            version: "1.0.0".to_string(),
            scope: polytunnel_core::DependencyScope::Compile,
            optional: false,
        },
    );

    ProjectConfig {
        project: ProjectInfo {
            name: "scoped-classpath".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies,
        repositories: vec![],
    }
}

fn touch_cached_dependency(cache_dir: &std::path::Path, coord: &Coordinate) {
    let jar_path = cache_dir.join(coord.repo_path()).join(coord.jar_filename());

    if let Some(parent) = jar_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(jar_path, b"dummy jar").unwrap();
}

#[tokio::test]
async fn test_build_classpath_rejects_invalid_dependency_key() {
    let mut builder = ClasspathBuilder::new(config_with_invalid_dependency());
    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join("cache");

    let result = builder
        .build_classpath(cache_dir.to_str().unwrap(), false)
        .await;
    assert!(result.is_err(), "invalid dependency key should fail early");
}

#[tokio::test]
async fn test_build_classpath_resolves_empty_dependencies_without_network() {
    let mut config = config_with_scoped_dependencies();
    config.dependencies.clear();

    let mut builder = ClasspathBuilder::new(config);

    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join("classpaths").to_string_lossy().to_string();

    let result = builder
        .build_classpath(&cache_dir, false)
        .await
        .expect("empty dependency graph should build an empty classpath");

    assert!(result.compile_classpath.is_empty());
    assert!(result.test_classpath.is_empty());
    assert!(result.runtime_classpath.is_empty());
}

#[tokio::test]
async fn test_build_classpath_from_resolved_tree_handles_all_scopes() {
    let mut builder = ClasspathBuilder::new(config_with_scoped_dependencies());
    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join("cache").to_string_lossy().to_string();

    let compile = Coordinate::parse("com.example:compile-lib:1.0.0").unwrap();
    let provided = Coordinate::parse("com.example:provided-lib:1.0.0").unwrap();
    let runtime = Coordinate::parse("com.example:runtime-lib:1.0.0").unwrap();
    let test = Coordinate::parse("com.example:test-lib:1.0.0").unwrap();
    let transitive = Coordinate::parse("com.example:transitive-lib:1.0.0").unwrap();
    let unknown = Coordinate::parse("com.example:unknown-lib:2.0.0").unwrap();

    touch_cached_dependency(temp.path().join("cache").as_path(), &compile);
    touch_cached_dependency(temp.path().join("cache").as_path(), &provided);
    touch_cached_dependency(temp.path().join("cache").as_path(), &runtime);
    touch_cached_dependency(temp.path().join("cache").as_path(), &test);
    touch_cached_dependency(temp.path().join("cache").as_path(), &transitive);
    touch_cached_dependency(temp.path().join("cache").as_path(), &unknown);

    let resolved_dependencies = vec![compile, provided, runtime, test, transitive, unknown];

    let result = builder
        .build_classpath_from_resolved_tree_for_tests(&cache_dir, resolved_dependencies, false)
        .await
        .expect("cached entries should produce deterministic classpaths");

    assert_eq!(result.compile_classpath.len(), 4);
    assert_eq!(result.test_classpath.len(), 6);
    assert_eq!(result.runtime_classpath.len(), 4);

    let cached = builder.get_cached_classpath();
    assert_eq!(cached.compile_classpath.len(), 4);
    assert_eq!(cached.test_classpath.len(), 6);
    assert_eq!(cached.runtime_classpath.len(), 4);
}
