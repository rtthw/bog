//! Bog



pub mod graphics;
pub mod layout;
pub mod window {
    pub use winit::{
        dpi,
        error::{EventLoopError as WindowManagerError, OsError as WindowError},
        event::{Event as WindowManagerEvent, WindowEvent},
        event_loop::EventLoop,
        window::{Window, WindowBuilder},
    };
}



pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("graphics error")]
    GraphicsError(#[from] graphics::GraphicsError),
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
    #[error("window error")]
    WindowError(#[from] window::WindowError),
    #[error("window manager error")]
    WindowManagerError(#[from] window::WindowManagerError),
}
