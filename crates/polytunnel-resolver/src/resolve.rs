//! Dependency resolution algorithm

use crate::error::Result;
use crate::graph::DependencyGraph;
use futures::future::{BoxFuture, FutureExt, try_join_all};
use polytunnel_maven::{Coordinate, MavenClient};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Resolved dependency tree
#[derive(Debug)]
pub struct ResolvedTree {
    pub root_dependencies: Vec<Coordinate>,
    pub all_dependencies: Vec<Coordinate>,
}

/// Dependency resolver
pub struct Resolver {
    client: MavenClient,
    pub graph: DependencyGraph, // Made public or accessible if needed, or we just fill it.
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            client: MavenClient::new(),
            graph: DependencyGraph::new(),
        }
    }

    pub fn with_client(client: MavenClient) -> Self {
        Self {
            client,
            graph: DependencyGraph::new(),
        }
    }

    /// Resolve all dependencies starting from root dependencies
    pub async fn resolve(&mut self, deps: &[Coordinate]) -> Result<ResolvedTree> {
        // Build map of overrides from root dependencies (G:A -> Version)
        let mut overrides = HashMap::new();
        for dep in deps {
            let key = format!("{}:{}", dep.group_id, dep.artifact_id);
            overrides.insert(key, dep.version.clone());
        }

        let overrides = Arc::new(overrides);
        let client = self.client.clone();

        // Shared state for visited nodes to prevent cycles and redundant work
        let visited = Arc::new(Mutex::new(HashSet::new()));
        // Shared graph to populate (protected by mutex)
        let graph = Arc::new(Mutex::new(std::mem::take(&mut self.graph)));

        // Start concurrent resolution for all root dependencies
        let mut futures = Vec::new();
        for dep in deps {
            futures.push(Self::resolve_recursive(
                client.clone(),
                dep.clone(),
                0,
                overrides.clone(),
                visited.clone(),
                graph.clone(),
            ));
        }

        let results = try_join_all(futures).await?;

        // Flatten results
        let mut all_deps = Vec::new();
        for res in results {
            all_deps.extend(res);
        }

        // Restore graph
        let final_graph = Arc::try_unwrap(graph).unwrap().into_inner().unwrap();
        self.graph = final_graph;

        // Dedup all_dependencies based on GA or GAV?
        // Usually we want the exact resolved versions.
        // Simple dedup:
        let mut unique_deps = Vec::new();
        let mut seen = HashSet::new();
        for dep in all_deps {
            if seen.insert(dep.to_string()) {
                unique_deps.push(dep);
            }
        }

        Ok(ResolvedTree {
            root_dependencies: deps.to_vec(),
            all_dependencies: unique_deps,
        })
    }

    fn apply_override(coord: &Coordinate, overrides: &HashMap<String, String>) -> Coordinate {
        let ga = format!("{}:{}", coord.group_id, coord.artifact_id);
        let mut new_coord = coord.clone();
        if let Some(override_version) = overrides.get(&ga)
            && coord.version != *override_version
        {
            new_coord.version = override_version.clone();
        }
        new_coord
    }

    // Helper to fetch effective POM (recursive parent resolution - stays sequential/linear per artifact)
    fn fetch_effective_pom(
        client: MavenClient,
        coord: Coordinate,
        depth: usize,
    ) -> BoxFuture<'static, Result<polytunnel_maven::Pom>> {
        async move {
            let mut pom = client.fetch_pom(&coord).await?;

            if depth > 10 {
                return Ok(pom);
            }

            if let Some(parent_coord) = &pom.parent {
                match Self::fetch_effective_pom(client, parent_coord.clone(), depth + 1).await {
                    Ok(parent_pom) => {
                        pom.merge_dependency_management(parent_pom.dependency_management);
                        pom.merge_properties(&parent_pom.properties);
                    }
                    Err(e) => {
                        println!("Warning: Failed to resolve parent {}: {}", parent_coord, e);
                    }
                }
            }
            Ok(pom)
        }
        .boxed()
    }

    fn determine_transitive_deps(pom: &polytunnel_maven::Pom) -> Vec<Coordinate> {
        pom.dependencies
            .iter()
            .filter(|d| {
                matches!(
                    d.scope,
                    polytunnel_maven::DependencyScope::Compile
                        | polytunnel_maven::DependencyScope::Provided
                )
            })
            .filter(|d| !d.optional)
            .filter_map(|d| {
                d.version
                    .as_ref()
                    .map(|v| Coordinate::new(&d.group_id, &d.artifact_id, v))
            })
            .collect()
    }

    fn resolve_recursive(
        client: MavenClient,
        requested_coord: Coordinate,
        depth: usize,
        overrides: Arc<HashMap<String, String>>,
        visited: Arc<Mutex<HashSet<String>>>,
        graph: Arc<Mutex<DependencyGraph>>,
    ) -> BoxFuture<'static, Result<Vec<Coordinate>>> {
        async move {
            let coord = Self::apply_override(&requested_coord, &overrides);
            let key = coord.to_string();

            // Check visited
            {
                let mut v = visited.lock().unwrap();
                if v.contains(&key) {
                    return Ok(Vec::new());
                }
                v.insert(key.clone());
            }

            // Fetch POM
            let mut pom = Self::fetch_effective_pom(client.clone(), coord.clone(), 0).await?;
            pom.fill_missing_versions();

            let transitive = Self::determine_transitive_deps(&pom);

            // Update graph
            {
                let mut g = graph.lock().unwrap();
                g.add_node(coord.clone(), transitive.clone(), depth);
            }

            let mut my_deps = Vec::new();
            if pom.packaging != "pom" {
                my_deps.push(coord.clone());
            }

            // Concurrent transitive resolution
            let mut futures: Vec<BoxFuture<'static, Result<Vec<Coordinate>>>> = Vec::new();
            for trans_dep in transitive {
                let client = client.clone();
                let overrides = overrides.clone();
                let visited = visited.clone();
                let graph = graph.clone();
                let dep_clone = trans_dep.clone();

                futures.push(
                    async move {
                        match Self::resolve_recursive(
                            client,
                            trans_dep,
                            depth + 1,
                            overrides,
                            visited,
                            graph,
                        )
                        .await
                        {
                            Ok(deps) => Ok(deps),
                            Err(e) => {
                                println!(
                                    "Warning: Failed to resolve dependency {}: {}",
                                    dep_clone, e
                                );
                                Ok(Vec::new())
                            }
                        }
                    }
                    .boxed(),
                );
            }

            let results = try_join_all(futures).await?;

            for res in results {
                my_deps.extend(res);
            }

            Ok(my_deps)
        }
        .boxed()
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
