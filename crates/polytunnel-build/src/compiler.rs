//! Java compiler wrapper for javac

use polytunnel_core::{AppError, ProjectConfig, Result};
use std::path::PathBuf;
use std::process::Command;

/// Result of a compilation operation
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Whether compilation succeeded
    pub success: bool,
    /// Standard output from compiler
    pub stdout: String,
    /// Standard error from compiler
    pub stderr: String,
}

/// A Java compiler wrapper around javac
pub struct JavaCompiler {
    javac_path: PathBuf,
    java_version: String,
}

impl JavaCompiler {
    /// Create a new JavaCompiler instance
    ///
    /// Locates javac in PATH or uses JAVA_HOME environment variable.
    ///
    /// # Arguments
    ///
    /// * `config` - Project configuration for Java version info
    ///
    /// # Returns
    ///
    /// A new JavaCompiler instance
    ///
    /// # Errors
    ///
    /// * `AppError::JavacNotFound` - If javac cannot be found
    ///
    /// # Example
    ///
    /// ```ignore
    /// let compiler = JavaCompiler::new(&config)?;
    /// ```
    pub fn new(config: &ProjectConfig) -> Result<Self> {
        let javac_path = Self::find_javac()?;

        Ok(Self {
            javac_path,
            java_version: config.project.java_version.clone(),
        })
    }

    /// Compile Java source files
    ///
    /// # Arguments
    ///
    /// * `source_files` - Paths to .java source files to compile
    /// * `classpath` - JAR files for classpath
    /// * `output_dir` - Directory to place compiled .class files
    /// * `args` - Additional compiler arguments (encoding, debug flags, etc.)
    ///
    /// # Returns
    ///
    /// CompilationResult with success status and output
    ///
    /// # Errors
    ///
    /// * `AppError::CompilationFailed` - If javac returns non-zero exit code
    /// * `AppError::Io` - If file operations fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = compiler.compile(
    ///     vec![PathBuf::from("src/Main.java")],
    ///     vec![],
    ///     PathBuf::from("target/classes"),
    ///     vec!["-encoding".to_string(), "UTF-8".to_string()],
    /// )?;
    /// ```
    pub fn compile(
        &self,
        source_files: Vec<PathBuf>,
        classpath: Vec<PathBuf>,
        output_dir: PathBuf,
        args: Vec<String>,
    ) -> Result<CompilationResult> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&output_dir)?;

        // Build javac command
        let mut cmd = Command::new(&self.javac_path);

        // Set source/target version
        cmd.arg("-source").arg(&self.java_version);
        cmd.arg("-target").arg(&self.java_version);

        // Set output directory
        cmd.arg("-d").arg(&output_dir);

        // Add classpath if not empty
        if !classpath.is_empty() {
            let classpath_str = Self::format_classpath(&classpath);
            cmd.arg("-cp").arg(classpath_str);
        }

        // Add additional compiler arguments
        for arg in args {
            cmd.arg(arg);
        }

        // Add source files
        for file in &source_files {
            cmd.arg(file);
        }

        // Execute compilation
        let output = cmd.output()?;

        let success = output.status.success();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !success {
            return Err(AppError::CompilationFailed {
                message: format!(
                    "Compilation failed with {} file(s).\n{}",
                    source_files.len(),
                    stderr
                ),
            });
        }

        Ok(CompilationResult {
            success,
            stdout,
            stderr,
        })
    }

    /// Find javac executable in PATH or JAVA_HOME
    #[allow(clippy::collapsible_if)]
    fn find_javac() -> Result<PathBuf> {
        // Try to find javac in PATH
        if let Ok(output) = Command::new("which")
            .arg("javac")
            .output()
            .or_else(|_| Command::new("where").arg("javac").output())
        {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    return Ok(PathBuf::from(path_str));
                }
            }
        }

        // Try JAVA_HOME environment variable
        if let Ok(java_home) = std::env::var("JAVA_HOME") {
            let javac_path = if cfg!(windows) {
                PathBuf::from(&java_home).join("bin").join("javac.exe")
            } else {
                PathBuf::from(&java_home).join("bin").join("javac")
            };

            if javac_path.exists() {
                return Ok(javac_path);
            }
        }

        // Try direct javac on Windows
        if cfg!(windows) && Command::new("javac.exe").arg("-version").output().is_ok() {
            return Ok(PathBuf::from("javac.exe"));
        }

        Err(AppError::JavacNotFound)
    }

    /// Format classpath for command line
    fn format_classpath(paths: &[PathBuf]) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
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
    fn test_format_classpath() {
        let paths = vec![
            PathBuf::from("/usr/lib/lib1.jar"),
            PathBuf::from("/usr/lib/lib2.jar"),
        ];
        let result = JavaCompiler::format_classpath(&paths);
        assert!(result.contains("lib1.jar"));
        assert!(result.contains("lib2.jar"));
        if cfg!(windows) {
            assert!(result.contains(";"));
        } else {
            assert!(result.contains(":"));
        }
    }
}
