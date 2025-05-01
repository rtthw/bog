//! Application



use bog_event::WindowEvent;
use bog_layout::{Layout, LayoutTree, Node, Placement};
use bog_math::{glam::vec2, Rect, Vec2};
use bog_render::{Renderer, Viewport};
use bog_window::{
    WindowingClient, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem,
};

use crate::{
    graphics::WindowGraphics,
    ui::{UserInterface, UserInterfaceContext, UserInterfaceHandler},
    Result,
};



pub fn run_app(mut app: impl AppHandler) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;

    let mut ui = UserInterface::new(app.root_layout());
    app.init(&mut ui);

    let mut proxy = AppRunner {
        app: &mut app,
        state: AppState::Suspended(None),
        ui,
    };

    windowing_system.run_client(&mut proxy)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait AppHandler: 'static {
    fn render(&mut self, renderer: &mut Renderer, tree: &mut LayoutTree, viewport_rect: Rect);
    fn init(&mut self, ui: &mut UserInterface);

    fn window_desc(&self) -> WindowDescriptor;
    fn root_layout(&self) -> Layout;

    fn on_resize(&mut self, size: Vec2) {}
    fn on_mousemove(&mut self, pos: Vec2) {}
    fn on_mouseover(&mut self, node: Node, cx: AppContext) {}
    fn on_mouseleave(&mut self, node: Node, cx: AppContext) {}
    fn on_mousedown(&mut self, node: Node, cx: AppContext) {}
    fn on_mouseup(&mut self, node: Node, cx: AppContext) {}
    fn on_dragstart(&mut self, node: Node, cx: AppContext) {}
    fn on_dragend(&mut self, node: Node, cx: AppContext, over: Option<Node>) {}
    fn on_dragmove(&mut self, node: Node, cx: AppContext, delta: Vec2, over: Option<Node>) {}
    fn on_layout(&mut self, node: Node, placement: &Placement) {}
}

struct AppRunner<'a> {
    app: &'a mut dyn AppHandler,
    state: AppState,
    ui: UserInterface,
}

impl<'a> WindowingClient for AppRunner<'a> {
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
                self.app.render(renderer, self.ui.tree(), viewport.rect());
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
                    &mut Proxy { app: self.app, graphics, renderer },
                    physical_size,
                );
                window.request_redraw();
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                self.ui.handle_mouse_move(
                    &mut Proxy { app: self.app, graphics, renderer },
                    vec2(x, y),
                );
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.ui.handle_mouse_down(&mut Proxy { app: self.app, graphics, renderer });
                }
            }
            WindowEvent::MouseUp { code } => {
                if code == 0 {
                    self.ui.handle_mouse_up(&mut Proxy { app: self.app, graphics, renderer });
                }
            }
            _ => {}
        }
    }
}

struct Proxy<'a> {
    app: &'a mut dyn AppHandler,
    graphics: &'a mut WindowGraphics<'static>,
    renderer: &'a mut Renderer,
}

impl<'a> UserInterfaceHandler for Proxy<'a> {
    fn on_mouse_move(&mut self, pos: Vec2) {
        self.app.on_mousemove(pos);
    }

    fn on_mouse_enter(&mut self, node: Node, gui_cx: UserInterfaceContext) {
        self.app.on_mouseover(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_leave(&mut self, node: Node, gui_cx: UserInterfaceContext) {
        self.app.on_mouseleave(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_down(&mut self, node: Node, gui_cx: UserInterfaceContext) {
        self.app.on_mousedown(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_up(&mut self, node: Node, gui_cx: UserInterfaceContext) {
        self.app.on_mouseup(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_drag_move(&mut self, node: Node, gui_cx: UserInterfaceContext, delta: Vec2, over: Option<Node>) {
        self.app.on_dragmove(
            node,
            AppContext {
                graphics: self.graphics,
                renderer: self.renderer,
                gui_cx,
            },
            delta,
            over,
        );
    }

    fn on_drag_start(&mut self, node: Node, gui_cx: UserInterfaceContext) {
        self.app.on_dragstart(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_drag_end(&mut self, node: Node, gui_cx: UserInterfaceContext, over: Option<Node>) {
        self.app.on_dragend(node, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        }, over);
    }

    fn on_resize(&mut self, size: Vec2) {
        self.app.on_resize(size);
    }

    fn on_node_layout(&mut self, node: Node, placement: &Placement) {
        self.app.on_layout(node, placement);
    }
}

pub struct AppContext<'a> {
    pub graphics: &'a mut WindowGraphics<'static>,
    pub renderer: &'a mut Renderer,
    pub gui_cx: UserInterfaceContext<'a>,
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
