//! Application



use bog_event::{KeyCode, WheelMovement, WindowEvent};
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
    fn render(&mut self, renderer: &mut Renderer, layers: &mut LayerStack);
    fn on_mouse_move(&mut self, mouse_pos: Vec2) {}
    fn on_primary_mouse_down(&mut self) {}
    fn on_primary_mouse_up(&mut self) {}
    fn on_wheel_movement(&mut self, movement: WheelMovement) {}
    fn on_key_down(&mut self, code: KeyCode, repeat: bool) {}
    fn on_key_up(&mut self, code: KeyCode) {}
    fn on_close(&mut self) {}
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
                self.app.on_close();
                wm.exit();
            }
            WindowEvent::RedrawRequest => {
                let mut layer_stack = LayerStack::new();
                self.app.render(renderer, &mut layer_stack);
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
                self.app.on_key_down(code, repeat);
            }
            WindowEvent::KeyUp { code } => {
                self.app.on_key_up(code);
            }
            WindowEvent::MouseMove { x, y } => {
                self.app.on_mouse_move(vec2(x, y));
                window.request_redraw();
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.app.on_primary_mouse_down();
                    window.request_redraw();
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    self.app.on_primary_mouse_up();
                    window.request_redraw();
                }
            }
            WindowEvent::WheelMove(movement) => {
                self.app.on_wheel_movement(movement);
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
