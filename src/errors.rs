use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebAppError {
    #[error("File does not exist: {0}")]
    FileNotExist(String),
    #[error("Not a valid content config file: {0}")]
    NotValidConfig(String),
}
