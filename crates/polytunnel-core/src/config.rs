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

/// Validate Maven coordinate string and return `(ga_key, version)` pair.
///
/// Accepts `"groupId:artifactId:version"` format only.
/// Returns `CoreError::InvalidCoordinate` on failure.
pub fn parse_add_coordinate(input: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 3 {
        return Err(crate::error::CoreError::InvalidCoordinate {
            message: format!("expected format 'groupId:artifactId:version', got '{input}'"),
        });
    }

    let group_id = parts[0];
    let artifact_id = parts[1];
    let version = parts[2];

    if group_id.is_empty() || artifact_id.is_empty() || version.is_empty() {
        return Err(crate::error::CoreError::InvalidCoordinate {
            message: format!("groupId, artifactId, and version must not be empty: '{input}'"),
        });
    }

    let ga_key = format!("{group_id}:{artifact_id}");
    Ok((ga_key, version.to_string()))
}

/// Add a dependency to a TOML config file, preserving formatting and comments.
///
/// If the `[dependencies]` table does not exist it is created.
/// Returns `CoreError::DuplicateDependency` when the same `ga_key` is already present.
pub fn add_dependency_to_file(
    path: &Path,
    ga_key: &str,
    version: &str,
    scope: Option<DependencyScope>,
) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let mut doc: toml_edit::DocumentMut = content.parse::<toml_edit::DocumentMut>()?;

    // Ensure [dependencies] table exists
    if !doc.contains_table("dependencies") {
        doc["dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    }

    let deps = doc["dependencies"]
        .as_table_mut()
        .expect("dependencies should be a table");

    // Check for duplicate
    if deps.contains_key(ga_key) {
        return Err(crate::error::CoreError::DuplicateDependency {
            coordinate: ga_key.to_string(),
        });
    }

    // Insert dependency: simple string for Compile scope, inline table otherwise
    match scope {
        None | Some(DependencyScope::Compile) => {
            deps[ga_key] = toml_edit::value(version);
        }
        Some(s) => {
            let mut inline = toml_edit::InlineTable::new();
            inline.insert("version", version.into());
            inline.insert("scope", scope_to_toml_str(s).into());
            deps[ga_key] = toml_edit::value(inline);
        }
    }

    std::fs::write(path, doc.to_string())?;
    Ok(())
}

/// Validate a remove coordinate and return the `ga_key`.
///
/// Accepts `"groupId:artifactId"` format only (no version).
/// Returns `CoreError::InvalidCoordinate` on failure.
pub fn parse_remove_coordinate(input: &str) -> Result<String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(crate::error::CoreError::InvalidCoordinate {
            message: format!("expected format 'groupId:artifactId', got '{input}'"),
        });
    }

    let group_id = parts[0];
    let artifact_id = parts[1];

    if group_id.is_empty() || artifact_id.is_empty() {
        return Err(crate::error::CoreError::InvalidCoordinate {
            message: format!("groupId and artifactId must not be empty: '{input}'"),
        });
    }

    Ok(input.to_string())
}

/// Remove a dependency from a TOML config file, preserving formatting and comments.
///
/// Creates a backup (`.bak`) before writing. On success the backup is removed.
/// On write failure the original file is restored from the backup.
/// Returns `CoreError::DependencyNotFound` when the `ga_key` is not present.
pub fn remove_dependency_from_file(path: &Path, ga_key: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let mut doc: toml_edit::DocumentMut = content.parse::<toml_edit::DocumentMut>()?;

    let deps = doc
        .get_mut("dependencies")
        .and_then(|d| d.as_table_mut())
        .ok_or_else(|| crate::error::CoreError::DependencyNotFound {
            coordinate: ga_key.to_string(),
        })?;

    if !deps.contains_key(ga_key) {
        return Err(crate::error::CoreError::DependencyNotFound {
            coordinate: ga_key.to_string(),
        });
    }

    deps.remove(ga_key);

    let backup_path = unique_backup_path(path);
    std::fs::copy(path, &backup_path)?;

    let write_result = std::fs::write(path, doc.to_string());
    finalize_backup_write(path, &backup_path, write_result)
}

/// Finalize a backup-protected write.
///
/// On write success the backup is removed. On write failure the original file
/// is restored from the backup and the backup is cleaned up. When the restore
/// itself fails, `CoreError::RollbackFailed` is returned carrying both the
/// original write error and the rollback error.
pub fn finalize_backup_write(
    path: &Path,
    backup_path: &Path,
    write_result: std::io::Result<()>,
) -> Result<()> {
    match write_result {
        Ok(()) => {
            let _ = std::fs::remove_file(backup_path);
            Ok(())
        }
        Err(e) => {
            if let Err(rollback_err) = std::fs::copy(backup_path, path) {
                let _ = std::fs::remove_file(backup_path);
                return Err(crate::error::CoreError::RollbackFailed {
                    write_error: e.to_string(),
                    rollback_error: rollback_err.to_string(),
                });
            }
            let _ = std::fs::remove_file(backup_path);
            Err(e.into())
        }
    }
}

/// Build a backup path that does not collide with any existing file.
///
/// Tries `<stem>.toml.bak` first, then `<stem>.toml.bak.1`, `.bak.2`, etc.
fn unique_backup_path(path: &Path) -> std::path::PathBuf {
    let base = path.with_extension("toml.bak");
    if !base.exists() {
        return base;
    }
    let base_str = base.to_string_lossy().into_owned();
    let mut n = 1u32;
    loop {
        let candidate = std::path::PathBuf::from(format!("{base_str}.{n}"));
        if !candidate.exists() {
            return candidate;
        }
        n += 1;
    }
}

fn scope_to_toml_str(scope: DependencyScope) -> &'static str {
    match scope {
        DependencyScope::Compile => "compile",
        DependencyScope::Runtime => "runtime",
        DependencyScope::Test => "test",
        DependencyScope::Provided => "provided",
    }
}
