use polytunnel_build::{BuildOptions, BuildOrchestrator, TestOptions};
use polytunnel_core::{BuildConfig, ProjectConfig, ProjectInfo};
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

fn test_config(temp_root: &std::path::Path, project: &str) -> ProjectConfig {
    let source_dir = temp_root.join("src/main/java");
    let test_source_dir = temp_root.join("src/test/java");
    let output_dir = temp_root.join("target/classes");
    let test_output_dir = temp_root.join("target/test-classes");
    let cache_dir = temp_root.join(".polytunnel/cache");

    let config = ProjectConfig {
        project: ProjectInfo {
            name: project.to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig {
            source_dirs: vec![source_dir.to_string_lossy().to_string()],
            test_source_dirs: vec![test_source_dir.to_string_lossy().to_string()],
            output_dir: output_dir.to_string_lossy().to_string(),
            test_output_dir: test_output_dir.to_string_lossy().to_string(),
            cache_dir: cache_dir.to_string_lossy().to_string(),
            ..BuildConfig::default()
        },
        dependencies: HashMap::new(),
        repositories: vec![],
    };

    fs::create_dir_all(&source_dir).unwrap();
    fs::create_dir_all(&test_source_dir).unwrap();

    fs::write(
        source_dir.join("Main.java"),
        "public class Main { public static void main(String[] args) {} }",
    )
    .unwrap();
    fs::write(
        test_source_dir.join("MainTest.java"),
        "public class MainTest {}",
    )
    .unwrap();

    config
}

#[tokio::test]
async fn test_orchestrator_compiles_main_and_test_sources() -> Result<(), Box<dyn std::error::Error>>
{
    let root = tempdir()?;
    let config = test_config(root.path(), "compile-flow");
    let mut orchestrator = BuildOrchestrator::new(config)?;

    let compiled = orchestrator.compile_sources().await?;
    assert_eq!(compiled, 1);
    assert!(
        std::path::Path::new(&orchestrator.config.build.output_dir)
            .join("Main.class")
            .exists()
    );

    orchestrator.compile_tests().await?;
    assert!(
        std::path::Path::new(&orchestrator.config.build.test_output_dir)
            .join("MainTest.class")
            .exists()
    );

    let options = TestOptions {
        pattern: None,
        verbose: false,
        fail_fast: false,
    };
    let tests = orchestrator.run_tests(&options).await?;
    assert_eq!(
        tests.total, 0,
        "no framework jar present, no tests should run"
    );

    Ok(())
}

#[tokio::test]
async fn test_orchestrator_build_with_clean_and_skip_tests()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempdir()?;
    let config = test_config(root.path(), "build-flow");
    let mut orchestrator = BuildOrchestrator::new(config)?;

    // place stale output so clean path is observable
    fs::create_dir_all(&orchestrator.config.build.output_dir)?;
    fs::write(
        std::path::Path::new(&orchestrator.config.build.output_dir).join("stale.class"),
        b"",
    )?;
    fs::create_dir_all(&orchestrator.config.build.test_output_dir)?;
    fs::write(
        std::path::Path::new(&orchestrator.config.build.test_output_dir).join("stale.class"),
        b"",
    )?;

    let result = orchestrator
        .build(&BuildOptions {
            clean: true,
            skip_tests: true,
            verbose: false,
        })
        .await?;

    assert_eq!(result.compiled_files, 1);
    assert!(result.test_result.is_none());

    Ok(())
}

#[tokio::test]
async fn test_orchestrator_reports_missing_source_directory()
-> Result<(), Box<dyn std::error::Error>> {
    let root = tempdir()?;
    let source_dir = root.path().join("missing-src");
    let config = ProjectConfig {
        project: ProjectInfo {
            name: "missing-source".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig {
            source_dirs: vec![source_dir.to_string_lossy().to_string()],
            ..BuildConfig::default()
        },
        dependencies: HashMap::new(),
        repositories: vec![],
    };
    let mut orchestrator = BuildOrchestrator::new(config)?;
    let result = orchestrator.compile_sources().await;

    assert!(
        result.is_err(),
        "missing source directory should return SourceDirNotFound"
    );
    if let Err(error) = result {
        let message = error.to_string();
        assert!(message.contains("Source directory not found"));
    }

    Ok(())
}
