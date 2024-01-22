use thiserror::Error;

#[derive(Error, Debug)]
pub enum RimeError {
    #[error("Missing Data Directory")]
    MissingDataDir,
}
