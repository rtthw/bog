//! Application


/* TODO

- Event propagation. This will probably involve an `Event` type that can be used by the event
  targets with just a simple flag that can tell the app proxy whether to "bubble up" events.
  Accessing ancestors should be as simple as calling `LayoutMap::parent` on the node. Should the UI
  handle event propagation, or is that best handled by the app?
- Figure out how to best dispatch events that involve multiple targets (like drags).

*/


use bog_collections::NoHashMap;
use bog_event::WindowEvent;
use bog_layout::{Layout, LayoutMap, Placement};
use bog_math::{glam::vec2, Rect, Vec2};
use bog_render::{Render, Renderer, Viewport};
use bog_view::*;
use bog_window::{
    WindowingClient, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem,
};

use crate::{
    graphics::WindowGraphics,
    ui::{UserInterface, UserInterfaceContext, UserInterfaceHandler},
    Result,
};



pub fn run_app<A: AppHandler>(mut app: A) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;

    let mut layout_map = LayoutMap::new();
    let model = app.build(&mut layout_map);
    let ui = UserInterface::new(layout_map, model.root_node());

    let mut runner = AppRunner {
        app: &mut app,
        model,
        state: AppState::Suspended(None),
        ui,
    };

    windowing_system.run_client(&mut runner)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait AppHandler: View {
    fn window_desc(&self) -> WindowDescriptor;
}

struct AppRunner<'a, A: AppHandler> {
    app: &'a mut A,
    model: Model<A>,
    state: AppState,
    ui: UserInterface,
}

impl<'a, A: AppHandler> WindowingClient for AppRunner<'a, A> {
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
                render_view(
                    &mut self.model,
                    self.app,
                    renderer,
                    self.ui.root_placement(),
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
                self.ui.handle_resize(
                    &mut Proxy { app: self.app, model: &mut self.model, graphics, window, renderer },
                    physical_size,
                );
                window.request_redraw();
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                self.ui.handle_mouse_move(
                    &mut Proxy { app: self.app, model: &mut self.model, graphics, window, renderer },
                    vec2(x, y),
                );
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.ui.handle_mouse_down(&mut Proxy {
                        app: self.app,
                        model: &mut self.model,
                        graphics,
                        window,
                        renderer,
                    });
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    self.ui.handle_mouse_up(&mut Proxy {
                        app: self.app,
                        model: &mut self.model,
                        graphics,
                        window,
                        renderer,
                    });
                }
            }
            _ => {}
        }
    }
}

struct Proxy<'a, A: AppHandler> {
    app: &'a mut A,
    model: &'a mut Model<A>,
    graphics: &'a mut WindowGraphics<'static>,
    window: &'a mut Window,
    renderer: &'a mut Renderer,
}

impl<'a, A: AppHandler> UserInterfaceHandler for Proxy<'a, A> {
    fn on_mouse_move(&mut self, _pos: Vec2) {}

    fn on_mouse_enter(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_mouse_enter();
            self.model.place(node, obj);
        }
    }

    fn on_mouse_leave(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_mouse_leave();
            self.model.place(node, obj);
        }
    }

    fn on_mouse_down(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_mouse_down();
            self.model.place(node, obj);
        }
    }

    fn on_mouse_up(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_mouse_up();
            self.model.place(node, obj);
        }
    }

    fn on_drag_move(
        &mut self,
        node: u64,
        gui_cx: UserInterfaceContext,
        delta: Vec2,
        over: Option<u64>,
    ) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_drag_move();
            self.model.place(node, obj);
        }
    }

    fn on_drag_start(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_drag_start();
            self.model.place(node, obj);
        }
    }

    fn on_drag_end(&mut self, node: u64, gui_cx: UserInterfaceContext, over: Option<u64>) {
        if let Some(mut obj) = self.model.grab(node) {
            // obj.on_drag_end();
            self.model.place(node, obj);
        }
    }

    fn on_resize(&mut self, _size: Vec2) {}

    fn on_node_layout(&mut self, _node: u64, _placement: &Placement) {}
}

pub struct AppContext<'a> {
    pub graphics: &'a mut WindowGraphics<'static>,
    pub window: &'a mut Window,
    pub renderer: &'a mut Renderer,
    pub gui_cx: UserInterfaceContext<'a>,
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



pub struct Context<'a, A: AppHandler> {
    app: &'a mut A,
    window: &'a mut Window,
    model: &'a mut Model<A>,

    viewport_size: Vec2,
    pointer_pos: Vec2,

    root_node: u64,
    hovered_node: Option<u64>,
    dragged_node: Option<u64>,
}
