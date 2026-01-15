//! Dependency resolution algorithm

use crate::graph::DependencyGraph;
use polytunnel_core::Result;
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

        for dep in deps {
            self.resolve_recursive(dep, 0, &mut all_deps).await?;
        }

        Ok(ResolvedTree {
            root_dependencies: deps.to_vec(),
            all_dependencies: all_deps,
        })
    }

    fn resolve_recursive<'a>(
        &'a mut self,
        coord: &'a Coordinate,
        depth: usize,
        collected: &'a mut Vec<Coordinate>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let key = coord.to_string();

            // Skip if already resolved
            if self.graph.contains(&key) {
                return Ok(());
            }

            // Fetch POM and extract dependencies
            let pom = self.client.fetch_pom(coord).await?;

            let transitive: Vec<Coordinate> = pom
                .dependencies
                .iter()
                .filter(|d| d.scope == polytunnel_maven::DependencyScope::Compile)
                .filter(|d| !d.optional)
                .filter_map(|d| {
                    d.version
                        .as_ref()
                        .map(|v| Coordinate::new(&d.group_id, &d.artifact_id, v))
                })
                .collect();

            // Add to graph
            self.graph
                .add_node(coord.clone(), transitive.clone(), depth);
            collected.push(coord.clone());

            // Recursively resolve transitive dependencies
            for trans_dep in transitive {
                self.resolve_recursive(&trans_dep, depth + 1, collected)
                    .await?;
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
