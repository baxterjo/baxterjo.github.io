use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebAppError {
    #[error("Not a valid content config file: {0}")]
    NotValidConfig(String),
}
