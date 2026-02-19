use super::init::do_init;
use super::sync::format_duration;
use super::tree::{parse_root_coords, render_tree};
use color_eyre::eyre::Result;
use polytunnel_maven::Coordinate;
use polytunnel_resolver::DependencyGraph;
use std::fs;
use std::time::Duration;
use tempfile::tempdir;

// === init tests ===

#[test]
fn test_init_creates_config() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    do_init("test-project", &config_path)?;

    assert!(config_path.exists());
    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("name = \"test-project\""));

    Ok(())
}

#[test]
fn test_init_ignores_existing() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");

    // Create initial config
    do_init("initial-project", &config_path)?;

    // Try to init again
    do_init("new-project", &config_path)?;

    // Verify content hasn't changed
    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("name = \"initial-project\""));
    assert!(!content.contains("name = \"new-project\""));

    Ok(())
}

// === format_duration tests ===

#[test]
fn test_format_duration_milliseconds() {
    let d = Duration::from_millis(42);
    assert_eq!(format_duration(&d), "42ms");
}

#[test]
fn test_format_duration_zero() {
    let d = Duration::from_millis(0);
    assert_eq!(format_duration(&d), "0ms");
}

#[test]
fn test_format_duration_seconds() {
    let d = Duration::from_secs(5);
    assert_eq!(format_duration(&d), "5s");
}

#[test]
fn test_format_duration_boundary() {
    // 999ms → still milliseconds
    let d = Duration::from_millis(999);
    assert_eq!(format_duration(&d), "999ms");

    // 1000ms → shows as seconds
    let d = Duration::from_millis(1000);
    assert_eq!(format_duration(&d), "1s");
}

// === parse_root_coords tests ===

#[test]
fn test_parse_root_coords_empty_config() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");
    fs::write(
        &config_path,
        "[project]\nname = \"demo\"\njava_version = \"17\"\n",
    )?;

    let config = polytunnel_core::ProjectConfig::load(&config_path)?;
    let coords = parse_root_coords(&config);
    assert!(coords.is_empty());

    Ok(())
}

#[test]
fn test_parse_root_coords_with_deps() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");
    fs::write(
        &config_path,
        r#"[project]
name = "demo"
java_version = "17"

[dependencies]
"org.slf4j:slf4j-api" = "2.0.9"
"com.google.guava:guava" = "33.0.0-jre"
"#,
    )?;

    let config = polytunnel_core::ProjectConfig::load(&config_path)?;
    let coords = parse_root_coords(&config);
    assert_eq!(coords.len(), 2);
    // Should be sorted
    assert_eq!(coords[0].group_id, "com.google.guava");
    assert_eq!(coords[1].group_id, "org.slf4j");

    Ok(())
}

#[test]
fn test_parse_root_coords_skips_invalid_key() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");
    fs::write(
        &config_path,
        r#"[project]
name = "demo"
java_version = "17"

[dependencies]
"invalid-no-colon" = "1.0"
"org.valid:artifact" = "2.0"
"#,
    )?;

    let config = polytunnel_core::ProjectConfig::load(&config_path)?;
    let coords = parse_root_coords(&config);
    // Only the valid dependency should be parsed
    assert_eq!(coords.len(), 1);
    assert_eq!(coords[0].group_id, "org.valid");

    Ok(())
}

// === render_tree tests ===

#[test]
fn test_render_tree_empty_deps() {
    let graph = DependencyGraph::new();
    let lines = render_tree("demo", &[], &graph, false);
    assert_eq!(lines, vec!["demo v0.1.0"]);
}

#[test]
fn test_render_tree_single_root_no_children() {
    let coord = Coordinate::new("com.example", "lib", "1.0.0");
    let mut graph = DependencyGraph::new();
    graph.add_node(coord.clone(), vec![], 0);

    let lines = render_tree("myproject", &[coord], &graph, false);
    assert_eq!(
        lines,
        vec!["myproject v0.1.0", "└── com.example:lib:1.0.0",]
    );
}

#[test]
fn test_render_tree_multiple_roots() {
    let coord1 = Coordinate::new("com.a", "lib-a", "1.0");
    let coord2 = Coordinate::new("com.b", "lib-b", "2.0");
    let mut graph = DependencyGraph::new();
    graph.add_node(coord1.clone(), vec![], 0);
    graph.add_node(coord2.clone(), vec![], 0);

    let lines = render_tree("proj", &[coord1, coord2], &graph, false);
    assert_eq!(
        lines,
        vec!["proj v0.1.0", "├── com.a:lib-a:1.0", "└── com.b:lib-b:2.0",]
    );
}

#[test]
fn test_render_tree_with_transitive_deps() {
    let root = Coordinate::new("com.example", "app", "1.0");
    let child = Coordinate::new("com.example", "lib", "2.0");
    let grandchild = Coordinate::new("com.example", "core", "3.0");

    let mut graph = DependencyGraph::new();
    graph.add_node(root.clone(), vec![child.clone()], 0);
    graph.add_node(child.clone(), vec![grandchild.clone()], 1);
    graph.add_node(grandchild.clone(), vec![], 2);

    let lines = render_tree("proj", &[root], &graph, false);
    assert_eq!(
        lines,
        vec![
            "proj v0.1.0",
            "└── com.example:app:1.0",
            "    └── com.example:lib:2.0",
            "        └── com.example:core:3.0",
        ]
    );
}

#[test]
fn test_render_tree_marks_duplicates() {
    let root1 = Coordinate::new("com.a", "lib-a", "1.0");
    let root2 = Coordinate::new("com.b", "lib-b", "2.0");
    let shared = Coordinate::new("com.shared", "common", "1.0");

    let mut graph = DependencyGraph::new();
    graph.add_node(root1.clone(), vec![shared.clone()], 0);
    graph.add_node(root2.clone(), vec![shared.clone()], 0);
    graph.add_node(shared.clone(), vec![], 1);

    let lines = render_tree("proj", &[root1, root2], &graph, false);
    assert_eq!(
        lines,
        vec![
            "proj v0.1.0",
            "├── com.a:lib-a:1.0",
            "│   └── com.shared:common:1.0",
            "└── com.b:lib-b:2.0",
            "    └── com.shared:common:1.0 (*)",
        ]
    );
}

#[test]
fn test_render_tree_multiple_children() {
    let root = Coordinate::new("com.example", "app", "1.0");
    let child1 = Coordinate::new("com.a", "a", "1.0");
    let child2 = Coordinate::new("com.b", "b", "1.0");

    let mut graph = DependencyGraph::new();
    graph.add_node(root.clone(), vec![child1.clone(), child2.clone()], 0);
    graph.add_node(child1.clone(), vec![], 1);
    graph.add_node(child2.clone(), vec![], 1);

    let lines = render_tree("proj", &[root], &graph, false);
    assert_eq!(
        lines,
        vec![
            "proj v0.1.0",
            "└── com.example:app:1.0",
            "    ├── com.a:a:1.0",
            "    └── com.b:b:1.0",
        ]
    );
}

#[test]
fn test_render_tree_node_not_in_graph() {
    // Root coordinate exists in root_coords but not in graph
    let coord = Coordinate::new("com.missing", "lib", "1.0");
    let graph = DependencyGraph::new();

    let lines = render_tree("proj", &[coord], &graph, false);
    assert_eq!(lines, vec!["proj v0.1.0", "└── com.missing:lib:1.0",]);
}

#[test]
fn test_render_tree_deep_nesting() {
    let a = Coordinate::new("com.a", "a", "1.0");
    let b = Coordinate::new("com.b", "b", "1.0");
    let c = Coordinate::new("com.c", "c", "1.0");

    let mut graph = DependencyGraph::new();
    graph.add_node(a.clone(), vec![b.clone()], 0);
    graph.add_node(b.clone(), vec![c.clone()], 1);
    graph.add_node(c.clone(), vec![], 2);

    let lines = render_tree("proj", &[a], &graph, false);
    assert_eq!(lines.len(), 4);
    assert!(lines[3].starts_with("        └── "));
}

#[test]
fn test_render_tree_verbose_flag_accepted() {
    // verbose=true should not change output format (currently unused but accepted)
    let graph = DependencyGraph::new();
    let lines_normal = render_tree("proj", &[], &graph, false);
    let lines_verbose = render_tree("proj", &[], &graph, true);
    assert_eq!(lines_normal, lines_verbose);
}

// === do_tree async tests ===

#[tokio::test]
async fn test_do_tree_zero_deps() -> Result<()> {
    let dir = tempdir()?;
    let config_path = dir.path().join("polytunnel.toml");
    fs::write(
        &config_path,
        "[project]\nname = \"test-proj\"\njava_version = \"17\"\n",
    )?;

    let result = super::tree::do_tree(&config_path, false).await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_do_tree_missing_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("nonexistent.toml");

    let result = super::tree::do_tree(&config_path, false).await;
    assert!(result.is_err());
}

// === do_sync async tests ===

#[tokio::test]
async fn test_do_sync_zero_deps() -> Result<()> {
    let dir = tempdir()?;
    let cache_dir = dir.path().join(".polytunnel/cache");
    let config_path = dir.path().join("polytunnel.toml");

    fs::write(
        &config_path,
        format!(
            r#"[project]
name = "test-proj"
java_version = "17"

[build]
source_dirs = ["{src}"]
test_source_dirs = ["{test_src}"]
output_dir = "{out}"
test_output_dir = "{test_out}"
cache_dir = "{cache}"
"#,
            src = dir.path().join("src/main/java").display(),
            test_src = dir.path().join("src/test/java").display(),
            out = dir.path().join("target/classes").display(),
            test_out = dir.path().join("target/test-classes").display(),
            cache = cache_dir.display(),
        ),
    )?;

    fs::create_dir_all(dir.path().join("src/main/java"))?;
    fs::create_dir_all(dir.path().join("src/test/java"))?;

    let result = super::sync::do_sync(&config_path, false).await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_do_sync_missing_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("nonexistent.toml");

    let result = super::sync::do_sync(&config_path, false).await;
    assert!(result.is_err());
}
