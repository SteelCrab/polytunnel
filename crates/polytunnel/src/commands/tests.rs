use super::*;
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

#[test]
fn test_cmd_init_public_api() -> Result<()> {
    // We need to lock this test because it changes the global CWD,
    // which could affect other tests if run in parallel.
    // Since we can't easily share a lock across test threads without lazy_static/std::sync::OnceLock
    // and we don't want to enforce serial execution on everything,
    // we assume this test is running in an environment where temporary CWD switches are tolerable
    // or we implement a simple local lock if possible.
    // Ideally we would run this test serially via `cargo test -- --test-threads=1` but we can't force that here.
    // Instead, we rely on the fact that other tests use absolute paths from `tempdir()`.

    static CWD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _lock = CWD_LOCK.lock().unwrap_or_else(|p| p.into_inner());

    let dir = tempdir()?;
    let original_dir = std::env::current_dir()?;

    // Change CWD
    std::env::set_current_dir(&dir)?;

    // Run the command
    let result = std::panic::catch_unwind(|| cmd_init("test-public-api"));

    // Restore CWD regardless of result
    std::env::set_current_dir(&original_dir)?;

    // Check result
    result.map_err(|e| color_eyre::eyre::eyre!("Test panicked: {:?}", e))??;

    // Verify
    assert!(dir.path().join("polytunnel.toml").exists());
    Ok(())
}
