//! Bog



#[cfg(feature = "app")]
pub mod app;
pub mod graphics;

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
    #[cfg(feature = "app")]
    pub use crate::app::{
        AppContext,
        SimpleApp,
        run_simple_app,
    };
    pub use crate::{
        Error,
        graphics::{
            GraphicsDescriptor,
            GraphicsError,
            WindowGraphics,
        },
        Result,
    };

    pub use bog_core::{
        Color,
        EventParser,
        Input,
        InputArea,
        InputEvent,
        KeyCode,
        KeyEventParser,
        KeyInput,
        KeyUpdate,
        Mat3,
        Mat4,
        MouseButton,
        MouseButtonMask,
        MouseEventParser,
        MouseInput,
        NoHashMap,
        Rect,
        vec2,
        Vec2,
        vec3,
        Vec3,
        WheelMovement,
        WindowEvent,
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
        App,
        AppEvent,
        CursorIcon,
        Window,
        WindowDescriptor,
        WindowError,
        WindowingSystem,
        WindowManager,
        WindowManagerError,
        WindowManagerEvent,
    };
}
