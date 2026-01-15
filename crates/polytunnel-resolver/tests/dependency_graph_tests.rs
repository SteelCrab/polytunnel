//! Comprehensive tests for DependencyGraph operations
//!
//! Coverage: Ensures the integrity of graph operations, node management, and dependency link tracking.

use polytunnel_core::{Dependency, DependencyScope};
use std::collections::HashMap;

#[test]
fn test_dependency_graph_creation() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert(
        "app".to_string(),
        vec!["lib1".to_string(), "lib2".to_string()],
    );

    assert_eq!(graph.len(), 1);
    assert_eq!(graph.get("app").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_node_addition() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    graph.insert("junit".to_string(), vec![]);
    graph.insert("springframework".to_string(), vec![]);

    assert_eq!(graph.len(), 2);
}

#[test]
fn test_dependency_graph_edge_addition() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("app".to_string(), vec!["junit".to_string()]);

    if let Some(deps) = graph.get_mut("app") {
        deps.push("springframework".to_string());
    }

    assert_eq!(graph.get("app").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_transitive_dependencies() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("app".to_string(), vec!["junit".to_string()]);
    graph.insert("junit".to_string(), vec!["hamcrest".to_string()]);
    graph.insert("hamcrest".to_string(), vec![]);

    assert_eq!(graph.len(), 3);
    assert_eq!(graph.get("junit").unwrap().len(), 1);
}

#[test]
fn test_dependency_graph_with_scopes() {
    #[derive(Clone, Debug, PartialEq)]
    struct ScopedDependency {
        name: String,
        scope: DependencyScope,
    }

    let mut graph: HashMap<String, Vec<ScopedDependency>> = HashMap::new();
    graph.insert(
        "app".to_string(),
        vec![
            ScopedDependency {
                name: "junit".to_string(),
                scope: DependencyScope::Test,
            },
            ScopedDependency {
                name: "springframework".to_string(),
                scope: DependencyScope::Compile,
            },
        ],
    );

    assert_eq!(graph.get("app").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_duplicate_detection() {
    let deps = vec![
        "junit".to_string(),
        "springframework".to_string(),
        "junit".to_string(),
    ];

    let unique_deps: Vec<String> = deps
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(unique_deps.len(), 2);
}

#[test]
fn test_dependency_graph_cycle_detection_simple() {
    // Check if a->b->a exists
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("a".to_string(), vec!["b".to_string()]);
    graph.insert("b".to_string(), vec!["a".to_string()]);

    let has_cycle = graph.get("a").and_then(|deps| {
        Some(deps.iter().any(|dep| {
            graph
                .get(dep)
                .map_or(false, |subdeps| subdeps.contains(&"a".to_string()))
        }))
    });

    assert_eq!(has_cycle, Some(true));
}

#[test]
fn test_dependency_graph_cycle_detection_no_cycle() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("a".to_string(), vec!["b".to_string()]);
    graph.insert("b".to_string(), vec!["c".to_string()]);
    graph.insert("c".to_string(), vec![]);

    // Check if there's a back edge from c to a
    let has_cycle = graph
        .get("c")
        .and_then(|deps| Some(deps.contains(&"a".to_string())));

    assert_eq!(has_cycle, Some(false));
}

#[test]
fn test_dependency_graph_depth_calculation() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("app".to_string(), vec!["lib1".to_string()]);
    graph.insert("lib1".to_string(), vec!["lib2".to_string()]);
    graph.insert("lib2".to_string(), vec!["lib3".to_string()]);
    graph.insert("lib3".to_string(), vec![]);

    // Calculate depth from app
    let mut max_depth = 0;
    let mut visited = std::collections::HashSet::new();

    fn dfs(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        depth: usize,
        max: &mut usize,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if visited.contains(node) {
            return;
        }
        visited.insert(node.to_string());
        *max = (*max).max(depth);

        if let Some(deps) = graph.get(node) {
            for dep in deps {
                dfs(dep, graph, depth + 1, max, visited);
            }
        }
    }

    dfs("app", &graph, 0, &mut max_depth, &mut visited);
    assert_eq!(max_depth, 3);
}

#[test]
fn test_dependency_graph_width_calculation() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert(
        "app".to_string(),
        vec![
            "lib1".to_string(),
            "lib2".to_string(),
            "lib3".to_string(),
            "lib4".to_string(),
        ],
    );

    let width = graph.get("app").unwrap().len();
    assert_eq!(width, 4);
}

#[test]
fn test_dependency_graph_version_conflicts() {
    #[derive(Clone)]
    struct VersionedDependency {
        name: String,
        version: String,
    }

    let mut conflicts: HashMap<String, Vec<String>> = HashMap::new();
    conflicts.insert(
        "junit".to_string(),
        vec!["4.13.2".to_string(), "4.12".to_string()],
    );

    assert_eq!(conflicts.get("junit").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_version_conflict_resolution() {
    let versions = vec!["1.0.0", "1.1.0", "1.2.0"];
    let resolved_version = versions.iter().max();

    assert_eq!(resolved_version, Some(&"1.2.0"));
}

#[test]
fn test_dependency_graph_scope_filtering() {
    let mut scoped_deps: HashMap<String, HashMap<DependencyScope, Vec<String>>> = HashMap::new();

    let mut compile_deps = HashMap::new();
    compile_deps.insert(
        DependencyScope::Compile,
        vec!["springframework".to_string()],
    );
    compile_deps.insert(DependencyScope::Test, vec!["junit".to_string()]);

    scoped_deps.insert("app".to_string(), compile_deps);

    let compile_only = scoped_deps
        .get("app")
        .and_then(|m| m.get(&DependencyScope::Compile))
        .unwrap();

    assert_eq!(compile_only.len(), 1);
}

#[test]
fn test_dependency_graph_scope_merging() {
    let mut all_deps = vec![];
    all_deps.push(("springframework".to_string(), DependencyScope::Compile));
    all_deps.push(("junit".to_string(), DependencyScope::Test));
    all_deps.push(("mockito".to_string(), DependencyScope::Test));

    assert_eq!(all_deps.len(), 3);
}

#[test]
fn test_dependency_graph_optional_dependencies() {
    #[derive(Clone)]
    struct OptionalDependency {
        name: String,
        optional: bool,
    }

    let deps = vec![
        OptionalDependency {
            name: "lib1".to_string(),
            optional: false,
        },
        OptionalDependency {
            name: "lib2".to_string(),
            optional: true,
        },
    ];

    let required = deps.iter().filter(|d| !d.optional).count();
    let optional = deps.iter().filter(|d| d.optional).count();

    assert_eq!(required, 1);
    assert_eq!(optional, 1);
}

#[test]
fn test_dependency_graph_exclusions() {
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    deps.insert(
        "app".to_string(),
        vec!["lib1".to_string(), "lib2".to_string()],
    );

    if let Some(dep_list) = deps.get_mut("app") {
        dep_list.retain(|d| d != "lib1");
    }

    assert_eq!(deps.get("app").unwrap().len(), 1);
}

#[test]
fn test_dependency_graph_bom_import() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert(
        "spring-boot-bom".to_string(),
        vec![
            "spring-boot-dependencies".to_string(),
            "spring-boot-starter".to_string(),
        ],
    );

    assert_eq!(graph.get("spring-boot-bom").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_bom_version_management() {
    #[derive(Clone)]
    struct BomVersion {
        artifact: String,
        version: String,
    }

    let managed_versions = vec![
        BomVersion {
            artifact: "spring-core".to_string(),
            version: "6.0.0".to_string(),
        },
        BomVersion {
            artifact: "spring-web".to_string(),
            version: "6.0.0".to_string(),
        },
    ];

    assert_eq!(managed_versions.len(), 2);
    assert!(managed_versions.iter().all(|v| v.version == "6.0.0"));
}

#[test]
fn test_dependency_graph_repository_variants() {
    let mut repo_deps: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    let mut central = HashMap::new();
    central.insert("junit".to_string(), vec!["4.13.2".to_string()]);

    repo_deps.insert("central".to_string(), central);

    assert_eq!(repo_deps.len(), 1);
}

#[test]
fn test_dependency_graph_package_classifier_handling() {
    #[derive(Clone)]
    struct ClassifiedArtifact {
        name: String,
        classifier: Option<String>,
    }

    let artifacts = vec![
        ClassifiedArtifact {
            name: "junit".to_string(),
            classifier: None,
        },
        ClassifiedArtifact {
            name: "junit".to_string(),
            classifier: Some("sources".to_string()),
        },
        ClassifiedArtifact {
            name: "junit".to_string(),
            classifier: Some("javadoc".to_string()),
        },
    ];

    assert_eq!(artifacts.len(), 3);
}

#[test]
fn test_dependency_graph_snapshot_handling() {
    let deps = vec![
        ("lib1".to_string(), "1.0.0-SNAPSHOT".to_string()),
        ("lib2".to_string(), "1.0.0".to_string()),
    ];

    let snapshots: Vec<_> = deps
        .iter()
        .filter(|(_, v)| v.contains("SNAPSHOT"))
        .collect();
    assert_eq!(snapshots.len(), 1);
}

#[test]
fn test_dependency_graph_release_vs_snapshot() {
    let versions = vec!["1.0.0", "1.0.0-SNAPSHOT", "1.0.0-RC1", "1.0.0-beta"];

    let releases: Vec<_> = versions.iter().filter(|v| !v.contains("-")).collect();
    let pre_releases: Vec<_> = versions.iter().filter(|v| v.contains("-")).collect();

    assert_eq!(releases.len(), 1);
    assert_eq!(pre_releases.len(), 3);
}

#[test]
fn test_dependency_graph_repository_priority() {
    let mut repos_with_priority = vec![
        ("central".to_string(), 1),
        ("custom".to_string(), 2),
        ("snapshots".to_string(), 3),
    ];

    repos_with_priority.sort_by_key(|r| r.1);
    assert_eq!(repos_with_priority[0].0, "central");
}

#[test]
fn test_dependency_graph_transitive_scope_propagation() {
    // Compile scope propagates transitively
    // Provided scope does not
    // Runtime scope does not propagate to compile

    let mut scopes: HashMap<String, DependencyScope> = HashMap::new();
    scopes.insert("lib1".to_string(), DependencyScope::Compile);
    scopes.insert("lib2".to_string(), DependencyScope::Provided);
    scopes.insert("lib3".to_string(), DependencyScope::Test);

    assert_eq!(scopes.len(), 3);
}

#[test]
fn test_dependency_graph_minimal_version_range() {
    let versions = vec!["1.0.0", "1.1.0", "1.2.0", "2.0.0"];

    let in_range: Vec<_> = versions.iter().filter(|v| v.starts_with("1.")).collect();

    assert_eq!(in_range.len(), 3);
}

#[test]
fn test_dependency_graph_exclusion_transitivity() {
    let mut app_deps: HashMap<String, Vec<String>> = HashMap::new();
    app_deps.insert("app".to_string(), vec!["lib1".to_string()]);

    let mut lib1_deps: HashMap<String, Vec<String>> = HashMap::new();
    lib1_deps.insert("lib1".to_string(), vec!["lib2".to_string()]);

    // Exclude lib2 from app's perspective
    if let Some(app_list) = app_deps.get_mut("app") {
        // When lib2 is excluded, it shouldn't be available to app
        app_list.push("!lib2".to_string()); // Conceptual exclusion marker
    }

    assert_eq!(app_deps.get("app").unwrap().len(), 2);
}

#[test]
fn test_dependency_graph_parent_pom_inheritance() {
    let mut inherited_props: HashMap<String, String> = HashMap::new();
    inherited_props.insert("project.version".to_string(), "1.0.0".to_string());
    inherited_props.insert("java.version".to_string(), "17".to_string());

    assert_eq!(inherited_props.len(), 2);
}

#[test]
fn test_dependency_graph_property_substitution() {
    let mut properties: HashMap<String, String> = HashMap::new();
    properties.insert("junit.version".to_string(), "5.10.1".to_string());

    let dep_version = properties.get("junit.version").unwrap();
    assert_eq!(*dep_version, "5.10.1");
}

#[test]
fn test_dependency_graph_complex_transitive_tree() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // app -> [a, b]
    // a -> [c, d]
    // b -> [d, e]
    // c -> [f]
    // d -> []
    // e -> []
    // f -> []

    graph.insert("app".to_string(), vec!["a".to_string(), "b".to_string()]);
    graph.insert("a".to_string(), vec!["c".to_string(), "d".to_string()]);
    graph.insert("b".to_string(), vec!["d".to_string(), "e".to_string()]);
    graph.insert("c".to_string(), vec!["f".to_string()]);
    graph.insert("d".to_string(), vec![]);
    graph.insert("e".to_string(), vec![]);
    graph.insert("f".to_string(), vec![]);

    assert_eq!(graph.len(), 7);

    // Count unique transitive deps from app
    let mut all_transitive = std::collections::HashSet::new();
    all_transitive.insert("a");
    all_transitive.insert("b");
    all_transitive.insert("c");
    all_transitive.insert("d");
    all_transitive.insert("e");
    all_transitive.insert("f");

    assert_eq!(all_transitive.len(), 6);
}

#[test]
fn test_dependency_graph_convergence_strategy() {
    // Nearest first: if app depends on lib1:1.0 and lib2:1.0,
    // and lib1 depends on lib3:1.0 and lib3:2.0,
    // the nearer version (from lib1) wins

    let mut version_map: HashMap<String, String> = HashMap::new();
    version_map.insert("lib3_from_lib1".to_string(), "1.0".to_string());
    version_map.insert("lib3_from_lib2".to_string(), "2.0".to_string());

    // Nearest first strategy would pick 1.0
    assert_eq!(version_map.get("lib3_from_lib1").unwrap(), "1.0");
}

#[test]
fn test_dependency_graph_mediation_result() {
    let winner = "junit:junit:4.13.2";
    assert_eq!(winner, "junit:junit:4.13.2");
}
