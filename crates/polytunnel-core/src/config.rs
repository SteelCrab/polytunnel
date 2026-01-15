//! Configuration types for polytunnel

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::Result;

/// Project configuration (polytunnel.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
    #[serde(default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    #[serde(default = "default_java_version")]
    pub java_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Simple format: "1.4.14"
    Simple(String),
    /// Detailed format with scope and options
    Detailed {
        version: String,
        #[serde(default)]
        scope: DependencyScope,
        #[serde(default)]
        optional: bool,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyScope {
    #[default]
    Compile,
    Runtime,
    Test,
    Provided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

/// Build configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_source_dirs")]
    pub source_dirs: Vec<String>,

    #[serde(default = "default_test_source_dirs")]
    pub test_source_dirs: Vec<String>,

    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    #[serde(default = "default_test_output_dir")]
    pub test_output_dir: String,

    #[serde(default)]
    pub compiler_args: Vec<String>,

    #[serde(default)]
    pub test_compiler_args: Vec<String>,

    #[serde(default = "default_test_framework")]
    pub test_framework: String,

    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
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
            Dependency::Detailed { scope, .. } => scope.clone(),
        }
    }
}
