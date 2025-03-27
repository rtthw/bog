//! Bog



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Shm(ration::Error),

    ConnectionDenied,
    WindowTitleTooLong,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ration::Error> for Error {
    fn from(value: ration::Error) -> Self {
        Self::Shm(value)
    }
}
