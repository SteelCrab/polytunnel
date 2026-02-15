use polytunnel_build::BuildCache;
use polytunnel_core::{BuildConfig, ProjectConfig, ProjectInfo};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

fn create_config(cache_dir: &Path) -> ProjectConfig {
    let build = BuildConfig {
        cache_dir: cache_dir.to_string_lossy().to_string(),
        ..BuildConfig::default()
    };

    ProjectConfig {
        project: ProjectInfo {
            name: "cache-test".to_string(),
            java_version: "17".to_string(),
        },
        build,
        dependencies: HashMap::new(),
        repositories: vec![],
    }
}

fn write_source(root: &Path, relative: &str, content: &str) -> PathBuf {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_new_starts_empty_when_cache_file_absent() {
    let temp = tempdir().unwrap();
    let config = create_config(&temp.path().join(".polytunnel/cache"));

    let cache = BuildCache::new(&config).unwrap();
    assert!(cache.entries.is_empty());
}

#[test]
fn test_new_reads_existing_cache_file() {
    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join(".polytunnel/cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_file = cache_dir.join("build-cache.json");
    fs::write(
        &cache_file,
        r#"{
  "/tmp/src/Main.java": {
    "source_file": "/tmp/src/Main.java",
    "last_modified": 10,
    "output_file": "/tmp/target/Main.class"
  }
}"#,
    )
    .unwrap();

    let config = create_config(&cache_dir);
    let cache = BuildCache::new(&config).unwrap();

    assert_eq!(cache.entries.len(), 1);
    assert!(cache.entries.contains_key("/tmp/src/Main.java"));
}

#[test]
fn test_new_tolerates_invalid_cache_json() {
    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join(".polytunnel/cache");
    fs::create_dir_all(&cache_dir).unwrap();
    fs::write(cache_dir.join("build-cache.json"), "{invalid").unwrap();

    let config = create_config(&cache_dir);
    let cache = BuildCache::new(&config).unwrap();

    assert!(cache.entries.is_empty());
}

#[test]
fn test_update_get_files_to_compile_and_clear() {
    let temp = tempdir().unwrap();
    let cache_dir = temp.path().join(".polytunnel/cache");
    let config = create_config(&cache_dir);
    let source_root = temp.path().join("src");

    let source_a = write_source(&source_root, "Main.java", "class Main {}");
    let source_b = write_source(&source_root, "Util.java", "class Util {}");

    let mut cache = BuildCache::new(&config).unwrap();
    cache
        .update_for_sources(std::slice::from_ref(&source_a))
        .unwrap();

    assert_eq!(cache.entries.len(), 1);
    assert!(cache_dir.join("build-cache.json").exists());

    let first = cache
        .get_files_to_compile(&[source_a.clone(), source_b.clone()])
        .unwrap();
    assert!(first.contains(&source_b));
    assert!(!first.contains(&source_a));

    thread::sleep(Duration::from_secs(1));
    fs::write(&source_a, "class Main { int v = 1; }").unwrap();
    let second = cache
        .get_files_to_compile(std::slice::from_ref(&source_a))
        .unwrap();
    assert_eq!(second, vec![source_a.clone()]);

    fs::remove_file(&source_a).unwrap();
    let third = cache
        .get_files_to_compile(std::slice::from_ref(&source_a))
        .unwrap();
    assert!(third.is_empty());

    cache.clear().unwrap();
    assert!(cache.entries.is_empty());
}

#[test]
fn test_update_ignores_missing_sources() {
    let temp = tempdir().unwrap();
    let config = create_config(&temp.path().join(".polytunnel/cache"));
    let missing = temp.path().join("src/Missing.java");

    let mut cache = BuildCache::new(&config).unwrap();
    cache.update_for_sources(&[missing]).unwrap();

    assert!(cache.entries.is_empty());
}
