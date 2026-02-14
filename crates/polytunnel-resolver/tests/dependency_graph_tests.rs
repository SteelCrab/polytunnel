//! Unit tests for DependencyGraph behavior.

use polytunnel_maven::Coordinate;
use polytunnel_resolver::{DependencyGraph, DependencyNode};

#[test]
fn test_add_node_and_get() {
    let mut graph = DependencyGraph::new();
    let coord = Coordinate::parse("org.slf4j:slf4j-api:2.0.9").unwrap();

    graph.add_node(coord.clone(), vec![], 0);

    assert!(graph.contains("org.slf4j:slf4j-api:2.0.9"));
    let node = graph.get("org.slf4j:slf4j-api:2.0.9").unwrap();
    assert_eq!(node.coordinate, coord);
    assert_eq!(node.dependencies.len(), 0);
}

#[test]
fn test_graph_node_overwrites_previous_entry() {
    let mut graph = DependencyGraph::new();
    let first = Coordinate::parse("org.app:app:1.0.0").unwrap();
    let second = Coordinate::parse("org.app:app:1.0.0").unwrap();
    let replacement_dep = Coordinate::parse("org.shared:dep:1.0.0").unwrap();

    graph.add_node(first, vec![], 0);
    graph.add_node(second.clone(), vec![replacement_dep.clone()], 3);

    let node = graph.get("org.app:app:1.0.0").unwrap();
    assert_eq!(node.depth, 3);
    assert_eq!(node.dependencies, vec![replacement_dep]);
}

#[test]
fn test_nodes_iterator_exposes_all_nodes() {
    let mut graph = DependencyGraph::new();
    let coords = vec![
        Coordinate::parse("org.a:a:1.0.0").unwrap(),
        Coordinate::parse("org.b:b:2.0.0").unwrap(),
        Coordinate::parse("org.c:c:3.0.0").unwrap(),
    ];

    for coord in &coords {
        graph.add_node(coord.clone(), vec![], 0);
    }

    let mut listed = graph
        .nodes()
        .map(|node: &DependencyNode| node.coordinate.to_string())
        .collect::<Vec<_>>();
    listed.sort();

    let mut expected = coords.iter().map(|c| c.to_string()).collect::<Vec<_>>();
    expected.sort();

    assert_eq!(listed, expected);
}
