//! Graphics



// pub extern crate three_d;
pub extern crate winit;

pub mod animation;
pub mod fonts;
pub mod layout;
pub mod mesh;
pub mod scene;

pub use three_d::{
    Camera,
    ClearState,
    ColorMaterial,
    RenderTarget,
    Srgba,
    Viewport,
};

use glutin::{
    prelude::{
        GlDisplay as _,
        NotCurrentGlContextSurfaceAccessor as _,
        PossiblyCurrentContextGlSurfaceAccessor as _,
    },
    surface::*,
};
use mesh::Mesh;



#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("glutin error")]
    GlutinError(#[from] glutin::error::Error),
    #[error("winit error")]
    WinitError(#[from] winit::error::OsError),
    #[error("three-d error")]
    ThreeDError(#[from] three_d::CoreError),
    #[error("it's not possible to create a graphics context/surface with the given settings")]
    SurfaceCreationError,
    #[error("font error")]
    FontError(#[from] fonts::Error),
}



#[derive(Clone)]
pub struct Renderer {
    pub(crate) context: three_d::Context,
    fonts: fonts::Fonts,
}

impl std::ops::Deref for Renderer {
    type Target = three_d::Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

// Text rendering.
impl Renderer {
    pub fn load_font(&mut self, name: &str, bytes: Vec<u8>, size: f32) -> Result<(), Error> {
        Ok(self.fonts.load_font(name, bytes, size)?)
    }

    pub fn get_font(&self, name: &str) -> Option<&fonts::Font> {
        self.fonts.get_font(name)
    }

    pub fn mesh_for_text(&self, font: &str, text: &str, line_height: Option<f32>) -> Option<Mesh> {
        let font = self.fonts.get_font(font)?;
        let cpu_mesh = font.cpu_mesh_for_text(text, line_height);

        Some(Mesh::new(&self, &cpu_mesh))
    }
}



pub trait Render {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object>;
}

pub trait RenderOne {
    fn object(&self) -> impl three_d::Object;
    fn destructure(self) -> (Mesh, ColorMaterial);
}

impl<T: RenderOne> Render for T {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object> {
        std::iter::once(self.object())
    }
}



/// Configuration variables used to initialize a [`WindowGraphics`] instance.
pub struct GraphicsConfig {
    /// Whether vertical syncing should be used, which would limit the FPS to the display's
    /// refresh rate. This can be useful for games.
    ///
    /// A good default is `true`.
    pub use_vsync: bool,
    /// Whether hardware acceleration should be used. A value of `None` means "preferred".
    /// `Some(true)` means "required". `Some(false)` means "off".
    ///
    /// A good default is `None`.
    pub hardware_acceleration: Option<bool>,
    /// The number of bits in the depth buffer. A value of `0` means no depth buffer.
    ///
    /// A good default is `24`.
    pub depth_buffer_size: u8,
    /// The number of bits in the stencil buffer. A value of `0` means no stencil buffer.
    ///
    /// A good default is `0`.
    pub stencil_buffer_size: u8,
    /// The level of the multisample anti-aliasing (MSAA). Must be a power-of-two. A value of '0'
    /// means "off".
    ///
    /// A good default is `4`.
    pub multisamples: u8,
    /// The initial width of the surface.
    pub width: u32,
    /// The initial height of the surface.
    pub height: u32,
}

impl GraphicsConfig {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            use_vsync: true,
            hardware_acceleration: None,
            depth_buffer_size: 24,
            stencil_buffer_size: 0,
            multisamples: 4,
            width,
            height,
        }
    }
}



pub struct WindowGraphics {
    renderer: Renderer,
    surface: Surface<WindowSurface>,
    glutin_context: glutin::context::PossiblyCurrentContext,
}

impl WindowGraphics {
    /// This will overwrite `GraphicsConfig::width` and `GraphicsConfig::height` with the window's
    /// inner size. Use `Self::from_raw` to avoid this behavior.
    pub fn from_winit_window(
        window: &winit::window::Window,
        config: GraphicsConfig,
    ) -> Result<Self, Error> {
        use raw_window_handle::{HasRawDisplayHandle as _, HasRawWindowHandle as _};

        let window_handle = window.raw_window_handle();
        let display_handle = window.raw_display_handle();
        let (width, height) = window.inner_size().into();

        Self::from_raw(
            window_handle,
            display_handle,
            GraphicsConfig {
                width,
                height,
                ..config
            },
        )
    }

    pub fn from_raw(
        window_handle: raw_window_handle::RawWindowHandle,
        display_handle: raw_window_handle::RawDisplayHandle,
        config: GraphicsConfig,
    ) -> Result<Self, Error> {
        // Try EGL and fallback to WGL.
        #[cfg(target_os = "windows")]
        let preference =
            glutin::display::DisplayApiPreference::WglThenEgl(Some(window_handle));
        // Try egl and fallback to X11 GLX.
        #[cfg(target_os = "linux")]
        let preference = glutin::display::DisplayApiPreference::EglThenGlx(Box::new(
            winit::platform::x11::register_xlib_error_hook,
        ));
        #[cfg(target_os = "macos")]
        let preference = glutin::display::DisplayApiPreference::Cgl;
        #[cfg(target_os = "android")]
        let preference = glutin::display::DisplayApiPreference::Egl;

        let gl_display = unsafe {
            glutin::display::Display::new(display_handle, preference)?
        };
        let swap_interval = if config.use_vsync {
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())
        } else {
            glutin::surface::SwapInterval::DontWait
        };

        let config_template = glutin::config::ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(config.hardware_acceleration)
            .with_depth_size(config.depth_buffer_size);
        let config_template = if config.multisamples > 0 {
            config_template.with_multisampling(config.multisamples)
        } else {
            config_template
        };
        let config_template = config_template
            .with_stencil_size(config.stencil_buffer_size)
            .compatible_with_native_window(window_handle)
            .build();

        // Find all valid configs supported by this display that match the `config_template`.
        // This is where we will try to get a "fallback" config if we're okay with ignoring some
        // native options required by the user like multisampling and such.
        let gl_config = unsafe {
            gl_display
                .find_configs(config_template)?
                .next()
                .ok_or(Error::SurfaceCreationError)?
        };

        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(Some(window_handle));
        let width = std::num::NonZeroU32::new(config.width.max(1)).unwrap();
        let height = std::num::NonZeroU32::new(config.height.max(1)).unwrap();
        let surface_attributes =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                .build(window_handle, width, height);

        let gl_context = unsafe {
            gl_display.create_context(&gl_config, &context_attributes)?
        };
        let gl_surface = unsafe {
            gl_display.create_window_surface(&gl_config, &surface_attributes)?
        };
        let gl_context = gl_context.make_current(&gl_surface)?;
        gl_surface.set_swap_interval(&gl_context, swap_interval)?;

        Ok(Self {
            renderer: Renderer {
                context: three_d::Context::from_gl_context(std::sync::Arc::new(unsafe {
                    three_d::context::Context::from_loader_function(|s| {
                        let s = std::ffi::CString::new(s)
                            .expect("failed to construct C string from string for gl proc address");

                        gl_display.get_proc_address(&s)
                    })
                }))?,
                fonts: fonts::Fonts::default(),
            },
            glutin_context: gl_context,
            surface: gl_surface,
        })
    }
}

impl WindowGraphics {
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
        let width = std::num::NonZeroU32::new(physical_size.width.max(1)).unwrap();
        let height = std::num::NonZeroU32::new(physical_size.height.max(1)).unwrap();
        self.surface.resize(&self.glutin_context, width, height);
    }

    pub fn make_current(&self) -> Result<(), Error> {
        Ok(self.glutin_context.make_current(&self.surface)?)
    }

    pub fn swap_buffers(&self) -> Result<(), Error> {
        Ok(self.surface.swap_buffers(&self.glutin_context)?)
    }

    pub fn set_vsync(&self, use_vsync: bool) -> Result<(), Error> {
        let swap_interval = if use_vsync {
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())
        } else {
            glutin::surface::SwapInterval::DontWait
        };

        Ok(self.surface.set_swap_interval(&self.glutin_context, swap_interval)?)
    }
}
