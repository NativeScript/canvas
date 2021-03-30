#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnexpectedEndOfStream,
    InvalidNumber,
    UnexpectedData,
}

pub type Result<T> = std::result::Result<T, Error>;
