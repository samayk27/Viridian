use thiserror::Error;

#[derive(Debug, Error)]
pub enum VdError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("LLM error: {0}")]
    Llm(String),
    #[error("Tool error: {0}")]
    Tool(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
