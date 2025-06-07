//! Application



use bog_core::{InputEvent, vec2, WindowEvent};
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
    fn input(&mut self, cx: AppContext, input: InputEvent) {}
    fn on_close(&mut self, cx: AppContext) -> bool { true }
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
                if self.app.on_close(AppContext { window, renderer }) {
                    wm.exit();
                }
            }
            WindowEvent::RedrawRequest => {
                let mut pass = RenderPass::new();
                self.app.render(AppContext { window, renderer }, &mut pass);
                let texture = graphics.get_current_texture();
                let target = texture.texture.create_view(&gpu::TextureViewDescriptor::default());
                renderer.render(&mut pass, &target, &viewport);
                texture.present();
            }
            WindowEvent::Input(input) => match input {
                InputEvent::Resize { width, height } => {
                    let physical_size = vec2(width as f32, height as f32);
                    graphics.resize(renderer.device(), physical_size);
                    viewport.resize(physical_size);
                    renderer.resize(physical_size);
                    self.app.input(
                        AppContext { window, renderer },
                        InputEvent::Resize { width, height },
                    );
                    window.request_redraw();
                }
                other => {
                    self.app.input(AppContext { window, renderer }, other);
                }
            }
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
