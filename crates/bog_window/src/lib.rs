//! Bog Window



use bog_math::{vec2, Vec2};
pub use winit::{
    dpi,
    error::{EventLoopError as WindowManagerError, OsError as WindowError},
    event::{ElementState, Event as WindowManagerEvent, MouseButton, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{CursorIcon, Window},
};



pub struct WindowManager {
    event_loop: EventLoop<()>,
}

impl WindowManager {
    pub fn new() -> Result<Self, WindowManagerError> {
        Ok(Self {
            event_loop: EventLoop::new()?,
        })
    }

    pub fn create_window(&self, desc: WindowDescriptor) -> Result<Window, WindowError> {
        winit::window::WindowBuilder::new()
            .with_title(desc.title)
            .with_inner_size(dpi::LogicalSize::new(desc.inner_size.x, desc.inner_size.y))
            .with_active(desc.active)
            .with_maximized(desc.maximized)
            .with_visible(desc.visible)
            .with_transparent(desc.transparent)
            .with_blur(desc.blurred)
            .with_decorations(desc.decorated)
            .build(&self.event_loop)
    }

    pub fn run<F>(self, func: F) -> Result<(), WindowManagerError>
    where
        F: FnMut(WindowManagerEvent<()>, &EventLoopWindowTarget<()>),
    {
        self.event_loop.run(func)
    }
}



pub struct WindowDescriptor<'a> {
    pub title: &'a str,
    pub inner_size: Vec2,
    pub active: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blurred: bool,
    pub decorated: bool,
}

impl<'a> Default for WindowDescriptor<'a> {
    fn default() -> Self {
        Self {
            title: "Untitled Window",
            inner_size: vec2(1280.0, 720.0),
            active: true,
            maximized: false,
            visible: true,
            transparent: false,
            blurred: false,
            decorated: true,
        }
    }
}
