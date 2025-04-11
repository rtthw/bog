//! Bog



pub mod window {
    pub use winit::*;
}

pub mod layout;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
}
