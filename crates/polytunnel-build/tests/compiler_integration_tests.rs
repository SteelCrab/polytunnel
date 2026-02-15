use polytunnel_build::{BuildError, JavaCompiler};
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

fn create_config() -> ProjectConfig {
    ProjectConfig {
        project: ProjectInfo {
            name: "compiler-test".to_string(),
            java_version: "17".to_string(),
        },
        build: BuildConfig::default(),
        dependencies: HashMap::new(),
        repositories: vec![],
    }
}

fn write_java_file(root: &Path, relative: &str, content: &str) -> PathBuf {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_compiler_compiles_valid_source() {
    if !java_tools_available() {
        eprintln!("skipping test_compiler_compiles_valid_source: java/javac not found");
        return;
    }

    let temp = tempdir().unwrap();
    let source = write_java_file(
        temp.path(),
        "src/main/java/com/example/Main.java",
        r#"package com.example;
public class Main {
    public static void main(String[] args) {}
}
"#,
    );
    let output_dir = temp.path().join("target/classes");

    let compiler = JavaCompiler::new(&create_config()).unwrap();
    let result = compiler
        .compile(
            vec![source],
            vec![temp.path().join("libs/unused.jar")],
            output_dir.clone(),
            vec!["-encoding".to_string(), "UTF-8".to_string()],
        )
        .unwrap();

    assert!(result.success);
    assert!(result.stderr.is_empty());
    assert!(output_dir.join("com/example/Main.class").exists());
}

#[test]
fn test_compiler_returns_error_for_invalid_source() {
    if !java_tools_available() {
        eprintln!("skipping test_compiler_returns_error_for_invalid_source: java/javac not found");
        return;
    }

    let temp = tempdir().unwrap();
    let source = write_java_file(
        temp.path(),
        "src/main/java/com/example/Broken.java",
        r#"package com.example;
public class Broken {
    public static void broken( {
}
"#,
    );

    let compiler = JavaCompiler::new(&create_config()).unwrap();
    let err = compiler
        .compile(
            vec![source],
            vec![],
            temp.path().join("target/classes"),
            vec![],
        )
        .unwrap_err();

    match err {
        BuildError::CompilationFailed { message } => {
            assert!(message.contains("Compilation failed"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
