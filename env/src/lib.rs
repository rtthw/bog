//! Bog Environment



/// A connection to the runtime environment.
pub struct Connection {
    id: u32,
}

impl Connection {
    /// The universal identifier for this connection.
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Attempt to create a connection to the environment.
pub fn connect() -> Result<Connection> {
    let id = std::process::id();

    Ok(Connection { id })
}



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
