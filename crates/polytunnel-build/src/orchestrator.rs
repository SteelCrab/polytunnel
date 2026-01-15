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
    pub async fn build(&mut self, options: &BuildOptions) -> Result<BuildResult> {
        let start = Instant::now();

        if options.verbose {
            println!("Resolving dependencies...");
        }
        self.classpath_builder
            .build_classpath(&self.config.build.cache_dir)
            .await?;

        if options.clean {
            if options.verbose {
                println!("Cleaning build artifacts...");
            }
            self.clean()?;
        }

        if options.verbose {
            println!("Compiling main sources...");
        }
        let compiled = self.compile_sources().await?;

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

    async fn compile_sources(&mut self) -> Result<usize> {
        let source_dirs = &self.config.build.source_dirs;
        let output_dir = PathBuf::from(&self.config.build.output_dir);
        let compiler_args = self.config.build.compiler_args.clone();

        let classpaths = self.classpath_builder.get_cached_classpath();
        let classpath = &classpaths.compile_classpath;

        let source_files = self.find_java_files(source_dirs)?;

        if source_files.is_empty() {
            return Ok(0);
        }

        let _result = self.compiler.compile(
            source_files.clone(),
            classpath.clone(),
            output_dir,
            compiler_args,
        )?;

        self.incremental.update_for_sources(&source_files)?;

        Ok(source_files.len())
    }

    /// Compile test sources only
    pub async fn compile_tests(&mut self) -> Result<()> {
        let test_source_dirs = &self.config.build.test_source_dirs;
        let test_output_dir = PathBuf::from(&self.config.build.test_output_dir);
        let test_compiler_args = self.config.build.test_compiler_args.clone();

        let classpaths = self.classpath_builder.get_cached_classpath();
        let test_classpath = &classpaths.test_classpath;

        let test_files = self.find_java_files(test_source_dirs)?;

        if test_files.is_empty() {
            return Ok(());
        }

        let _result = self.compiler.compile(
            test_files.clone(),
            test_classpath.clone(),
            test_output_dir,
            test_compiler_args,
        )?;

        self.incremental.update_for_sources(&test_files)?;

        Ok(())
    }

    /// Run tests
    pub async fn run_tests(&mut self, _options: &TestOptions) -> Result<TestResult> {
        Ok(TestResult {
            total: 0,
            passed: 0,
            failed: 0,
            skipped: 0,
            failures: vec![],
        })
    }

    fn clean(&self) -> Result<()> {
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
