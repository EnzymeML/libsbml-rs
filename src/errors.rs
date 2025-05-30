#[derive(Debug, thiserror::Error)]
pub enum LibSBMLError {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}
