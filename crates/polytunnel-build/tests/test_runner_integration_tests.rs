use polytunnel_build::{TestFramework, TestRunner};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::tempdir;

fn java_tools_available() -> bool {
    Command::new("java")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn build_fake_launcher(root: &Path, output_lines: &[&str]) -> std::path::PathBuf {
    let launcher_class = "PolytunnelLauncher";
    let src_dir = root.join("launcher-src");
    let out_dir = root.join("launcher-bin");
    let jar_path = root.join("junit-platform-console-standalone-1.10.2.jar");

    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    let mut source = String::new();
    source.push_str("public class ");
    source.push_str(launcher_class);
    source.push_str(" {\n    public static void main(String[] args) {\n");
    for line in output_lines {
        source.push_str("        System.out.println(\"");
        source.push_str(&line.replace('\\', "\\\\").replace('\"', "\\\""));
        source.push_str("\");\n");
    }
    source.push_str("    }\n}\n");

    let java_file = src_dir.join(format!("{}.java", launcher_class));
    fs::write(&java_file, source).unwrap();

    let javac_status = Command::new("javac")
        .arg("-d")
        .arg(&out_dir)
        .arg(&java_file)
        .status()
        .expect("javac should run");
    assert!(javac_status.success());

    let jar_status = Command::new("jar")
        .arg("cfe")
        .arg(&jar_path)
        .arg(launcher_class)
        .arg("-C")
        .arg(&out_dir)
        .arg(".")
        .status()
        .expect("jar should run");
    assert!(jar_status.success());

    jar_path
}

#[tokio::test]
async fn test_run_uses_junit5_path_when_launcher_is_present() {
    if !java_tools_available() {
        return;
    }

    let root = tempdir().unwrap();
    let test_output = root.path().join("target/test-classes");
    fs::create_dir_all(&test_output).unwrap();

    let launcher = build_fake_launcher(root.path(), &[]);

    let runner = TestRunner::new(TestFramework::JUnit5, vec![launcher], test_output.clone());
    let result = runner.run(None, false, false).await.unwrap();

    assert_eq!(result.total, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
}

#[tokio::test]
async fn test_run_junit5_parses_java_output() {
    if !java_tools_available() {
        return;
    }

    let root = tempdir().unwrap();
    let test_output = root.path().join("target/test-classes");
    fs::create_dir_all(&test_output).unwrap();
    fs::write(test_output.join("HelloTest.class"), b"fake class").unwrap();
    let launcher = build_fake_launcher(
        root.path(),
        &["tests found 2", "tests successful 2", "tests failed 0"],
    );

    let runner = TestRunner::new(TestFramework::JUnit5, vec![launcher], test_output);
    let result = runner.run(None, true, false).await.unwrap();

    assert_eq!(result.total, 2);
    assert_eq!(result.passed, 2);
    assert_eq!(result.failed, 0);
}

#[tokio::test]
async fn test_run_junit5_uses_pass_failed_fallback() {
    if !java_tools_available() {
        return;
    }

    let root = tempdir().unwrap();
    let test_output = root.path().join("target/test-classes");
    fs::create_dir_all(&test_output).unwrap();
    fs::write(test_output.join("MathTest.class"), b"fake class").unwrap();
    let launcher = build_fake_launcher(root.path(), &["tests successful 1"]);

    let runner = TestRunner::new(TestFramework::JUnit5, vec![launcher], test_output);
    let result = runner.run(None, false, false).await.unwrap();

    assert_eq!(result.total, 1);
    assert_eq!(result.passed, 1);
    assert_eq!(result.failed, 0);
}

#[tokio::test]
async fn test_run_junit5_missing_launcher_is_error() {
    let root = tempdir().unwrap();
    let test_output = root.path().join("target/test-classes");
    fs::create_dir_all(&test_output).unwrap();
    fs::write(test_output.join("HelloTest.class"), b"fake class").unwrap();

    let runner = TestRunner::new(TestFramework::JUnit5, vec![], test_output);
    let result = runner.run(None, false, false).await;

    let err = result.expect_err("run should fail without junit console jar");
    assert!(
        err.to_string()
            .contains("JUnit Platform Console Standalone")
    );
}

#[tokio::test]
async fn test_run_falls_back_for_unsupported_framework() {
    let root = tempfile::tempdir().unwrap();
    let test_output = root.path().join("target/test-classes");
    fs::create_dir_all(&test_output).unwrap();
    fs::write(test_output.join("NotATestCase.class"), b"fake class").unwrap();

    let runner = TestRunner::new(TestFramework::TestNG, vec![], test_output);
    let result = runner.run(None, false, false).await.unwrap();

    assert_eq!(result.total, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
}
