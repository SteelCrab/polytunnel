//! Incremental build support with caching

use crate::error::Result;
use polytunnel_core::ProjectConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Cache entry for a source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCacheEntry {
    /// Source file path
    pub source_file: PathBuf,
    /// Last modification time (unix timestamp)
    pub last_modified: u64,
    /// Output .class file path
    pub output_file: PathBuf,
}

/// Build cache for incremental compilation
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildCache {
    /// Cache entries keyed by source file path
    pub entries: HashMap<String, BuildCacheEntry>,
    /// Path to cache file
    cache_file: PathBuf,
}

impl BuildCache {
    /// Create a new build cache
    ///
    /// Loads existing cache from disk if available.
    ///
    /// # Arguments
    ///
    /// * `config` - Project configuration
    ///
    /// # Returns
    ///
    /// A new BuildCache instance
    ///
    /// # Errors
    ///
    /// * `BuildError::Io` - If cache file cannot be read
    ///
    /// # Example
    ///
    /// ```ignore
    /// let cache = BuildCache::new(&config)?;
    /// ```
    pub fn new(config: &ProjectConfig) -> Result<Self> {
        let cache_dir = PathBuf::from(&config.build.cache_dir);
        let cache_file = cache_dir.join("build-cache.json");

        let entries = if cache_file.exists() {
            let content = std::fs::read_to_string(&cache_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self {
            entries,
            cache_file,
        })
    }

    /// Update cache for compiled sources
    ///
    /// # Arguments
    ///
    /// * `source_files` - Paths to compiled source files
    ///
    /// # Returns
    ///
    /// Ok(()) on success
    ///
    /// # Errors
    ///
    /// * `BuildError::Io` - If cache cannot be saved
    ///
    /// # Example
    ///
    /// ```ignore
    /// cache.update_for_sources(&[PathBuf::from("src/Main.java")])?;
    /// ```
    #[allow(clippy::collapsible_if)]
    pub fn update_for_sources(&mut self, source_files: &[PathBuf]) -> Result<()> {
        for source_file in source_files {
            if let Ok(metadata) = std::fs::metadata(source_file) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                        let key = source_file.to_string_lossy().to_string();
                        self.entries.insert(
                            key,
                            BuildCacheEntry {
                                source_file: source_file.clone(),
                                last_modified: duration.as_secs(),
                                output_file: PathBuf::new(), // Will be set by compiler
                            },
                        );
                    }
                }
            }
        }

        // Save cache to disk
        self.save()?;

        Ok(())
    }

    /// Determine which files need recompilation
    ///
    /// Compares modification times of source files with cached values.
    ///
    /// # Arguments
    ///
    /// * `source_files` - Paths to check
    ///
    /// # Returns
    ///
    /// Vec of source files that need recompilation
    ///
    /// # Example
    ///
    /// ```ignore
    /// let to_compile = cache.get_files_to_compile(&all_sources)?;
    /// ```
    pub fn get_files_to_compile(&self, source_files: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut to_compile = Vec::new();

        for source_file in source_files {
            let key = source_file.to_string_lossy().to_string();

            // Check if file needs recompilation
            let needs_compile = if let Some(cached) = self.entries.get(&key) {
                if let Ok(metadata) = std::fs::metadata(source_file) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                            duration.as_secs() > cached.last_modified
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    // File doesn't exist, skip
                    false
                }
            } else {
                // Not in cache, needs compilation
                true
            };

            if needs_compile {
                to_compile.push(source_file.clone());
            }
        }

        Ok(to_compile)
    }

    /// Save cache to disk
    fn save(&self) -> Result<()> {
        // Create cache directory if needed
        if let Some(parent) = self.cache_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self.entries).map_err(std::io::Error::other)?;
        std::fs::write(&self.cache_file, content)?;

        Ok(())
    }

    /// Clear all cache entries
    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();
        self.save()
    }
}
