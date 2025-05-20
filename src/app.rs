//! Application


/* TODO

- Event propagation. This will probably involve an `Event` type that can be used by the event
  targets with just a simple flag that can tell the app proxy whether to "bubble up" events.
  Accessing ancestors should be as simple as calling `LayoutMap::parent` on the node. Should the UI
  handle event propagation, or is that best handled by the app?
- Figure out how to best dispatch events that involve multiple targets (like drags).

*/


use bog_event::WindowEvent;
use bog_layout::LayoutMap;
use bog_math::{vec2, Vec2};
use bog_render::{Renderer, Viewport};
use bog_view::*;
use bog_window::{
    WindowingClient, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem,
};

use crate::{
    graphics::WindowGraphics,
    Result,
};



pub fn run_app<A: AppHandler>(mut app: A) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;
    let mut layout_map = LayoutMap::new();
    let model = app.build(&mut layout_map);

    let mut runner = AppRunner {
        app,
        model,
        layout_map,
        state: AppState::Suspended(None),
    };

    windowing_system.run_client(&mut runner)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait AppHandler: View {
    fn window_desc(&self) -> WindowDescriptor;
}

struct AppRunner<A: AppHandler> {
    app: A,
    model: Model<A>,
    layout_map: LayoutMap,
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
                let root_placement = self.layout_map.placement(self.model.root_node(), Vec2::ZERO);
                render_view(
                    &mut self.model,
                    &mut self.app,
                    renderer,
                    root_placement,
                    viewport.rect(),
                );
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
                ModelProxy {
                    view: &mut self.app,
                    model: &mut self.model,
                    layout_map: &mut self.layout_map,
                    window: Some(&window),
                    renderer,
                }.handle_resize(physical_size);
                window.request_redraw();
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                ModelProxy {
                    view: &mut self.app,
                    model: &mut self.model,
                    layout_map: &mut self.layout_map,
                    window: Some(&window),
                    renderer,
                }.handle_mouse_move(vec2(x, y));
                window.request_redraw();
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    ModelProxy {
                        view: &mut self.app,
                        model: &mut self.model,
                        layout_map: &mut self.layout_map,
                        window: Some(&window),
                        renderer,
                    }.handle_mouse_down();
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    ModelProxy {
                        view: &mut self.app,
                        model: &mut self.model,
                        layout_map: &mut self.layout_map,
                        window: Some(&window),
                        renderer,
                    }.handle_mouse_up();
                }
            }
            WindowEvent::WheelMove(movement) => {
                ModelProxy {
                    view: &mut self.app,
                    model: &mut self.model,
                    layout_map: &mut self.layout_map,
                    window: Some(&window),
                    renderer,
                }.handle_wheel_movement(movement);
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
