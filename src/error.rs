use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write to PPM file: {0}")]
    FileWriteError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
