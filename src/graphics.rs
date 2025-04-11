//! Graphics module



use crate::window::Window;



type Result<T> = std::result::Result<T, GraphicsError>;

#[derive(thiserror::Error, Debug)]
pub enum GraphicsError {
    #[error("create surface error")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("request device error")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}



pub struct WindowGraphics<'w> {
    surface: wgpu::Surface<'w>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: [u32; 2],

    // NOTE: Window must be dropped after the other surface fields.
    window: &'w Window,
}

// Constructors.
impl<'w> WindowGraphics<'w> {
    pub async fn from_window(window: &'w Window) -> Result<Self> {
        let size: [u32; 2] = window.inner_size().into();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap(); // TODO: Remove unwrap.

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size[0],
            height: size[1],
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        })
    }
}

// Getters.
impl<'w> WindowGraphics<'w> {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn window(&self) -> &Window {
        self.window
    }
}

impl<'w> WindowGraphics<'w> {
    pub fn render(&self) -> Result<()> {
        Ok(())
    }
}



pub struct ShaderDescriptor<'a> {
    pub label: Option<&'a str>,
}
