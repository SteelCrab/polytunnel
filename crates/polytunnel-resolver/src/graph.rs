//! Dependency graph representation

use polytunnel_maven::Coordinate;
use std::collections::HashMap;

/// Node in the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    pub coordinate: Coordinate,
    pub dependencies: Vec<Coordinate>,
    pub depth: usize,
}

/// Dependency graph for resolution
#[derive(Debug, Default)]
pub struct DependencyGraph {
    nodes: HashMap<String, DependencyNode>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, coord: Coordinate, deps: Vec<Coordinate>, depth: usize) {
        let key = coord.to_string();
        self.nodes.insert(
            key,
            DependencyNode {
                coordinate: coord,
                dependencies: deps,
                depth,
            },
        );
    }

    /// Get a node by coordinate string
    pub fn get(&self, key: &str) -> Option<&DependencyNode> {
        self.nodes.get(key)
    }

    /// Get all nodes
    pub fn nodes(&self) -> impl Iterator<Item = &DependencyNode> {
        self.nodes.values()
    }

    /// Check if coordinate exists
    pub fn contains(&self, key: &str) -> bool {
        self.nodes.contains_key(key)
    }
}
