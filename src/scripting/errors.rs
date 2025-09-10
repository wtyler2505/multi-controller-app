use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Script execution error: {0}")]
    Execution(String),
    
    #[error("Script compilation error: {0}")]
    Compilation(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),
    
    #[error("Security violation: {0}")]
    Security(String),
    
    #[error("Device operation error: {0}")]
    DeviceOperation(String),
    
    #[error("Script timeout after {0:?}")]
    Timeout(std::time::Duration),
    
    #[error("Invalid script: {0}")]
    Invalid(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Rhai error: {0}")]
    Rhai(#[from] Box<rhai::EvalAltResult>),
}

pub type ScriptResult<T> = Result<T, ScriptError>;