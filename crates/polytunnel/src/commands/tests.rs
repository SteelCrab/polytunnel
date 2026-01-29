use super::*;
use color_eyre::eyre::Result;
use std::fs;
use tempfile::tempdir;

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
