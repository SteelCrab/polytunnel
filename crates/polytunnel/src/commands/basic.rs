use super::utils::print_status;
use color_eyre::eyre::Result;
use colored::Color;

pub async fn cmd_add(dependency: &str) -> Result<()> {
    print_status("Adding", dependency, Color::Green);
    // TODO: Implement in Phase 3
    Ok(())
}

pub fn cmd_remove(dependency: &str) -> Result<()> {
    print_status("Removing", dependency, Color::Red);
    // TODO: Implement in Phase 3
    Ok(())
}

pub async fn cmd_sync() -> Result<()> {
    print_status("Syncing", "dependencies...", Color::Cyan);
    // TODO: Implement in Phase 3
    Ok(())
}

pub async fn cmd_tree() -> Result<()> {
    println!("Dependency tree:");
    // TODO: Implement in Phase 3
    Ok(())
}
