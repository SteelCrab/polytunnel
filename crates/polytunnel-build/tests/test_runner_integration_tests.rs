use polytunnel_build::{BuildError, TestFramework, TestRunner};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tempfile::tempdir;
use tokio::sync::Mutex;

fn env_lock() -> &'static Mutex<()> {
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

struct PathGuard {
    original_path: OsString,
}

impl Drop for PathGuard {
    fn drop(&mut self) {
        // SAFETY: tests mutate process env only while holding env_lock().
        unsafe {
            std::env::set_var("PATH", &self.original_path);
        }
    }
}

fn prepend_path(dir: &Path) -> PathGuard {
    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let new_path = std::env::join_paths(
        std::iter::once(dir.to_path_buf()).chain(std::env::split_paths(&original_path)),
    )
    .expect("failed to join PATH");

    // SAFETY: tests mutate process env only while holding env_lock().
    unsafe {
        std::env::set_var("PATH", new_path);
    }

    PathGuard { original_path }
}

fn write_class_file(root: &Path, relative: &str) -> PathBuf {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create class parent");
    }
    fs::write(&path, []).expect("create class file");
    path
}

fn create_fake_java(bin_dir: &Path, script_body: &str) {
    if cfg!(windows) {
        let path = bin_dir.join("java.cmd");
        fs::write(path, script_body).expect("write fake java cmd");
    } else {
        use std::os::unix::fs::PermissionsExt;

        let path = bin_dir.join("java");
        fs::write(&path, script_body).expect("write fake java script");
        let mut perms = fs::metadata(&path)
            .expect("read fake java metadata")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).expect("set fake java executable permissions");
    }
}

#[test]
fn test_framework_name_is_stable() {
    assert_eq!(TestFramework::JUnit5.name(), "JUnit 5");
    assert_eq!(TestFramework::JUnit4.name(), "JUnit 4");
    assert_eq!(TestFramework::TestNG.name(), "TestNG");
}

#[test]
fn test_detect_framework_returns_none_when_missing() {
    let classpath = vec![PathBuf::from("/tmp/app.jar")];
    assert_eq!(TestRunner::detect_framework(&classpath), None);
}

#[tokio::test]
async fn test_run_returns_empty_when_test_output_dir_missing() {
    let runner = TestRunner::new(
        TestFramework::JUnit5,
        vec![PathBuf::from(
            "/tmp/junit-platform-console-standalone-1.10.0.jar",
        )],
        PathBuf::from("/definitely/missing/test-classes"),
    );

    let result = runner.run(None, false, false).await.unwrap();
    assert_eq!(result.total, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
}

#[tokio::test]
async fn test_run_junit4_uses_fallback_result() {
    let temp = tempdir().unwrap();
    write_class_file(temp.path(), "com/example/AppTest.class");

    let runner = TestRunner::new(TestFramework::JUnit4, vec![], temp.path().to_path_buf());
    let result = runner.run(None, false, false).await.unwrap();

    assert_eq!(result.total, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
}

#[tokio::test]
async fn test_run_junit5_requires_console_launcher() {
    let temp = tempdir().unwrap();
    write_class_file(temp.path(), "com/example/AppTest.class");

    let runner = TestRunner::new(
        TestFramework::JUnit5,
        vec![PathBuf::from("/tmp/not-a-console-launcher.jar")],
        temp.path().to_path_buf(),
    );

    let err = runner.run(None, false, false).await.unwrap_err();
    assert!(matches!(err, BuildError::TestExecutionFailed { .. }));
    assert!(
        err.to_string()
            .contains("JUnit Platform Console Standalone JAR not found")
    );
}

#[tokio::test]
async fn test_run_junit5_parses_found_line() {
    let temp = tempdir().unwrap();
    let bin_dir = temp.path().join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    write_class_file(temp.path(), "com/example/AppTest.class");
    write_class_file(temp.path(), "com/example/Helper.class");

    if cfg!(windows) {
        create_fake_java(
            &bin_dir,
            "@echo off\r\necho 8 tests found\r\necho 6 tests successful\r\necho 2 tests failed\r\necho warning 1>&2\r\nexit /b 0\r\n",
        );
    } else {
        create_fake_java(
            &bin_dir,
            "#!/bin/sh\nprintf '%s\\n' '8 tests found' '6 tests successful' '2 tests failed'\nprintf '%s\\n' 'warning' 1>&2\nexit 0\n",
        );
    }

    let _lock = env_lock().lock().await;
    let _path_guard = prepend_path(&bin_dir);

    let runner = TestRunner::new(
        TestFramework::JUnit5,
        vec![PathBuf::from(
            "/tmp/junit-platform-console-standalone-1.10.0.jar",
        )],
        temp.path().to_path_buf(),
    );

    let result = runner.run(None, true, false).await.unwrap();
    assert_eq!(result.total, 8);
    assert_eq!(result.passed, 6);
    assert_eq!(result.failed, 2);
    assert_eq!(result.skipped, 0);
    assert!(result.failures.is_empty());
}

#[tokio::test]
async fn test_run_junit5_uses_passed_failed_sum_when_found_missing() {
    let temp = tempdir().unwrap();
    let bin_dir = temp.path().join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    write_class_file(temp.path(), "com/example/AppTests.class");

    if cfg!(windows) {
        create_fake_java(
            &bin_dir,
            "@echo off\r\necho 3 tests successful\r\necho 1 tests failed\r\nexit /b 0\r\n",
        );
    } else {
        create_fake_java(
            &bin_dir,
            "#!/bin/sh\nprintf '%s\\n' '3 tests successful' '1 tests failed'\nexit 0\n",
        );
    }

    let _lock = env_lock().lock().await;
    let _path_guard = prepend_path(&bin_dir);

    let runner = TestRunner::new(
        TestFramework::JUnit5,
        vec![PathBuf::from(
            "/tmp/junit-platform-console-standalone-1.10.0.jar",
        )],
        temp.path().to_path_buf(),
    );

    let result = runner.run(None, false, false).await.unwrap();
    assert_eq!(result.total, 4);
    assert_eq!(result.passed, 3);
    assert_eq!(result.failed, 1);
    assert_eq!(result.skipped, 0);
}

#[tokio::test]
async fn test_run_falls_back_for_unsupported_framework() {
    let temp = tempdir().unwrap();
    write_class_file(temp.path(), "com/example/NotATestCase.class");

    let runner = TestRunner::new(TestFramework::TestNG, vec![], temp.path().to_path_buf());
    let result = runner.run(None, false, false).await.unwrap();

    assert_eq!(result.total, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.skipped, 0);
}
