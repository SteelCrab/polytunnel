use super::utils::print_status;
use color_eyre::eyre::{Result, bail};
use colored::Color;
use polytunnel_core::{parse_remove_coordinate, remove_dependency_from_file};
use std::path::Path;

pub fn cmd_remove(dependency: &str) -> Result<()> {
    do_remove(dependency, Path::new("polytunnel.toml"))
}

pub(crate) fn do_remove(dependency: &str, config_path: &Path) -> Result<()> {
    if !config_path.exists() {
        bail!("polytunnel.toml not found. Run `pt init` first.");
    }

    let ga_key = parse_remove_coordinate(dependency)?;

    remove_dependency_from_file(config_path, &ga_key)?;

    print_status("Removed", &ga_key, Color::Red);

    Ok(())
}
