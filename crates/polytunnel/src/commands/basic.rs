use super::utils::print_status;
use color_eyre::eyre::Result;
use colored::Color;

pub fn cmd_remove(dependency: &str) -> Result<()> {
    print_status("Removing", dependency, Color::Red);
    // TODO: Implement in Phase 3
    Ok(())
}
