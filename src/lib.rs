//! Bog



pub extern crate winit;

pub mod graphics;
pub mod layout;
pub mod program;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("graphics error")]
    GraphicsError(#[from] graphics::Error),
}
