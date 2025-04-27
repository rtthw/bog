//! Application



use bog_event::WindowEvent;
use bog_math::glam::vec2;
use bog_render::{Renderer, Viewport};
use bog_window::{Client, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem};

use crate::{graphics::WindowGraphics, Result};



pub fn run_app(mut app: impl AppHandler) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;
    let mut proxy = Proxy {
        app: &mut app,
        state: AppState::Suspended(None),
    };

    windowing_system.run_client(&mut proxy)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
pub trait AppHandler: 'static {
    fn title(&self) -> String {
        "Untitled".to_string()
    }

    fn render(&mut self, renderer: &mut Renderer);
}

struct Proxy<'a> {
    app: &'a mut dyn AppHandler,
    state: AppState,
}

impl<'a> Client for Proxy<'a> {
    fn on_resume(&mut self, mut wm: WindowManager) {
        let AppState::Suspended(window) = &mut self.state else {
            return;
        };
        let window = window.take().unwrap_or_else(|| make_window(&mut wm, self.app));
        let (graphics, device, queue, format) = pollster::block_on(async {
            WindowGraphics::from_window(window.clone()).await
        }).unwrap();
        let renderer = Renderer::new(device, queue, format);

        self.state = AppState::Active {
            window,
            graphics,
            viewport: Viewport::default(),
            renderer,
        };
    }

    fn on_suspend(&mut self, _wm: WindowManager) {
        if let AppState::Active { window, .. } = &self.state {
            self.state = AppState::Suspended(Some(window.clone()));
        }
    }

    fn on_event(&mut self, wm: WindowManager, _id: WindowId, event: WindowEvent) {
        let AppState::Active { window, graphics, viewport, renderer } = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::CloseRequest => {
                wm.exit();
            }
            WindowEvent::RedrawRequest => {
                self.app.render(renderer);
                let texture = graphics.get_current_texture();
                let target = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                renderer.render(&target, &viewport);
                texture.present();
            }
            WindowEvent::Resize { width, height } => {
                let physical_size = vec2(width as f32, height as f32);
                graphics.resize(renderer.device(), physical_size);
                viewport.resize(physical_size);
                renderer.resize(physical_size);
                window.request_redraw();
            }
            _ => {}
        }
    }
}

enum AppState {
    Suspended(Option<Window>),
    Active {
        window: Window,
        graphics: WindowGraphics<'static>,
        viewport: Viewport,
        renderer: Renderer,
    },
}

fn make_window(wm: &mut WindowManager, app: &mut dyn AppHandler) -> Window {
    wm.create_window(WindowDescriptor {
        title: &app.title(),
        ..Default::default()
    }).unwrap()
}
