use color_eyre::eyre::Result;
use polytunnel_build::BuildError;
use polytunnel_core::ProjectConfig;
use std::path::Path;

fn map_ide_error(error: polytunnel_ide::IdeError) -> BuildError {
    match error {
        polytunnel_ide::IdeError::Build(e) => e,
        polytunnel_ide::IdeError::Core(e) => BuildError::Core(e),
        polytunnel_ide::IdeError::Io(e) => BuildError::Io(e),
    }
}

pub async fn cmd_vscode() -> Result<()> {
    // Load configuration
    let config = ProjectConfig::load(Path::new("polytunnel.toml"))?;

    // Delegate to IDE crate
    polytunnel_ide::vscode::generate(&config, Path::new("."))
        .await
        .map_err(map_ide_error)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::map_ide_error;
    use polytunnel_build::BuildError;
    use std::io::ErrorKind;

    #[test]
    fn test_map_ide_error_core_variant() {
        let core = polytunnel_core::CoreError::ConfigNotFound {
            path: "polytunnel.toml".to_string(),
        };
        let mapped = map_ide_error(polytunnel_ide::IdeError::Core(core));
        assert!(matches!(mapped, BuildError::Core(_)));
    }

    #[test]
    fn test_map_ide_error_io_variant() {
        let io = std::io::Error::new(ErrorKind::PermissionDenied, "denied");
        let mapped = map_ide_error(polytunnel_ide::IdeError::Io(io));
        assert!(matches!(mapped, BuildError::Io(_)));
    }

    #[test]
    fn test_map_ide_error_build_variant() {
        let build = BuildError::Io(std::io::Error::new(ErrorKind::BrokenPipe, "broken"));
        let mapped = map_ide_error(polytunnel_ide::IdeError::Build(build));
        assert!(matches!(mapped, BuildError::Io(_)));
    }
}
