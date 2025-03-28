//! Bog



pub extern crate three_d;

pub mod window;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("window error")]
    WindowError(#[from] window::WindowError),
}
