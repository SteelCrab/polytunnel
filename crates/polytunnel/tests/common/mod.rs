#![allow(dead_code)]

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestProject {
    dir: TempDir,
    toml_header: String,
    deps: Vec<String>,
    raw_override: Option<String>,
}

impl TestProject {
    pub fn new() -> Self {
        Self::new_named("demo")
    }

    pub fn new_named(name: &str) -> Self {
        let dir = TempDir::new().expect("create tempdir");
        fs::create_dir_all(dir.path().join("src/main/java")).unwrap();
        fs::create_dir_all(dir.path().join("src/test/java")).unwrap();

        let project = Self {
            dir,
            toml_header: base_toml(name),
            deps: Vec::new(),
            raw_override: None,
        };
        project.flush_toml();
        project
    }

    pub fn with_main(self, class: &str, main_body: &str) -> Self {
        let source = format!(
            "package {package}; public class {simple} {{ public static void main(String[] args) {{ {body} }} }}",
            package = package_of(class),
            simple = simple_of(class),
            body = main_body,
        );
        write_java(self.path(), "src/main/java", class, &source);
        self
    }

    pub fn with_test(self, class: &str, class_body: &str) -> Self {
        let source = format!(
            "package {package}; public class {simple} {{ {body} }}",
            package = package_of(class),
            simple = simple_of(class),
            body = class_body,
        );
        write_java(self.path(), "src/test/java", class, &source);
        self
    }

    pub fn with_dependency(mut self, coord: &str) -> Self {
        let (ga, version) = split_coord(coord);
        self.deps.push(format!("\"{ga}\" = \"{version}\""));
        self.flush_toml();
        self
    }

    pub fn with_raw_toml(mut self, toml: &str) -> Self {
        self.raw_override = Some(toml.to_string());
        self.flush_toml();
        self
    }

    pub fn pt(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_pt"));
        cmd.current_dir(self.dir.path()).args(args);
        cmd
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    fn flush_toml(&self) {
        let contents = match &self.raw_override {
            Some(raw) => raw.clone(),
            None => {
                let mut out = self.toml_header.clone();
                if !self.deps.is_empty() {
                    out.push_str("\n[dependencies]\n");
                    for dep in &self.deps {
                        out.push_str(dep);
                        out.push('\n');
                    }
                }
                out
            }
        };
        fs::write(self.dir.path().join("polytunnel.toml"), contents).unwrap();
    }
}

impl Default for TestProject {
    fn default() -> Self {
        Self::new()
    }
}

pub fn java_toolchain_available() -> bool {
    std::process::Command::new("javac")
        .arg("--version")
        .output()
        .is_ok()
        && std::process::Command::new("java")
            .arg("--version")
            .output()
            .is_ok()
}

fn base_toml(name: &str) -> String {
    format!(
        r#"[project]
name = "{name}"
java_version = "17"

[build]
source_dirs = ["src/main/java"]
test_source_dirs = ["src/test/java"]
output_dir = "target/classes"
test_output_dir = "target/test-classes"
cache_dir = ".polytunnel/cache"
"#
    )
}

fn package_of(class: &str) -> &str {
    class.rsplit_once('.').map(|(pkg, _)| pkg).unwrap_or("")
}

fn simple_of(class: &str) -> &str {
    class
        .rsplit_once('.')
        .map(|(_, name)| name)
        .unwrap_or(class)
}

fn split_coord(coord: &str) -> (&str, &str) {
    let (ga, version) = coord
        .rsplit_once(':')
        .expect("dependency coord must be groupId:artifactId:version");
    (ga, version)
}

fn write_java(root: &Path, source_root: &str, class: &str, contents: &str) {
    let pkg = package_of(class);
    let simple = simple_of(class);
    let mut target: PathBuf = root.join(source_root);
    if !pkg.is_empty() {
        for segment in pkg.split('.') {
            target.push(segment);
        }
    }
    fs::create_dir_all(&target).unwrap();
    target.push(format!("{simple}.java"));
    fs::write(target, contents).unwrap();
}
