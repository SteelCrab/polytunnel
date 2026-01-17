use thiserror::Error;

pub type Result<T> = std::result::Result<T, IdeError>;

#[derive(Error, Debug)]
pub enum IdeError {
    #[error("Build error: {0}")]
    Build(#[from] polytunnel_build::BuildError),

    #[error("Core error: {0}")]
    Core(#[from] polytunnel_core::CoreError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
