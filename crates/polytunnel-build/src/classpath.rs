//! Classpath management and dependency resolution

use crate::error::{BuildError, Result};
use polytunnel_core::ProjectConfig;
use polytunnel_maven::Coordinate;
use std::path::PathBuf;

/// Result of classpath construction
#[derive(Debug, Clone)]
pub struct ClasspathResult {
    /// Classpath for compilation (compile + provided scope)
    pub compile_classpath: Vec<PathBuf>,
    /// Classpath for testing (compile + test + provided scope)
    pub test_classpath: Vec<PathBuf>,
    /// Classpath for runtime (compile + runtime scope)
    pub runtime_classpath: Vec<PathBuf>,
}

/// Builds and manages classpaths for compilation and execution
#[derive(Debug, Clone)]
pub struct ClasspathBuilder {
    #[allow(dead_code)]
    config: ProjectConfig,
    cached_result: Option<ClasspathResult>,
}

impl ClasspathBuilder {
    /// Create a new classpath builder
    ///
    /// # Arguments
    ///
    /// * `config` - Project configuration
    ///
    /// # Returns
    ///
    /// A new ClasspathBuilder instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let builder = ClasspathBuilder::new(config);
    /// ```
    pub fn new(config: ProjectConfig) -> Self {
        Self {
            config,
            cached_result: None,
        }
    }

    /// Build and cache classpath from dependencies
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Directory to cache downloaded JARs
    /// * `verbose` - Whether to print download progress
    ///
    /// # Returns
    ///
    /// ClasspathResult with separate classpaths for compile, test, and runtime
    ///
    /// # Errors
    ///
    /// * `BuildError::InvalidDependency` - If dependency format is invalid
    /// * `BuildError::Io` - If JAR download fails
    /// * `BuildError::Maven` - If Maven resolution fails
    /// * `BuildError::Resolver` - If dependency resolution fails
    pub async fn build_classpath(
        &mut self,
        cache_dir: &str,
        verbose: bool,
    ) -> Result<ClasspathResult> {
        let cache_path = PathBuf::from(cache_dir);
        if !cache_path.exists() {
            std::fs::create_dir_all(&cache_path)?;
        }

        // 1. Convert config dependencies to Coordinates
        let root_coords = self.get_root_coordinates()?;

        // 2. Resolve dependencies
        let mut resolver = polytunnel_resolver::Resolver::new();
        // Map ResolverError to BuildError
        let resolved_tree = resolver.resolve(&root_coords).await.map_err(|e| match e {
            polytunnel_resolver::ResolverError::Io(e) => BuildError::Io(e),
            polytunnel_resolver::ResolverError::Maven(e) => BuildError::from(e), // Uses From<MavenError> for BuildError
            polytunnel_resolver::ResolverError::Config(e) => BuildError::Core(e),
            _ => BuildError::CompilationFailed {
                message: e.to_string(),
            }, // Fallback for other resolver errors
        })?;

        // 3. Download artifacts
        let client = polytunnel_maven::MavenClient::new();
        let mut jar_paths = std::collections::HashMap::new();

        for coord in &resolved_tree.all_dependencies {
            let file_name = coord.jar_filename();

            // Layout: cache_dir/group/id/artifact/id/version/artifact-version.jar
            // But for simplicity in this phase, let's use the Maven repo layout structure relative to cache_dir
            let artifact_path = cache_path.join(coord.repo_path()).join(&file_name);

            if !artifact_path.exists() {
                // Ensure parent directory exists
                if let Some(parent) = artifact_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Download
                client
                    .download_jar(coord, &artifact_path, verbose)
                    .await
                    .map_err(BuildError::from)?;
            }

            jar_paths.insert(coord.to_string(), artifact_path);
        }

        // 4. Construct Classpath vectors
        let mut compile_cp = Vec::new();
        let mut test_cp = Vec::new();
        let mut runtime_cp = Vec::new();

        // Add resolved dependencies to appropriate classpaths based on scope
        // Note: The resolver should ideally give us the scope for each resolved dependency.
        // For now, we'll iterate through the resolved list.
        // A limitation of the current Resolver::resolve output is it gives a flat list of coordinates without scope info for transitive ones.
        // However, Maven transitive rules are complex.
        // For MVP/Phase 3: We will assume transitive dependencies are Scope::Compile unless specified otherwise.

        // Improve: We need to know the scope of each dependency in the resolved tree.
        // Current Resolver implementation returns simple Coordinate list.
        // We will assume 'Compile' scope for all transitive dependencies for this iteration,
        // but respect the root dependency scope for root items.

        for coord in &resolved_tree.all_dependencies {
            if let Some(path) = jar_paths.get(&coord.to_string()) {
                // Determine scope - naïve approach: check if it's a root dep and get its scope
                // If generic transitive, assume Compile/Runtime

                let scope = self
                    .get_dependency_scope(coord)
                    .unwrap_or(polytunnel_maven::DependencyScope::Compile);

                match scope {
                    polytunnel_maven::DependencyScope::Compile => {
                        compile_cp.push(path.clone());
                        test_cp.push(path.clone());
                        runtime_cp.push(path.clone());
                    }
                    polytunnel_maven::DependencyScope::Provided => {
                        compile_cp.push(path.clone());
                        test_cp.push(path.clone());
                    }
                    polytunnel_maven::DependencyScope::Runtime => {
                        runtime_cp.push(path.clone());
                        test_cp.push(path.clone());
                    }
                    polytunnel_maven::DependencyScope::Test => {
                        test_cp.push(path.clone());
                    }
                    _ => {} // Ignore System and Import scopes for now
                }
            }
        }

        let result = ClasspathResult {
            compile_classpath: compile_cp,
            test_classpath: test_cp,
            runtime_classpath: runtime_cp,
        };

        self.cached_result = Some(result.clone());
        Ok(result)
    }

    fn get_root_coordinates(&self) -> Result<Vec<Coordinate>> {
        let mut coords = Vec::new();
        for (key, dep) in &self.config.dependencies {
            let coord = Self::parse_coordinate(key)?;
            // We need to carry the version from the TOML value
            let version = dep.version();

            // Create a new coordinate with the specific version from config
            let full_coord = Coordinate::new(&coord.group_id, &coord.artifact_id, version);
            coords.push(full_coord);
        }
        Ok(coords)
    }

    fn get_dependency_scope(
        &self,
        coord: &Coordinate,
    ) -> Option<polytunnel_maven::DependencyScope> {
        // Find if this coordinate matches any root dependency key
        for (key, dep) in &self.config.dependencies {
            if let Ok(root_coord) = Self::parse_coordinate(key)
                && root_coord.group_id == coord.group_id
                    && root_coord.artifact_id == coord.artifact_id
                {
                    // Map ProjectConfig scope (polytunnel_core::DependencyScope) to Maven scope
                    let core_scope = dep.scope();
                    return Some(match core_scope {
                        polytunnel_core::DependencyScope::Compile => {
                            polytunnel_maven::DependencyScope::Compile
                        }
                        polytunnel_core::DependencyScope::Test => {
                            polytunnel_maven::DependencyScope::Test
                        }
                        polytunnel_core::DependencyScope::Runtime => {
                            polytunnel_maven::DependencyScope::Runtime
                        }
                        polytunnel_core::DependencyScope::Provided => {
                            polytunnel_maven::DependencyScope::Provided
                        }
                    });
                }
        }
        None
    }

    /// Get the cached classpath result
    ///
    /// # Returns
    ///
    /// The cached ClasspathResult from the last build_classpath call
    ///
    /// # Panics
    ///
    /// Panics if build_classpath has not been called yet
    ///
    /// # Example
    ///
    /// ```ignore
    /// let classpaths = builder.get_cached_classpath();
    /// ```
    pub fn get_cached_classpath(&self) -> ClasspathResult {
        self.cached_result
            .clone()
            .unwrap_or_else(|| ClasspathResult {
                compile_classpath: vec![],
                test_classpath: vec![],
                runtime_classpath: vec![],
            })
    }

    /// Parse Maven coordinate from dependency key
    fn parse_coordinate(key: &str) -> Result<Coordinate> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() < 2 {
            return Err(BuildError::InvalidDependency {
                input: key.to_string(),
            });
        }

        // For now, parse simple format
        // Full implementation will handle more complex cases
        Ok(Coordinate::new(
            parts[0],
            parts[1],
            if parts.len() > 2 { parts[2] } else { "LATEST" },
        ))
    }

    /// Format classpath for command line (helper for tests)
    #[allow(dead_code)]
    fn format_classpath(paths: &[PathBuf]) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
        paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        let coord = ClasspathBuilder::parse_coordinate("org.slf4j:slf4j-api:2.0.9");
        assert!(coord.is_ok());

        let bad = ClasspathBuilder::parse_coordinate("invalid");
        assert!(bad.is_err());
    }

    #[test]
    fn test_format_classpath() {
        let paths = vec![
            PathBuf::from("/path/to/lib1.jar"),
            PathBuf::from("/path/to/lib2.jar"),
        ];
        let result = ClasspathBuilder::format_classpath(&paths);
        assert!(result.contains("lib1.jar"));
        assert!(result.contains("lib2.jar"));
    }
}
