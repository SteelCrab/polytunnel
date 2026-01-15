//! Build orchestrator for managing the complete build lifecycle.
//!
//! Coordinates compilation, testing, and artifact management.

use crate::error::{BuildError, Result};
use crate::{BuildCache, ClasspathBuilder, JavaCompiler, TestResult, TestRunner};
use polytunnel_core::ProjectConfig;
use std::path::PathBuf;
use std::time::Instant;

/// Options for build command
#[derive(Debug, Clone)]
pub struct BuildOptions {
    /// Clean build (remove existing outputs)
    pub clean: bool,
    /// Skip test compilation and execution
    pub skip_tests: bool,
    /// Verbose output
    pub verbose: bool,
}

/// Options for test command
#[derive(Debug, Clone)]
pub struct TestOptions {
    /// Test pattern/filter (optional)
    pub pattern: Option<String>,
    /// Verbose output
    pub verbose: bool,
    /// Stop on first failure
    pub fail_fast: bool,
}

/// Result of a build operation
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Number of compiled files
    pub compiled_files: usize,
    /// Test results (if tests ran)
    pub test_result: Option<TestResult>,
    /// Build duration
    pub duration: std::time::Duration,
}

/// Central orchestrator for build operations
pub struct BuildOrchestrator {
    pub config: ProjectConfig,
    classpath_builder: ClasspathBuilder,
    compiler: JavaCompiler,
    #[allow(dead_code)]
    test_runner: Option<TestRunner>,
    incremental: BuildCache,
}

impl BuildOrchestrator {
    /// Create a new build orchestrator
    ///
    /// # Arguments
    ///
    /// * `config` - Project configuration from polytunnel.toml
    ///
    /// # Returns
    ///
    /// A new BuildOrchestrator instance
    ///
    /// # Errors
    ///
    /// * `BuildError::JavacNotFound` - If javac cannot be found in PATH
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;
    /// let orchestrator = BuildOrchestrator::new(config)?;
    /// ```
    pub fn new(config: ProjectConfig) -> Result<Self> {
        let compiler = JavaCompiler::new(&config)?;
        let classpath_builder = ClasspathBuilder::new(config.clone());
        let incremental = BuildCache::new(&config)?;

        Ok(Self {
            config,
            classpath_builder,
            compiler,
            test_runner: None,
            incremental,
        })
    }

    /// Execute full build (compile + tests)
    ///
    /// # Arguments
    ///
    /// * `options` - Build options (clean, skip_tests, verbose)
    ///
    /// # Returns
    ///
    /// BuildResult with compilation count and optional test results
    ///
    /// # Errors
    ///
    /// * `BuildError::CompilationFailed` - If compilation fails
    /// * `BuildError::TestExecutionFailed` - If test execution fails (when not skipped)
    /// * `BuildError::SourceDirNotFound` - If source directories don't exist
    ///
    /// # Example
    ///
    /// ```ignore
    /// let options = BuildOptions {
    ///     clean: false,
    ///     skip_tests: false,
    ///     verbose: true,
    /// };
    /// let result = orchestrator.build(&options).await?;
    /// println!("Compiled {} files", result.compiled_files);
    /// ```
    pub async fn build(&mut self, options: &BuildOptions) -> Result<BuildResult> {
        let start = Instant::now();

        // 1. Resolve and download dependencies
        if options.verbose {
            println!("Resolving dependencies...");
        }
        self.resolve_dependencies(options.verbose).await?;

        // 2. Clean if requested
        if options.clean {
            if options.verbose {
                println!("Cleaning build artifacts...");
            }
            self.clean()?;
        }

        // 3. Compile main sources
        if options.verbose {
            println!("Compiling main sources...");
        }
        let compiled = self.compile_sources().await?;

        // 4. Compile and run tests (if not skipped)
        let test_result = if !options.skip_tests {
            if options.verbose {
                println!("Compiling test sources...");
            }
            self.compile_tests().await?;

            if options.verbose {
                println!("Running tests...");
            }
            let test_opts = TestOptions {
                pattern: None,
                verbose: options.verbose,
                fail_fast: false,
            };
            Some(self.run_tests(&test_opts).await?)
        } else {
            None
        };

        let duration = start.elapsed();

        Ok(BuildResult {
            compiled_files: compiled,
            test_result,
            duration,
        })
    }

    /// Resolve dependencies
    pub async fn resolve_dependencies(&mut self, verbose: bool) -> Result<()> {
        self.classpath_builder
            .build_classpath(&self.config.build.cache_dir, verbose)
            .await
            .map(|_| ())
    }

    /// Compile main sources only
    pub async fn compile_sources(&mut self) -> Result<usize> {
        let source_dirs = &self.config.build.source_dirs;
        let output_dir = PathBuf::from(&self.config.build.output_dir);
        let compiler_args = self.config.build.compiler_args.clone();

        // Get compile classpath
        let classpaths = self.classpath_builder.get_cached_classpath();
        let classpath = &classpaths.compile_classpath;

        // Find all Java source files
        let source_files = self.find_java_files(source_dirs)?;

        if source_files.is_empty() {
            return Ok(0);
        }

        // Compile
        let _result = self.compiler.compile(
            source_files.clone(),
            classpath.clone(),
            output_dir,
            compiler_args,
        )?;

        // Update cache
        self.incremental.update_for_sources(&source_files)?;

        Ok(source_files.len())
    }

    /// Compile test sources only
    pub async fn compile_tests(&mut self) -> Result<()> {
        let test_source_dirs = &self.config.build.test_source_dirs;
        // ... (rest is unchanged logic, just ensuring pub)
        let test_output_dir = PathBuf::from(&self.config.build.test_output_dir);
        let test_compiler_args = self.config.build.test_compiler_args.clone();

        // Get test classpath
        let classpaths = self.classpath_builder.get_cached_classpath();
        let mut test_classpath = classpaths.test_classpath.clone();

        // Add main output dir to classpath so tests can see main classes
        let output_dir = PathBuf::from(&self.config.build.output_dir);
        test_classpath.push(output_dir);

        // Find all test Java source files
        let test_files = self.find_java_files(test_source_dirs)?;

        if test_files.is_empty() {
            return Ok(());
        }

        // Compile tests
        let _result = self.compiler.compile(
            test_files.clone(),
            test_classpath.clone(),
            test_output_dir,
            test_compiler_args,
        )?;

        // Update cache
        self.incremental.update_for_sources(&test_files)?;

        Ok(())
    }

    /// Run tests
    pub async fn run_tests(&mut self, options: &TestOptions) -> Result<TestResult> {
        let test_output_dir = PathBuf::from(&self.config.build.test_output_dir);

        // Construct full classpath for tests (compile + test + test_output + output)
        let classpaths = self.classpath_builder.get_cached_classpath();
        let mut full_classpath = classpaths.test_classpath.clone();

        // Add main classes and test classes to classpath
        full_classpath.push(PathBuf::from(&self.config.build.output_dir));
        full_classpath.push(test_output_dir.clone());

        // Detect framework
        let framework =
            if let Some(fw) = crate::test_runner::TestRunner::detect_framework(&full_classpath) {
                fw
            } else {
                if options.verbose {
                    println!("No supported test framework detected.");
                }
                return Ok(TestResult {
                    total: 0,
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                    failures: vec![],
                });
            };

        if options.verbose {
            println!("Detected test framework: {}", framework.name());
        }

        let runner =
            crate::test_runner::TestRunner::new(framework, full_classpath, test_output_dir);

        runner
            .run(options.pattern.clone(), options.verbose, options.fail_fast)
            .await
    }

    /// Clean build artifacts
    pub fn clean(&self) -> Result<()> {
        let output_dir = PathBuf::from(&self.config.build.output_dir);
        let test_output_dir = PathBuf::from(&self.config.build.test_output_dir);

        if output_dir.exists() {
            std::fs::remove_dir_all(&output_dir)?;
        }
        if test_output_dir.exists() {
            std::fs::remove_dir_all(&test_output_dir)?;
        }

        Ok(())
    }

    /// Find all Java files in given directories
    fn find_java_files(&self, dirs: &[String]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for dir_str in dirs {
            let dir = PathBuf::from(dir_str);
            if !dir.exists() {
                return Err(BuildError::SourceDirNotFound {
                    path: dir_str.clone(),
                });
            }

            for entry in walkdir::WalkDir::new(&dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("java") {
                    files.push(entry.path().to_path_buf());
                }
            }
        }

        Ok(files)
    }
}
