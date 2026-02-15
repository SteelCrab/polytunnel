use polytunnel_core::{Dependency, DependencyScope, ProjectConfig, ProjectInfo};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn make_temp_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch should not happen")
        .as_nanos();
    path.push(format!(
        "polytunnel-core-test-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

#[test]
fn test_project_config_load_and_save_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = make_temp_dir();
    let config_path = project_dir.join("polytunnel.toml");

    let mut config = ProjectConfig::new("roundtrip");
    config.build.source_dirs = vec!["src/main/java".to_string()];
    config.build.output_dir = "target/classes".to_string();
    config.build.cache_dir = ".polytunnel/cache".to_string();
    config.repositories.push(polytunnel_core::Repository {
        name: "central".to_string(),
        url: "https://repo1.maven.org/maven2/".to_string(),
    });
    config.dependencies.insert(
        "org.junit.jupiter:junit-jupiter".to_string(),
        Dependency::Detailed {
            version: "5.10.2".to_string(),
            scope: DependencyScope::Test,
            optional: false,
        },
    );

    config.save(&config_path)?;

    let loaded = ProjectConfig::load(&config_path)?;
    assert_eq!(loaded.project.name, config.project.name);
    assert_eq!(loaded.build.source_dirs, config.build.source_dirs);
    assert_eq!(
        loaded.dependencies.len(),
        1,
        "dependency table should roundtrip with size 1"
    );

    fs::write(config_path.with_extension("bad"), ":::bad toml:::")?;
    let bad_load = ProjectConfig::load(&config_path.with_extension("bad"));
    assert!(bad_load.is_err(), "invalid TOML should return an error");

    fs::remove_dir_all(project_dir)?;
    Ok(())
}

#[test]
fn test_dependency_scope_helpers() {
    let simple_dep = Dependency::Simple("1.0.0".to_string());
    assert_eq!(simple_dep.version(), "1.0.0");
    assert_eq!(simple_dep.scope(), DependencyScope::Compile);

    let dep = Dependency::Detailed {
        version: "2.0".to_string(),
        scope: DependencyScope::Runtime,
        optional: true,
    };
    assert_eq!(dep.version(), "2.0");
    assert_eq!(dep.scope(), DependencyScope::Runtime);

    let compile_dep = Dependency::Simple("3.14.0".to_string());
    assert_eq!(compile_dep.scope(), DependencyScope::Compile);

    let info = ProjectInfo {
        name: "app".to_string(),
        java_version: "21".to_string(),
    };
    assert_eq!(info.java_version, "21");
}
