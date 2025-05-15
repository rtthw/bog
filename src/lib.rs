//! Bog



pub mod app;
pub mod graphics;

pub use bog_alloc as alloc;
pub use bog_collections as collections;
pub use bog_color as color;
pub use bog_env as env;
pub use bog_event as event;
pub use bog_layout as layout;
pub use bog_math as math;
pub use bog_render as render;
pub use bog_view as view;
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
            AppHandler,
            run_app,
        },
        Error,
        graphics::{
            GraphicsError,
            wgpu,
            WindowGraphics,
        },
        Result,
    };

    pub use bog_color::Color;
    pub use bog_layout::{
        Layout,
        LayoutMap,
        Placement,
    };
    pub use bog_math::{
        Mat3,
        Mat4,
        Rect,
        vec2,
        Vec2,
        vec3,
        Vec3,
    };
    pub use bog_render::{
        Border,
        FontFamily,
        FontStyle,
        FontWeight,
        FontWidth,
        Quad,
        Render as _,
        Renderer,
        Shadow,
        Text,
        Viewport,
    };
    pub use bog_view::{
        Element,
        EventContext,
        Model,
        ModelProxy,
        Object,
        render_view,
        RenderContext,
        View,
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
