use super::utils::print_status;
use color_eyre::eyre::Result;
use colored::Color;
use polytunnel_core::ProjectConfig;
use std::path::Path;

pub fn cmd_init(name: &str) -> Result<()> {
    do_init(name, Path::new("polytunnel.toml"))
}

pub(crate) fn do_init(name: &str, config_path: &Path) -> Result<()> {
    if config_path.exists() {
        print_status("Ignored", "polytunnel.toml already exists", Color::Yellow);
        return Ok(());
    }

    let config = ProjectConfig::new(name);
    config.save(config_path)?;
    print_status(
        "Created",
        &format!("polytunnel.toml for project: {}", name),
        Color::Green,
    );
    Ok(())
}
