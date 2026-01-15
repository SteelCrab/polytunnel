//! Classpath management and dependency resolution

#[cfg(test)]
use polytunnel_core::AppError;
use polytunnel_core::{ProjectConfig, Result};
#[cfg(test)]
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
    ///
    /// # Returns
    ///
    /// ClasspathResult with separate classpaths for compile, test, and runtime
    ///
    /// # Errors
    ///
    /// * `AppError::InvalidDependency` - If dependency format is invalid
    /// * `AppError::Io` - If JAR download fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = builder.build_classpath(".polytunnel/cache").await?;
    /// ```
    pub async fn build_classpath(&mut self, _cache_dir: &str) -> Result<ClasspathResult> {
        // For now, return empty classpaths
        // Full implementation will use Resolver and MavenClient
        let result = ClasspathResult {
            compile_classpath: vec![],
            test_classpath: vec![],
            runtime_classpath: vec![],
        };

        self.cached_result = Some(result.clone());
        Ok(result)
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

    /// Convert classpaths to platform-specific string
    #[cfg(test)]
    fn format_classpath(paths: &[PathBuf]) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
        paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Parse Maven coordinate from dependency key
    #[cfg(test)]
    fn parse_coordinate(key: &str) -> Result<Coordinate> {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() < 2 {
            return Err(AppError::InvalidDependency {
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
