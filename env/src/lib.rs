//! Bog Environment



/// A connection to the runtime environment.
// Remember, this is the client-side handle to the environment, NOT the actual environment.
pub struct Env {}

impl Env {
    /// Attempt to create a connection to the environment.
    pub fn connect() -> Result<Self> {
        Ok(Self {})
    }
}



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An error occurred when trying to create a connection to the environment.
    Connection(ConnectError),
}

#[derive(Debug)]
pub enum ConnectError {
    NotRunning,
    Other(String),
}
