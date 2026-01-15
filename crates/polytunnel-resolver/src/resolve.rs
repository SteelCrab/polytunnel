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

        for dep in deps {
            self.resolve_recursive(dep, 0, &mut all_deps).await?;
        }

        Ok(ResolvedTree {
            root_dependencies: deps.to_vec(),
            all_dependencies: all_deps,
        })
    }

    fn resolve_parent_data<'a>(
        &'a self,
        parent_coord: &'a Coordinate,
        depth: usize,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<(
                        std::collections::HashMap<String, String>,
                        Vec<polytunnel_maven::PomDependency>,
                    )>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            if depth > 10 {
                // Avoid infinite parent recursion
                return Ok((std::collections::HashMap::new(), Vec::new()));
            }

            let pom = self.client.fetch_pom(parent_coord).await?;
            let mut props = pom.properties.clone();
            let mut dm = pom.dependency_management.clone();

            if let Some(grandparent) = &pom.parent {
                let (gp_props, gp_dm) = self.resolve_parent_data(grandparent, depth + 1).await?;

                // Parent keys override Grandparent keys
                for (k, v) in gp_props {
                    props.entry(k).or_insert(v);
                }

                // Parent DM comes before Grandparent DM (we append GP to end)
                dm.extend(gp_dm);
            }

            Ok((props, dm))
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

            // Fetch POM
            let mut pom = self.client.fetch_pom(coord).await?;

            // Resolve parent properties and DM
            if let Some(parent) = &pom.parent {
                match self.resolve_parent_data(parent, 0).await {
                    Ok((parent_props, parent_dm)) => {
                        pom.merge_dependency_management(parent_dm);
                        pom.merge_properties(&parent_props);
                    }
                    Err(e) => {
                        println!("Warning: Failed to resolve parent {}: {}", parent, e);
                    }
                }
            }

            // Fill missing versions using Dependency Management
            pom.fill_missing_versions();

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

            // Only add to collected dependencies if it's not a POM (no JAR)
            if pom.packaging != "pom" {
                // Deduplication: if already have this G:A in collected with some version,
                // we should decide. Maven uses "nearest" (first one wins in BFS).
                // Since this is DFS, first one wins might be okay, or we can keep map.
                let ga = format!("{}:{}", coord.group_id, coord.artifact_id);
                if !collected
                    .iter()
                    .any(|c| format!("{}:{}", c.group_id, c.artifact_id) == ga)
                {
                    collected.push(coord.clone());
                }
            }

            // Recursively resolve transitive dependencies
            for trans_dep in transitive {
                if let Err(e) = self
                    .resolve_recursive(&trans_dep, depth + 1, collected)
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
