//! Tests for incremental build cache

use polytunnel_build::{BuildCache, BuildCacheEntry};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_build_cache_entry_serialization() {
    let entry = BuildCacheEntry {
        source_file: PathBuf::from("src/Main.java"),
        last_modified: 1705334400,
        output_file: PathBuf::from("target/classes/Main.class"),
    };

    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: BuildCacheEntry = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.source_file, entry.source_file);
    assert_eq!(deserialized.last_modified, entry.last_modified);
}

#[test]
fn test_cache_entry_map_serialization() {
    let mut entries = HashMap::new();
    entries.insert(
        "src/Main.java".to_string(),
        BuildCacheEntry {
            source_file: PathBuf::from("src/Main.java"),
            last_modified: 1705334400,
            output_file: PathBuf::from("target/classes/Main.class"),
        },
    );

    let json = serde_json::to_string_pretty(&entries).unwrap();
    let deserialized: HashMap<String, BuildCacheEntry> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.len(), 1);
    assert!(deserialized.contains_key("src/Main.java"));
}

fn make_config(cache_dir: &std::path::Path) -> polytunnel_core::ProjectConfig {
    let mut config = polytunnel_core::ProjectConfig::new("test");
    config.build.cache_dir = cache_dir.to_string_lossy().to_string();
    config
}

#[test]
fn test_build_cache_persistence_round_trip() {
    let tmpdir = tempfile::tempdir().unwrap();
    let config = make_config(tmpdir.path());

    // Create a source file to track
    let source_file = tmpdir.path().join("Test.java");
    std::fs::write(&source_file, "class Test {}").unwrap();

    // Populate cache and save to disk
    let mut cache = BuildCache::new(&config).unwrap();
    cache
        .update_for_sources(std::slice::from_ref(&source_file))
        .unwrap();

    // Load a fresh cache instance from the same directory
    let cache2 = BuildCache::new(&config).unwrap();
    let key = source_file.to_string_lossy().to_string();
    assert!(
        cache2.entries.contains_key(&key),
        "persisted entry should be present after reload"
    );
}

#[test]
fn test_build_cache_clear_persists() {
    let tmpdir = tempfile::tempdir().unwrap();
    let config = make_config(tmpdir.path());

    let source_file = tmpdir.path().join("Foo.java");
    std::fs::write(&source_file, "class Foo {}").unwrap();

    let mut cache = BuildCache::new(&config).unwrap();
    cache.update_for_sources(&[source_file]).unwrap();
    assert!(
        !cache.entries.is_empty(),
        "cache should have entries before clear"
    );

    cache.clear().unwrap();

    // New instance should load the now-empty cache
    let cache2 = BuildCache::new(&config).unwrap();
    assert!(
        cache2.entries.is_empty(),
        "cache should be empty after clear() + reload"
    );
}
