//! Application



use bog_event::WindowEvent;
use bog_math::{vec2, Vec2};
use bog_render::{LayerStack, Renderer, Viewport};
use bog_window::{
    WindowingClient, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem,
};

use crate::{
    graphics::WindowGraphics,
    Result,
};



pub fn run_app<A: AppHandler>(app: A) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;

    let mut runner = AppRunner {
        app,
        state: AppState::Suspended(None),
    };

    windowing_system.run_client(&mut runner)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait AppHandler {
    fn window_desc(&self) -> WindowDescriptor;
}

struct AppRunner<A: AppHandler> {
    app: A,
    state: AppState,
}

impl<A: AppHandler> WindowingClient for AppRunner<A> {
    fn on_startup(&mut self, _wm: WindowManager) {}

    fn on_resume(&mut self, wm: WindowManager) {
        let AppState::Suspended(window) = &mut self.state else {
            return;
        };
        let window = window.take().unwrap_or_else(|| {
            wm.create_window(self.app.window_desc()).unwrap()
        });
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
                let mut layer_stack = LayerStack::new();
                let texture = graphics.get_current_texture();
                let target = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                renderer.render(&mut layer_stack, &target, &viewport);
                texture.present();
            }
            WindowEvent::Resize { width, height } => {
                let physical_size = vec2(width as f32, height as f32);
                graphics.resize(renderer.device(), physical_size);
                viewport.resize(physical_size);
                renderer.resize(physical_size);
                window.request_redraw();
            }
            WindowEvent::KeyDown { code, repeat } => {
            }
            WindowEvent::KeyUp { code } => {
            }
            WindowEvent::MouseMove { x, y } => {
                window.request_redraw();
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    window.request_redraw();
                }
            }
            WindowEvent::WheelMove(movement) => {
                window.request_redraw();
            }
            _ => {}
        }
    }
}

enum AppState {
    Suspended(Option<Window>),
    Active {
        graphics: WindowGraphics<'static>,
        window: Window,
        viewport: Viewport,
        renderer: Renderer,
    },
}
