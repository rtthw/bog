//! Application



use bog_core::{InputEvent, vec2, WindowEvent};
use bog_render::{gpu, RenderPass, Renderer, Viewport};
use bog_window::{
    App, AppEvent, Window, WindowDescriptor, WindowManager, WindowingSystem
};

use crate::{
    graphics::WindowGraphics,
    Result,
};



pub fn run_simple_app<A, E>(existing_system: Option<WindowingSystem<E>>, app: A) -> Result<()>
where
    A: SimpleApp<CustomEvent = E>,
    E: 'static,
{
    let windowing_system = {
        // NOTE: Can't use `unwrap_or_else` here because we need the error.
        if let Some(windowing_system) = existing_system {
            windowing_system
        } else {
            WindowingSystem::new()?
        }
    };

    let mut runner = AppRunner {
        app,
        state: AppState::Suspended(None),
    };

    windowing_system.run_app(&mut runner)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait SimpleApp {
    type CustomEvent: 'static;

    fn startup(&mut self, cx: AppContext) {}
    fn render<'pass>(&'pass mut self, cx: AppContext, pass: &mut RenderPass<'pass>);
    fn input(&mut self, cx: AppContext, input: InputEvent) {}
    fn event(&mut self, cx: AppContext, event: Self::CustomEvent) {}
    fn on_close(&mut self, cx: AppContext) -> bool { true }
    fn window_desc(&self) -> WindowDescriptor;
}

pub struct AppContext<'a> {
    pub window: &'a Window,
    pub renderer: &'a mut Renderer,
}

struct AppRunner<A: SimpleApp<CustomEvent = E>, E: 'static> {
    app: A,
    state: AppState,
}

impl<A: SimpleApp<CustomEvent = E>, E: 'static> App for AppRunner<A, E> {
    type CustomEvent = E;

    fn on_event(&mut self, wm: WindowManager, event: AppEvent<E>) {
        match event {
            AppEvent::Custom(event) => {
                let AppState::Active { window, renderer, .. } = &mut self.state else {
                    return;
                };
                self.app.event(AppContext { window, renderer }, event);
            }
            AppEvent::Init => {}
            AppEvent::Suspend => {
                if let AppState::Active { window, .. } = &self.state {
                    self.state = AppState::Suspended(Some(window.clone()));
                }
            }
            AppEvent::Resume => {
                let AppState::Suspended(window) = &mut self.state else {
                    return;
                };
                let window = window.take().unwrap_or_else(|| {
                    wm.create_window(self.app.window_desc()).unwrap()
                });
                let (graphics, device, queue, format, backend) = pollster::block_on(async {
                    WindowGraphics::from_window(window.clone(), None).await
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
            AppEvent::Window { id: _, event } => {
                let AppState::Active {
                    window, graphics, viewport, renderer
                } = &mut self.state else {
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
