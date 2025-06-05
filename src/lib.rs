//! Bog



pub mod app;
pub mod graphics;

pub use bog_event as event;
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



pub mod prelude {
    pub use crate::{
        app::{
            AppContext,
            AppHandler,
            run_app,
        },
        Error,
        graphics::{
            GraphicsError,
            WindowGraphics,
        },
        Result,
    };

    pub use bog_event::{
        EventMask,
        KeyCode,
        KeyUpdate,
        WheelMovement,
        WindowEvent,
    };
    pub use bog_core::{
        Color,
        Mat3,
        Mat4,
        NoHashMap,
        Rect,
        vec2,
        Vec2,
        vec3,
        Vec3,
    };
    pub use bog_render::{
        Border,
        FontFamily,
        Image,
        ImageFilterMethod,
        ImageHandle,
        Layer,
        Quad,
        RasterImage,
        Renderer,
        RenderPass,
        Shadow,
        Text,
        Viewport,
    };
    pub use bog_window::{
        CursorIcon,
        Window,
        WindowDescriptor,
        WindowError,
        WindowId,
        WindowingClient,
        WindowingSystem,
        WindowManager,
        WindowManagerError,
        WindowManagerEvent,
    };
}
