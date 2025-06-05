//! Application



use bog_event::{KeyCode, WheelMovement, WindowEvent};
use bog_math::{vec2, Vec2};
use bog_render::{gpu, RenderPass, Renderer, Viewport};
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
    fn startup(&mut self, cx: AppContext) {}
    fn render<'pass>(&'pass mut self, cx: AppContext, pass: &mut RenderPass<'pass>);
    fn on_resize(&mut self, cx: AppContext, size: Vec2) {}
    fn on_mouse_move(&mut self, cx: AppContext, mouse_pos: Vec2) {}
    fn on_primary_mouse_down(&mut self, cx: AppContext) {}
    fn on_primary_mouse_up(&mut self, cx: AppContext) {}
    fn on_wheel_movement(&mut self, cx: AppContext, movement: WheelMovement) {}
    fn on_key_down(&mut self, cx: AppContext, code: KeyCode, repeat: bool) {}
    fn on_key_up(&mut self, cx: AppContext, code: KeyCode) {}
    fn on_close(&mut self, cx: AppContext) {}
    fn window_desc(&self) -> WindowDescriptor;
}

pub struct AppContext<'a> {
    pub window: &'a Window,
    pub renderer: &'a mut Renderer,
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
        let (graphics, device, queue, format, backend) = pollster::block_on(async {
            WindowGraphics::from_window(window.clone()).await
        }).unwrap();
        let mut renderer = Renderer::new(device, queue, format, backend);

        self.app.startup(AppContext { window: &window, renderer: &mut renderer });

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
                self.app.on_close(AppContext { window, renderer });
                wm.exit();
            }
            WindowEvent::RedrawRequest => {
                let mut pass = RenderPass::new();
                self.app.render(AppContext { window, renderer }, &mut pass);
                let texture = graphics.get_current_texture();
                let target = texture.texture.create_view(&gpu::TextureViewDescriptor::default());
                renderer.render(&mut pass, &target, &viewport);
                texture.present();
            }
            WindowEvent::Resize { width, height } => {
                let physical_size = vec2(width as f32, height as f32);
                graphics.resize(renderer.device(), physical_size);
                viewport.resize(physical_size);
                renderer.resize(physical_size);
                self.app.on_resize(AppContext { window, renderer }, physical_size);
                window.request_redraw();
            }
            WindowEvent::KeyDown { code, repeat } => {
                self.app.on_key_down(AppContext { window, renderer }, code, repeat);
            }
            WindowEvent::KeyUp { code } => {
                self.app.on_key_up(AppContext { window, renderer }, code);
            }
            WindowEvent::MouseMove { x, y } => {
                self.app.on_mouse_move(AppContext { window, renderer }, vec2(x, y));
                window.request_redraw();
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.app.on_primary_mouse_down(AppContext { window, renderer });
                    window.request_redraw();
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    self.app.on_primary_mouse_up(AppContext { window, renderer });
                    window.request_redraw();
                }
            }
            WindowEvent::WheelMove(movement) => {
                self.app.on_wheel_movement(AppContext { window, renderer }, movement);
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
