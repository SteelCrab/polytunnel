use polytunnel_build::compiler::JavaCompiler;
use polytunnel_core::ProjectConfig;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_java_compiler_compiles_sources() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let source_file = temp_dir.path().join("Hello.java");
    let output_dir = temp_dir.path().join("out");

    fs::write(
        &source_file,
        r#"public class Hello {
            public static void main(String[] args) {}
        }"#,
    )?;

    let compiler = JavaCompiler::new(&ProjectConfig::new("compiler-success"))?;
    let result = compiler.compile(
        vec![source_file.clone()],
        vec![],
        output_dir.clone(),
        vec![],
    )?;

    assert!(result.success);
    assert!(
        output_dir.join("Hello.class").exists(),
        "javac should emit compiled class in output directory"
    );

    Ok(())
}

#[test]
fn test_java_compiler_reports_syntax_errors() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let source_file = temp_dir.path().join("Broken.java");
    let output_dir = temp_dir.path().join("out");

    fs::write(
        &source_file,
        "public class Broken { this is not valid java;",
    )?;

    let compiler = JavaCompiler::new(&ProjectConfig::new("compiler-failure"))?;
    let result = compiler.compile(vec![source_file], vec![], output_dir, vec![]);

    let error = result.expect_err("compilation should fail with invalid source");
    assert!(
        error.to_string().contains("Compilation failed"),
        "error message should include failure context: {error}"
    );

    Ok(())
}
