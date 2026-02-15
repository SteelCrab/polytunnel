use polytunnel_build::{BuildError, BuildOptions, BuildOrchestrator};
use polytunnel_core::{BuildConfig, ProjectConfig, ProjectInfo};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

fn java_tools_available() -> bool {
    Command::new("java").arg("-version").output().is_ok()
        && Command::new("javac").arg("-version").output().is_ok()
}

fn create_config(root: &Path) -> ProjectConfig {
    let build = BuildConfig {
        source_dirs: vec![root.join("src/main/java").to_string_lossy().to_string()],
        test_source_dirs: vec![root.join("src/test/java").to_string_lossy().to_string()],
        output_dir: root.join("target/classes").to_string_lossy().to_string(),
        test_output_dir: root
            .join("target/test-classes")
            .to_string_lossy()
            .to_string(),
        cache_dir: root.join(".polytunnel/cache").to_string_lossy().to_string(),
        ..BuildConfig::default()
    };

    ProjectConfig {
        project: ProjectInfo {
            name: "sample".to_string(),
            java_version: "17".to_string(),
        },
        build,
        dependencies: HashMap::new(),
        repositories: vec![],
    }
}

fn write_java(root: &Path, relative: &str, source: &str) -> PathBuf {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, source).unwrap();
    path
}

#[tokio::test]
async fn test_orchestrator_compiles_main_and_test_sources() {
    if !java_tools_available() {
        eprintln!(
            "skipping test_orchestrator_compiles_main_and_test_sources: java/javac not found"
        );
        return;
    }

    let workspace = tempdir().unwrap();
    let root = workspace.path();

    write_java(
        root,
        "src/main/java/com/example/Main.java",
        r#"package com.example;
public class Main {
    public static void main(String[] args) {}
}
"#,
    );
    write_java(
        root,
        "src/test/java/com/example/MainTest.java",
        r#"package com.example;
public class MainTest {
    public String run() {
        Main.main(new String[0]);
        return "ok";
    }
}
"#,
    );

    let mut orchestrator = BuildOrchestrator::new(create_config(root)).unwrap();
    let compiled = orchestrator.compile_sources().await.unwrap();
    orchestrator.compile_tests().await.unwrap();

    assert_eq!(compiled, 1);
    assert!(root.join("target/classes/com/example/Main.class").exists());
    assert!(
        root.join("target/test-classes/com/example/MainTest.class")
            .exists()
    );
}

#[tokio::test]
async fn test_orchestrator_build_clean_skip_tests_rebuilds_outputs() {
    if !java_tools_available() {
        eprintln!(
            "skipping test_orchestrator_build_clean_skip_tests_rebuilds_outputs: java/javac not found"
        );
        return;
    }

    let workspace = tempdir().unwrap();
    let root = workspace.path();
    write_java(
        root,
        "src/main/java/com/example/Main.java",
        r#"package com.example;
public class Main {
    public static int value() { return 42; }
}
"#,
    );

    let stale = root.join("target/classes/stale.txt");
    fs::create_dir_all(stale.parent().unwrap()).unwrap();
    fs::write(&stale, "old").unwrap();

    let mut orchestrator = BuildOrchestrator::new(create_config(root)).unwrap();
    let result = orchestrator
        .build(&BuildOptions {
            clean: true,
            skip_tests: true,
            verbose: false,
        })
        .await
        .unwrap();

    assert_eq!(result.compiled_files, 1);
    assert!(result.test_result.is_none());
    assert!(!stale.exists());
    assert!(root.join("target/classes/com/example/Main.class").exists());
}

#[tokio::test]
async fn test_orchestrator_build_with_tests_without_framework_returns_zero_test_result() {
    if !java_tools_available() {
        eprintln!(
            "skipping test_orchestrator_build_with_tests_without_framework_returns_zero_test_result: java/javac not found"
        );
        return;
    }

    let workspace = tempdir().unwrap();
    let root = workspace.path();
    write_java(
        root,
        "src/main/java/com/example/Main.java",
        r#"package com.example;
public class Main {
    public static int add(int a, int b) { return a + b; }
}
"#,
    );
    write_java(
        root,
        "src/test/java/com/example/MainTest.java",
        r#"package com.example;
public class MainTest {
    public int run() {
        return Main.add(1, 2);
    }
}
"#,
    );

    let mut orchestrator = BuildOrchestrator::new(create_config(root)).unwrap();
    let result = orchestrator
        .build(&BuildOptions {
            clean: false,
            skip_tests: false,
            verbose: false,
        })
        .await
        .unwrap();

    assert_eq!(result.compiled_files, 1);
    let test_result = result.test_result.expect("test result should exist");
    assert_eq!(test_result.total, 0);
    assert_eq!(test_result.passed, 0);
    assert_eq!(test_result.failed, 0);
    assert!(
        root.join("target/test-classes/com/example/MainTest.class")
            .exists()
    );
}

#[tokio::test]
async fn test_orchestrator_compile_sources_errors_for_missing_source_dir() {
    if !java_tools_available() {
        eprintln!(
            "skipping test_orchestrator_compile_sources_errors_for_missing_source_dir: java/javac not found"
        );
        return;
    }

    let workspace = tempdir().unwrap();
    let root = workspace.path();
    let mut config = create_config(root);
    config.build.source_dirs = vec![root.join("missing-src").to_string_lossy().to_string()];

    let mut orchestrator = BuildOrchestrator::new(config).unwrap();
    let err = orchestrator.compile_sources().await.unwrap_err();
    assert!(matches!(err, BuildError::SourceDirNotFound { .. }));
}

#[test]
fn test_orchestrator_clean_removes_output_directories() {
    if !java_tools_available() {
        eprintln!(
            "skipping test_orchestrator_clean_removes_output_directories: java/javac not found"
        );
        return;
    }

    let workspace = tempdir().unwrap();
    let root = workspace.path();
    let output_dir = root.join("target/classes");
    let test_output_dir = root.join("target/test-classes");

    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(&test_output_dir).unwrap();
    fs::write(output_dir.join("stale.class"), "").unwrap();
    fs::write(test_output_dir.join("stale.class"), "").unwrap();

    let orchestrator = BuildOrchestrator::new(create_config(root)).unwrap();
    orchestrator.clean().unwrap();

    assert!(!output_dir.exists());
    assert!(!test_output_dir.exists());
}
