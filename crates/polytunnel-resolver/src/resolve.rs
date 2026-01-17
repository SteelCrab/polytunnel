//! Dependency resolution algorithm

use crate::error::Result;
use crate::graph::DependencyGraph;
use polytunnel_maven::{Coordinate, MavenClient};
use std::future::Future;
use std::pin::Pin;

/// Resolved dependency tree
#[derive(Debug)]
pub struct ResolvedTree {
    pub root_dependencies: Vec<Coordinate>,
    pub all_dependencies: Vec<Coordinate>,
}

/// Dependency resolver
pub struct Resolver {
    client: MavenClient,
    graph: DependencyGraph,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            client: MavenClient::new(),
            graph: DependencyGraph::new(),
        }
    }

    /// Resolve all dependencies starting from root dependencies
    pub async fn resolve(&mut self, deps: &[Coordinate]) -> Result<ResolvedTree> {
        let mut all_deps = Vec::new();

        // Build map of overrides from root dependencies (G:A -> Version)
        // This effectively implements "Nearest Wins" strategy where root deps (depth 0)
        // override any transitive versions.
        let mut overrides = std::collections::HashMap::new();
        for dep in deps {
            let key = format!("{}:{}", dep.group_id, dep.artifact_id);
            overrides.insert(key, dep.version.clone());
        }

        for dep in deps {
            self.resolve_recursive(dep, 0, &mut all_deps, &overrides)
                .await?;
        }

        Ok(ResolvedTree {
            root_dependencies: deps.to_vec(),
            all_dependencies: all_deps,
        })
    }

    fn apply_override(
        coord: &Coordinate,
        overrides: &std::collections::HashMap<String, String>,
    ) -> Coordinate {
        let ga = format!("{}:{}", coord.group_id, coord.artifact_id);
        let mut new_coord = coord.clone();
        if let Some(override_version) = overrides.get(&ga)
            && coord.version != *override_version
        {
            new_coord.version = override_version.clone();
        }

        new_coord
    }

    #[allow(clippy::type_complexity)]
    fn fetch_effective_pom<'a>(
        &'a self,
        coord: &'a Coordinate,
        depth: usize,
    ) -> Pin<Box<dyn Future<Output = Result<polytunnel_maven::Pom>> + Send + 'a>> {
        Box::pin(async move {
            let mut pom = self.client.fetch_pom(coord).await?;

            // Limit recursion depth for parent resolution
            if depth > 10 {
                return Ok(pom);
            }

            if let Some(parent_coord) = &pom.parent {
                match self.fetch_effective_pom(parent_coord, depth + 1).await {
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
        })
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
                // version is Option<String> in PomDependency, but Coordinate needs String.
                // After fill_missing_versions(), version should be set.
                // We default to "LATEST" or handle None if still missing,
                // but typically managed deps will have versions.
                d.version
                    .as_ref()
                    .map(|v| Coordinate::new(&d.group_id, &d.artifact_id, v))
            })
            .collect()
    }

    fn resolve_recursive<'a>(
        &'a mut self,
        requested_coord: &'a Coordinate,
        depth: usize,
        collected: &'a mut Vec<Coordinate>,
        overrides: &'a std::collections::HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let coord = Self::apply_override(requested_coord, overrides);
            let key = coord.to_string();

            if self.graph.contains(&key) {
                return Ok(());
            }

            let mut pom = self.fetch_effective_pom(&coord, 0).await?;
            pom.fill_missing_versions();

            let transitive = Self::determine_transitive_deps(&pom);

            // Add to graph
            self.graph
                .add_node(coord.clone(), transitive.clone(), depth);

            // Add to collected if it's a JAR
            if pom.packaging != "pom" {
                let ga = format!("{}:{}", coord.group_id, coord.artifact_id);
                // "Nearest wins" approximation for DFS: first visit wins (or keep map)
                if !collected
                    .iter()
                    .any(|c| format!("{}:{}", c.group_id, c.artifact_id) == ga)
                {
                    collected.push(coord.clone());
                }
            }

            for trans_dep in transitive {
                if let Err(e) = self
                    .resolve_recursive(&trans_dep, depth + 1, collected, overrides)
                    .await
                {
                    println!("Warning: Failed to resolve dependency {}: {}", trans_dep, e);
                }
            }

            Ok(())
        })
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
