//! Bog



pub mod fonts;
pub mod graphics;
pub mod gui;

pub use bog_alloc as alloc;
pub use bog_collections as collections;
pub use bog_color as color;
pub use bog_env as env;
pub use bog_event as event;
pub use bog_layout as layout;
pub use bog_math as math;
pub use bog_render as render;
pub use bog_window as window;



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
