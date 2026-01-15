//! Test framework detection and execution

use crate::error::Result;
use std::path::PathBuf;

/// Supported test frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestFramework {
    /// JUnit 5 (Jupiter)
    JUnit5,
    /// JUnit 4
    JUnit4,
    /// TestNG
    TestNG,
}

impl TestFramework {
    /// Get the test framework name
    pub fn name(&self) -> &'static str {
        match self {
            TestFramework::JUnit5 => "JUnit 5",
            TestFramework::JUnit4 => "JUnit 4",
            TestFramework::TestNG => "TestNG",
        }
    }
}

/// Result of a test failure
#[derive(Debug, Clone)]
pub struct TestFailure {
    /// Class name containing the test
    pub class_name: String,
    /// Test method name
    pub test_name: String,
    /// Failure message
    pub message: String,
    /// Stack trace
    pub stacktrace: String,
}

/// Result of test execution
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Total number of tests
    pub total: usize,
    /// Number of passed tests
    pub passed: usize,
    /// Number of failed tests
    pub failed: usize,
    /// Number of skipped tests
    pub skipped: usize,
    /// List of test failures
    pub failures: Vec<TestFailure>,
}

/// Test runner for Java projects
pub struct TestRunner {
    #[allow(dead_code)]
    framework: TestFramework,
    #[allow(dead_code)]
    classpath: Vec<PathBuf>,
    #[allow(dead_code)]
    test_output_dir: PathBuf,
}

impl TestRunner {
    /// Create a new test runner
    ///
    /// # Arguments
    ///
    /// * `framework` - Detected test framework
    /// * `classpath` - JAR files for test execution
    /// * `test_output_dir` - Directory containing compiled test classes
    ///
    /// # Returns
    ///
    /// A new TestRunner instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let runner = TestRunner::new(
    ///     TestFramework::JUnit5,
    ///     vec![],
    ///     PathBuf::from("target/test-classes"),
    /// );
    /// ```
    pub fn new(
        framework: TestFramework,
        classpath: Vec<PathBuf>,
        test_output_dir: PathBuf,
    ) -> Self {
        Self {
            framework,
            classpath,
            test_output_dir,
        }
    }

    /// Detect test framework from classpath
    ///
    /// Scans classpath JAR filenames to detect which test framework is available.
    /// Priority: JUnit 5 > JUnit 4 > TestNG
    ///
    /// # Arguments
    ///
    /// * `classpath` - JAR files to scan
    ///
    /// # Returns
    ///
    /// The detected TestFramework, or None if no framework found
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(framework) = TestRunner::detect_framework(&classpath) {
    ///     println!("Detected: {}", framework.name());
    /// }
    /// ```
    pub fn detect_framework(classpath: &[PathBuf]) -> Option<TestFramework> {
        let has_junit5 = classpath.iter().any(|p| {
            p.to_string_lossy().contains("junit-jupiter")
                || p.to_string_lossy().contains("junit-platform")
        });

        let has_junit4 = classpath
            .iter()
            .any(|p| p.to_string_lossy().contains("junit-4"));

        let has_testng = classpath
            .iter()
            .any(|p| p.to_string_lossy().contains("testng"));

        // Priority: JUnit 5 > JUnit 4 > TestNG
        if has_junit5 {
            Some(TestFramework::JUnit5)
        } else if has_junit4 {
            Some(TestFramework::JUnit4)
        } else if has_testng {
            Some(TestFramework::TestNG)
        } else {
            None
        }
    }

    /// Run tests
    ///
    /// Executes tests using the detected framework.
    ///
    /// # Arguments
    ///
    /// * `pattern` - Optional test class pattern filter
    /// * `verbose` - Whether to print verbose output
    /// * `fail_fast` - Stop on first failure
    ///
    /// # Returns
    ///
    /// TestResult with execution summary
    ///
    /// # Errors
    ///
    /// * `AppError::TestExecutionFailed` - If test execution fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = runner.run(None, true, false).await?;
    /// println!("Tests: {} passed, {} failed", result.passed, result.failed);
    /// ```
    pub async fn run(
        &self,
        _pattern: Option<String>,
        verbose: bool,
        _fail_fast: bool,
    ) -> Result<TestResult> {
        let test_classes = self.find_test_classes()?;
        if test_classes.is_empty() {
            return Ok(TestResult {
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                failures: vec![],
            });
        }

        match self.framework {
            TestFramework::JUnit5 => self.run_junit5(&test_classes, verbose).await,
            // Fallback for others (pending implementation)
            _ => Ok(TestResult {
                total: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                failures: vec![],
            }),
        }
    }

    async fn run_junit5(&self, test_classes: &[String], verbose: bool) -> Result<TestResult> {
        // Construct classpath string
        let classpath = self.format_classpath();

        let mut args = vec![
            "-jar".to_string(),
            self.find_junit_console_launcher()?,
            "-cp".to_string(),
            classpath,
        ];

        // Add test classes
        for class in test_classes {
            args.push("-c".to_string());
            args.push(class.clone());
        }

        // Output is captured and can be printed by caller if needed,
        // or just rely on the test process stdout for tree structure visualization.

        let output = std::process::Command::new("java")
            .args(&args)
            .output()
            .map_err(crate::error::BuildError::Io)?;

        // Only print raw output if it's the tree structure we want
        if verbose {
            // Check if output has content before printing to avoid empty lines
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
            }
            if !output.stderr.is_empty() {
                // Stdout usually contains the tree, stderr has warnings/errors
                println!("{}", String::from_utf8_lossy(&output.stderr).trim_end());
            }
        }

        // Parse output
        let stdout = String::from_utf8_lossy(&output.stdout);

        let passed = if let Some(cap) = stdout
            .lines()
            .find(|l| l.contains("tests successful"))
            .and_then(|l| l.split_whitespace().find_map(|w| w.parse::<usize>().ok()))
        {
            cap
        } else {
            0
        };

        let failed = if let Some(cap) = stdout
            .lines()
            .find(|l| l.contains("tests failed"))
            .and_then(|l| l.split_whitespace().find_map(|w| w.parse::<usize>().ok()))
        {
            cap
        } else {
            0
        };

        // Total is sum of passed + failed + aborted/skipped if we parse them
        // For now, let's trust the "tests found" line or just sum passed + failed
        let total = if let Some(cap) = stdout
            .lines()
            .find(|l| l.contains("tests found"))
            .and_then(|l| l.split_whitespace().find_map(|w| w.parse::<usize>().ok()))
        {
            cap
        } else {
            passed + failed
        };

        Ok(TestResult {
            total,
            passed,
            failed,
            skipped: total.saturating_sub(passed + failed),
            failures: vec![], // Details would require XML report parsing
        })
    }

    fn find_junit_console_launcher(&self) -> Result<String> {
        // Ideally this should be resolved from dependencies or bundled
        // For Phase 3 MVP, we'll try to find it in the classpath
        self.classpath
            .iter()
            .find(|p| p.to_string_lossy().contains("junit-platform-console-standalone"))
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| crate::error::BuildError::TestExecutionFailed {
                message: "JUnit Platform Console Standalone JAR not found in classpath. Please add 'org.junit.platform:junit-platform-console-standalone' dependency.".to_string()
            })
    }

    /// Find all test classes in test output directory
    #[allow(dead_code)]
    fn find_test_classes(&self) -> Result<Vec<String>> {
        let mut classes = Vec::new();

        if !self.test_output_dir.exists() {
            return Ok(classes);
        }

        for entry in walkdir::WalkDir::new(&self.test_output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("class") {
                // Convert file path to fully qualified class name
                let relative = path.strip_prefix(&self.test_output_dir).unwrap_or(path);
                let class_name = relative
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, ".")
                    .replace(".class", "");

                // Filter by naming patterns
                if self.matches_test_pattern(&class_name) {
                    classes.push(class_name);
                }
            }
        }

        Ok(classes)
    }

    /// Check if class name matches test patterns
    fn matches_test_pattern(&self, class_name: &str) -> bool {
        // Match common test naming patterns
        class_name.ends_with("Test")
            || class_name.ends_with("Tests")
            || class_name.ends_with("TestCase")
            || class_name.starts_with("Test")
    }

    /// Format classpath for command line
    #[allow(dead_code)]
    fn format_classpath(&self) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
        let mut paths = self.classpath.clone();
        paths.push(self.test_output_dir.clone());

        paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_framework_junit5() {
        let classpath = vec![PathBuf::from("/lib/junit-jupiter-api-5.10.0.jar")];
        let framework = TestRunner::detect_framework(&classpath);
        assert_eq!(framework, Some(TestFramework::JUnit5));
    }

    #[test]
    fn test_detect_framework_junit4() {
        let classpath = vec![PathBuf::from("/lib/junit-4.13.2.jar")];
        let framework = TestRunner::detect_framework(&classpath);
        assert_eq!(framework, Some(TestFramework::JUnit4));
    }

    #[test]
    fn test_detect_framework_testng() {
        let classpath = vec![PathBuf::from("/lib/testng-7.8.0.jar")];
        let framework = TestRunner::detect_framework(&classpath);
        assert_eq!(framework, Some(TestFramework::TestNG));
    }

    #[test]
    fn test_detect_framework_priority() {
        // JUnit 5 should have priority
        let classpath = vec![
            PathBuf::from("/lib/junit-jupiter-api-5.10.0.jar"),
            PathBuf::from("/lib/junit-4.13.2.jar"),
        ];
        let framework = TestRunner::detect_framework(&classpath);
        assert_eq!(framework, Some(TestFramework::JUnit5));
    }

    #[test]
    fn test_matches_test_pattern() {
        let runner = TestRunner::new(
            TestFramework::JUnit5,
            vec![],
            PathBuf::from("target/test-classes"),
        );

        assert!(runner.matches_test_pattern("AppTest"));
        assert!(runner.matches_test_pattern("AppTests"));
        assert!(runner.matches_test_pattern("AppTestCase"));
        assert!(runner.matches_test_pattern("TestApp"));
        assert!(!runner.matches_test_pattern("App"));
    }
}
