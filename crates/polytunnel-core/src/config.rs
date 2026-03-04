//! Configuration types for polytunnel

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::Result;

/// Project configuration (polytunnel.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project metadata (name, Java version)
    pub project: ProjectInfo,
    /// Build configuration (source dirs, output dirs, compiler args)
    #[serde(default)]
    pub build: BuildConfig,
    /// Dependency map: `"groupId:artifactId"` → version or detailed spec
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
    /// Additional Maven repositories (besides Maven Central)
    #[serde(default)]
    pub repositories: Vec<Repository>,
}

/// Basic project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Project name used for artifact identification
    pub name: String,
    /// Java language/bytecode version (e.g. `"17"`)
    #[serde(default = "default_java_version")]
    pub java_version: String,
}

/// Dependency specification: either a plain version string or a detailed struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Simple format: `"groupId:artifactId" = "1.4.14"`
    Simple(String),
    /// Detailed format with explicit scope and optional flag
    Detailed {
        /// Artifact version string
        version: String,
        /// Dependency scope (default: `Compile`)
        #[serde(default)]
        scope: DependencyScope,
        /// Whether the dependency is optional
        #[serde(default)]
        optional: bool,
    },
}

/// Dependency scope determining when a dependency is on the classpath
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DependencyScope {
    /// Available at compile time, test time, and runtime (default)
    #[default]
    Compile,
    /// Available at runtime and test time only
    Runtime,
    /// Available at test compilation and execution only
    Test,
    /// Available at compile time and test time, but not packaged into the artifact
    Provided,
}

/// External Maven repository definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Human-readable repository name (e.g. `"central"`)
    pub name: String,
    /// Repository base URL (e.g. `"https://repo1.maven.org/maven2/"`)
    pub url: String,
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Directories containing main Java sources (default: `["src/main/java"]`)
    #[serde(default = "default_source_dirs")]
    pub source_dirs: Vec<String>,

    /// Directories containing test Java sources (default: `["src/test/java"]`)
    #[serde(default = "default_test_source_dirs")]
    pub test_source_dirs: Vec<String>,

    /// Output directory for compiled main classes (default: `"target/classes"`)
    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    /// Output directory for compiled test classes (default: `"target/test-classes"`)
    #[serde(default = "default_test_output_dir")]
    pub test_output_dir: String,

    /// Additional arguments passed to `javac` for main sources
    #[serde(default)]
    pub compiler_args: Vec<String>,

    /// Additional arguments passed to `javac` for test sources
    #[serde(default)]
    pub test_compiler_args: Vec<String>,

    /// Test framework hint (`"auto"`, `"junit5"`, `"junit4"`, `"testng"`)
    #[serde(default = "default_test_framework")]
    pub test_framework: String,

    /// Directory for caching downloaded JARs and build metadata (default: `".polytunnel/cache"`)
    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            source_dirs: default_source_dirs(),
            test_source_dirs: default_test_source_dirs(),
            output_dir: default_output_dir(),
            test_output_dir: default_test_output_dir(),
            compiler_args: Vec::new(),
            test_compiler_args: Vec::new(),
            test_framework: default_test_framework(),
            cache_dir: default_cache_dir(),
        }
    }
}

fn default_source_dirs() -> Vec<String> {
    vec!["src/main/java".to_string()]
}

fn default_test_source_dirs() -> Vec<String> {
    vec!["src/test/java".to_string()]
}

fn default_output_dir() -> String {
    "target/classes".to_string()
}

fn default_test_output_dir() -> String {
    "target/test-classes".to_string()
}

fn default_test_framework() -> String {
    "auto".to_string()
}

fn default_cache_dir() -> String {
    ".polytunnel/cache".to_string()
}

fn default_java_version() -> String {
    "17".to_string()
}

impl ProjectConfig {
    /// Load configuration from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Create a new empty project configuration
    pub fn new(name: &str) -> Self {
        Self {
            project: ProjectInfo {
                name: name.to_string(),
                java_version: default_java_version(),
            },
            build: BuildConfig::default(),
            dependencies: HashMap::new(),
            repositories: vec![Repository {
                name: "central".to_string(),
                url: "https://repo1.maven.org/maven2/".to_string(),
            }],
        }
    }
}

impl Dependency {
    /// Get the version string
    pub fn version(&self) -> &str {
        match self {
            Dependency::Simple(v) => v,
            Dependency::Detailed { version, .. } => version,
        }
    }

    /// Get the scope (defaults to Compile for simple format)
    pub fn scope(&self) -> DependencyScope {
        match self {
            Dependency::Simple(_) => DependencyScope::Compile,
            Dependency::Detailed { scope, .. } => *scope,
        }
    }
}
