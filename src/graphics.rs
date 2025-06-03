//! Graphics module



use crate::{math::Vec2, window::rwh};

pub use bytemuck;
pub use wgpu;



pub const SAMPLE_COUNT: u32 = 4;



type Result<T> = core::result::Result<T, GraphicsError>;

#[derive(thiserror::Error, Debug)]
pub enum GraphicsError {
    #[error("create surface error")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),
    #[error("request device error")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}



// NOTE: Window must be dropped after the other surface fields.
pub struct WindowGraphics<'w> {
    surface: wgpu::Surface<'w>,
    config: wgpu::SurfaceConfiguration,
}

// Constructors.
impl<'w> WindowGraphics<'w> {
    pub async fn from_window<W>(
        window: W,
    ) -> Result<(Self, wgpu::Device, wgpu::Queue, wgpu::TextureFormat)>
    where W: rwh::HasWindowHandle + rwh::HasDisplayHandle + Send + Sync + 'w,
    {
        let backends = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                #[cfg(target_os = "linux")]
                {
                    wgpu::Backends::GL
                }
                #[cfg(not(target_os = "linux"))]
                wgpu::Backends::PRIMARY
            }
            #[cfg(target_arch = "wasm32")]
            wgpu::Backends::GL
        };
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
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
                    trace: wgpu::Trace::Off,
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
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
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
        ))
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

    pub fn surface_config_mut(&mut self) -> &mut wgpu::SurfaceConfiguration {
        &mut self.config
    }

    pub fn screen_size(&self) -> Vec2 {
        Vec2::new(self.surface_config().width as f32, self.surface_config().height as f32)
    }
}

impl<'w> WindowGraphics<'w> {
    pub fn get_current_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: Vec2) {
        self.config.width = new_size.x as _;
        self.config.height = new_size.y as _;
        self.surface.configure(device, &self.config);
    }
}
