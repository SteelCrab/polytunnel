use polytunnel_build::BuildCache;
use polytunnel_core::{BuildConfig, ProjectConfig, ProjectInfo, Repository};
use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use tempfile::tempdir;

fn build_config_with_cache(path: &std::path::Path) -> ProjectConfig {
    ProjectConfig {
        project: ProjectInfo {
            name: "incremental-cache".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig {
            cache_dir: path.to_string_lossy().to_string(),
            ..BuildConfig::default()
        },
        dependencies: HashMap::new(),
        repositories: vec![Repository {
            name: "central".to_string(),
            url: "https://repo1.maven.org/maven2/".to_string(),
        }],
    }
}

#[test]
fn test_build_cache_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempdir()?;
    let cache_dir = temp.path().join("cache");
    let source = temp.path().join("src").join("Main.java");
    fs::create_dir_all(source.parent().unwrap())?;
    fs::write(&source, "public class Main {}")?;

    let mut cache = BuildCache::new(&build_config_with_cache(&cache_dir))?;

    assert_eq!(
        cache.get_files_to_compile(std::slice::from_ref(&source))?,
        vec![source.clone()],
        "source is not tracked yet, so file should be marked for compilation"
    );

    cache.update_for_sources(std::slice::from_ref(&source))?;
    assert!(
        cache
            .get_files_to_compile(std::slice::from_ref(&source))?
            .is_empty()
    );

    std::thread::sleep(Duration::from_millis(1100));
    fs::write(&source, "public class Main {} // touched")?;
    assert_eq!(
        cache.get_files_to_compile(std::slice::from_ref(&source))?,
        vec![source.clone()]
    );

    cache.clear()?;
    assert!(
        cache.get_files_to_compile(&[source])?.len() == 1,
        "clearing cache should require all files to recompile"
    );

    Ok(())
}
