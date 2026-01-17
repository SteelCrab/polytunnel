use polytunnel_core::ProjectConfig;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_vscode_generation() {
    // 1. Setup temp directory
    let dir = tempdir().unwrap();
    let root_path = dir.path();

    // 2. Create minimal config
    let config = ProjectConfig::new("test-project");

    // 3. Run generation
    let result = polytunnel_ide::vscode::generate(&config, root_path).await;
    assert!(
        result.is_ok(),
        "vscode generation failed: {:?}",
        result.err()
    );

    // 4. Verify files exist
    let project_file = root_path.join(".project");
    let classpath_file = root_path.join(".classpath");
    let settings_file = root_path.join(".vscode/settings.json");
    let gitignore_file = root_path.join(".gitignore");

    assert!(project_file.exists());
    assert!(classpath_file.exists());
    assert!(settings_file.exists());
    assert!(gitignore_file.exists());

    // 5. Verify content basics
    let project_xml = fs::read_to_string(project_file).unwrap();
    assert!(project_xml.contains("<name>test-project</name>"));

    let classpath_xml = fs::read_to_string(classpath_file).unwrap();
    assert!(classpath_xml.contains("kind=\"src\""));

    let gitignore_content = fs::read_to_string(gitignore_file).unwrap();
    assert!(gitignore_content.contains(".project"));
    assert!(gitignore_content.contains(".classpath"));
}
