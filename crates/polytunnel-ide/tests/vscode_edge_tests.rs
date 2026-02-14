use polytunnel_core::ProjectConfig;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_vscode_generation_is_idempotent() {
    let root = tempdir().unwrap();
    let config = ProjectConfig::new("vscode-idempotent");

    polytunnel_ide::vscode::generate(&config, root.path())
        .await
        .expect("first generation should succeed");
    polytunnel_ide::vscode::generate(&config, root.path())
        .await
        .expect("second generation should succeed");

    let gitignore_content = fs::read_to_string(root.path().join(".gitignore")).unwrap();
    let project_count = gitignore_content
        .lines()
        .filter(|line| line.trim() == ".project")
        .count();
    let classpath_count = gitignore_content
        .lines()
        .filter(|line| line.trim() == ".classpath")
        .count();

    assert_eq!(project_count, 1);
    assert_eq!(classpath_count, 1);
}
