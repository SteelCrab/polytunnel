use color_eyre::eyre::Result;
use polytunnel_build::BuildError;
use polytunnel_core::ProjectConfig;
use std::path::Path;

pub async fn cmd_vscode() -> Result<()> {
    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    // Delegate to IDE crate
    polytunnel_ide::vscode::generate(&config, Path::new("."))
        .await
        .map_err(|e| match e {
            polytunnel_ide::IdeError::Build(e) => e,
            polytunnel_ide::IdeError::Core(e) => BuildError::Core(e),
            polytunnel_ide::IdeError::Io(e) => BuildError::Io(e),
        })?;

    Ok(())
}
