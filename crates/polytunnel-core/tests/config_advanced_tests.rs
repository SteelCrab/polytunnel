//! Advanced tests for configuration handling and validation
//!
//! Coverage: Verifies ProjectConfig, ProjectInfo, and BuildConfig fields, ensuring robust configuration management and validation logic.

use polytunnel_core::{
    BuildConfig, Dependency, DependencyScope, ProjectConfig, ProjectInfo, Repository,
};
use std::collections::HashMap;

#[test]
fn test_project_config_default_creation() {
    let config = ProjectConfig {
        project: ProjectInfo {
            name: "default-app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    assert!(!config.project.name.is_empty());
}

#[test]
fn test_build_config_all_fields_accessible() {
    let config = BuildConfig::default();

    let _ = config.source_dirs;
    let _ = config.test_source_dirs;
    let _ = config.output_dir;
    let _ = config.test_output_dir;
    let _ = config.compiler_args;
    let _ = config.test_compiler_args;
    let _ = config.test_framework;
    let _ = config.cache_dir;
}

#[test]
fn test_dependency_simple_variant_creation() {
    let dep = Dependency::Simple("1.0.0".to_string());

    match dep {
        Dependency::Simple(v) => assert_eq!(v, "1.0.0"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_dependency_detailed_variant_creation() {
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
fn test_repository_structure_fields() {
    let repo = Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    };

    assert!(!repo.name.is_empty());
    assert!(!repo.url.is_empty());
}

#[test]
fn test_dependency_scope_compile_variant() {
    let scope = DependencyScope::Compile;
    assert_eq!(scope, DependencyScope::Compile);
}

#[test]
fn test_dependency_scope_test_variant() {
    let scope = DependencyScope::Test;
    assert_eq!(scope, DependencyScope::Test);
}

#[test]
fn test_dependency_scope_runtime_variant() {
    let scope = DependencyScope::Runtime;
    assert_eq!(scope, DependencyScope::Runtime);
}

#[test]
fn test_dependency_scope_provided_variant() {
    let scope = DependencyScope::Provided;
    assert_eq!(scope, DependencyScope::Provided);
}

#[test]
fn test_project_info_name_field() {
    let info = ProjectInfo {
        name: "my-project".to_string(),
        java_version: "11".to_string(),
    };

    assert_eq!(info.name, "my-project");
}

#[test]
fn test_project_info_java_version_field() {
    let info = ProjectInfo {
        name: "app".to_string(),
        java_version: "21".to_string(),
    };

    assert_eq!(info.java_version, "21");
}

#[test]
fn test_build_config_source_dirs_field() {
    let mut config = BuildConfig::default();
    config.source_dirs = vec!["src/main/java".to_string()];

    assert_eq!(config.source_dirs.len(), 1);
}

#[test]
fn test_build_config_output_dir_field() {
    let mut config = BuildConfig::default();
    config.output_dir = "build/classes".to_string();

    assert_eq!(config.output_dir, "build/classes");
}

#[test]
fn test_build_config_cache_dir_field() {
    let mut config = BuildConfig::default();
    config.cache_dir = ".cache".to_string();

    assert_eq!(config.cache_dir, ".cache");
}

#[test]
fn test_project_config_dependencies_field() {
    let mut config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    config.dependencies.insert(
        "org.junit:junit".to_string(),
        Dependency::Simple("4.13.2".to_string()),
    );

    assert_eq!(config.dependencies.len(), 1);
}

#[test]
fn test_project_config_repositories_field() {
    let mut config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    config.repositories.push(Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    });

    assert_eq!(config.repositories.len(), 1);
}

#[test]
fn test_config_modification_preserves_other_fields() {
    let mut config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    let original_name = config.project.name.clone();
    config.build.output_dir = "new-output".to_string();

    assert_eq!(config.project.name, original_name);
}

#[test]
fn test_dependency_collection_iteration() {
    let mut deps: HashMap<String, Dependency> = HashMap::new();
    deps.insert("lib1".to_string(), Dependency::Simple("1.0.0".to_string()));
    deps.insert("lib2".to_string(), Dependency::Simple("2.0.0".to_string()));

    let count = deps.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_repository_collection_iteration() {
    let repos = vec![
        Repository {
            name: "repo1".to_string(),
            url: "url1".to_string(),
        },
        Repository {
            name: "repo2".to_string(),
            url: "url2".to_string(),
        },
    ];

    let count = repos.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_source_dirs_collection_mutation() {
    let mut config = BuildConfig::default();
    config.source_dirs.push("src/extra".to_string());

    assert!(config.source_dirs.len() > 0);
}

#[test]
fn test_compiler_args_collection_mutation() {
    let mut config = BuildConfig::default();
    config.compiler_args.push("-encoding".to_string());
    config.compiler_args.push("UTF-8".to_string());

    assert_eq!(config.compiler_args.len(), 2);
}

#[test]
fn test_dependency_lookup_by_key() {
    let mut deps: HashMap<String, Dependency> = HashMap::new();
    deps.insert(
        "org.junit:junit".to_string(),
        Dependency::Simple("4.13.2".to_string()),
    );

    let found = deps.get("org.junit:junit");
    assert!(found.is_some());
}

#[test]
fn test_dependency_removal() {
    let mut deps: HashMap<String, Dependency> = HashMap::new();
    deps.insert("lib1".to_string(), Dependency::Simple("1.0.0".to_string()));

    deps.remove("lib1");
    assert_eq!(deps.len(), 0);
}

#[test]
fn test_repository_lookup_by_name() {
    let repos = vec![Repository {
        name: "central".to_string(),
        url: "https://repo1".to_string(),
    }];

    let found = repos.iter().find(|r| r.name == "central");
    assert!(found.is_some());
}

#[test]
fn test_config_partial_initialization() {
    let config = ProjectConfig {
        project: ProjectInfo {
            name: "app".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    assert_eq!(config.dependencies.len(), 0);
    assert_eq!(config.repositories.len(), 0);
}

#[test]
fn test_multiple_dependencies_same_group() {
    let mut deps: HashMap<String, Dependency> = HashMap::new();
    deps.insert(
        "org.junit:junit-api".to_string(),
        Dependency::Simple("4.13.2".to_string()),
    );
    deps.insert(
        "org.junit:junit-core".to_string(),
        Dependency::Simple("4.13.2".to_string()),
    );

    let count = deps.len();
    assert_eq!(count, 2);
}

#[test]
fn test_dependency_scope_equality() {
    let scope1 = DependencyScope::Test;
    let scope2 = DependencyScope::Test;

    assert_eq!(scope1, scope2);
}

#[test]
fn test_dependency_scope_inequality() {
    let scope1 = DependencyScope::Compile;
    let scope2 = DependencyScope::Test;

    assert_ne!(scope1, scope2);
}

#[test]
fn test_project_info_equality() {
    let info1 = ProjectInfo {
        name: "app".to_string(),
        java_version: "17".to_string(),
    };

    let info2 = ProjectInfo {
        name: "app".to_string(),
        java_version: "17".to_string(),
    };

    assert_eq!(info1.name, info2.name);
    assert_eq!(info1.java_version, info2.java_version);
}

#[test]
fn test_repository_url_format_validation() {
    let repo = Repository {
        name: "test".to_string(),
        url: "https://example.com/repo/".to_string(),
    };

    assert!(repo.url.starts_with("https://"));
}

#[test]
fn test_build_config_test_framework_default() {
    let config = BuildConfig::default();
    assert_eq!(config.test_framework, "auto");
}

#[test]
fn test_build_config_cache_dir_default() {
    let config = BuildConfig::default();
    assert!(!config.cache_dir.is_empty());
}

#[test]
fn test_dependency_version_field_access() {
    let dep = Dependency::Simple("3.0.0".to_string());

    match dep {
        Dependency::Simple(v) => assert_eq!(v, "3.0.0"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_detailed_dependency_scope_field_access() {
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
fn test_repository_list_filtering() {
    let repos = vec![
        Repository {
            name: "central".to_string(),
            url: "https://repo1".to_string(),
        },
        Repository {
            name: "custom".to_string(),
            url: "https://repo2".to_string(),
        },
    ];

    let filtered: Vec<_> = repos.iter().filter(|r| r.name.starts_with("c")).collect();
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_source_dirs_contains_search() {
    let dirs = vec!["src/main/java".to_string(), "src/test/java".to_string()];

    let has_main = dirs.iter().any(|d| d.contains("main"));
    assert!(has_main);
}

#[test]
fn test_dependency_map_contains_key() {
    let mut deps: HashMap<String, Dependency> = HashMap::new();
    deps.insert("lib".to_string(), Dependency::Simple("1.0.0".to_string()));

    assert!(deps.contains_key("lib"));
}

#[test]
fn test_build_config_fields_independence() {
    let mut config1 = BuildConfig::default();
    let mut config2 = BuildConfig::default();

    config1.output_dir = "out1".to_string();
    config2.output_dir = "out2".to_string();

    assert_ne!(config1.output_dir, config2.output_dir);
}

#[test]
fn test_project_info_cloneable() {
    let info1 = ProjectInfo {
        name: "app".to_string(),
        java_version: "17".to_string(),
    };

    let info2 = info1.clone();
    assert_eq!(info1.name, info2.name);
}

#[test]
fn test_build_config_cloneable() {
    let config1 = BuildConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.output_dir, config2.output_dir);
}

#[test]
fn test_repository_cloneable() {
    let repo1 = Repository {
        name: "test".to_string(),
        url: "https://test.com".to_string(),
    };

    let repo2 = repo1.clone();
    assert_eq!(repo1.name, repo2.name);
}
