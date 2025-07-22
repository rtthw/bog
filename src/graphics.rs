//! Graphics module



use bog_core::Vec2;

use crate::{render::gpu, window::rwh};



type Result<T> = core::result::Result<T, GraphicsError>;

#[derive(thiserror::Error, Debug)]
pub enum GraphicsError {
    #[error("create surface error")]
    CreateSurfaceError(#[from] gpu::CreateSurfaceError),
    #[error("request device error")]
    RequestDeviceError(#[from] gpu::RequestDeviceError),
}



pub struct GraphicsDescriptor {
    /// Override the GPU backend selection.
    pub backend_override: Option<gpu::Backends>,
    pub power_preference: gpu::PowerPreference,
    /// Use the system's fallback (generally, software-level) rendering adapter.
    pub force_fallback_adapter: bool,
}

impl Default for GraphicsDescriptor {
    fn default() -> Self {
        Self {
            backend_override: None,
            power_preference: gpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }
    }
}



// NOTE: Window must be dropped after the other surface fields.
pub struct WindowGraphics<'w> {
    surface: gpu::Surface<'w>,
    config: gpu::SurfaceConfiguration,
}

// Constructors.
impl<'w> WindowGraphics<'w> {
    pub async fn from_window<W>(window: W, desc: GraphicsDescriptor) -> Result<(
        Self,
        gpu::Device,
        gpu::Queue,
        gpu::TextureFormat,
        gpu::Backend,
    )>
    where W: rwh::HasWindowHandle + rwh::HasDisplayHandle + Send + Sync + 'w,
    {
        let backends = desc.backend_override.unwrap_or({
            #[cfg(not(target_arch = "wasm32"))]
            {
                // HACK: It's safer to default to GL on Linux because it is highly likely for users
                //       to not have properly configured Vulkan (especially when using an AMD GPU).
                //       And, for whatever reason (probably something unintentional), WGPU would
                //       rather use the integrated CPU graphics than try using GL. That would be
                //       VERY BAD.
                #[cfg(target_os = "linux")]
                {
                    gpu::Backends::GL
                }
                #[cfg(not(target_os = "linux"))]
                gpu::Backends::PRIMARY
            }
            #[cfg(target_arch = "wasm32")]
            gpu::Backends::GL
        });
        let instance = gpu::Instance::new(&gpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&gpu::RequestAdapterOptions {
                power_preference: desc.power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: desc.force_fallback_adapter,
            })
            .await
            .unwrap(); // TODO: Remove unwrap.

        let backend = adapter.get_info().backend;

        let (device, queue) = adapter
            .request_device(
                &gpu::DeviceDescriptor {
                    label: None,
                    required_features: gpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        gpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        gpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                    trace: gpu::Trace::Off,
                },
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = gpu::SurfaceConfiguration {
            usage: gpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: 0,
            height: 0,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        Ok((
            Self {
                surface,
                config,
            },
            device,
            queue,
            surface_format,
            backend,
        ))
    }
}

// Getters.
impl<'w> WindowGraphics<'w> {
    pub fn surface(&self) -> &gpu::Surface<'_> {
        &self.surface
    }

    pub fn surface_config(&self) -> &gpu::SurfaceConfiguration {
        &self.config
    }

    pub fn surface_config_mut(&mut self) -> &mut gpu::SurfaceConfiguration {
        &mut self.config
    }

    pub fn screen_size(&self) -> Vec2 {
        Vec2::new(self.surface_config().width as f32, self.surface_config().height as f32)
    }
}

impl<'w> WindowGraphics<'w> {
    pub fn get_current_texture(&self) -> gpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn resize(&mut self, device: &gpu::Device, new_size: Vec2) {
        self.config.width = new_size.x as _;
        self.config.height = new_size.y as _;
        self.surface.configure(device, &self.config);
    }
}
