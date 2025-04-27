//! Application



use bog_event::WindowEvent;
use bog_layout::{Layout, Placement};
use bog_math::{glam::vec2, Rect, Vec2};
use bog_render::{Renderer, Viewport};
use bog_window::{Client, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem};

use crate::{graphics::WindowGraphics, gui::{Element, Gui, GuiContext, GuiHandler}, Result};



pub fn run_app(mut app: impl AppHandler) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;

    let mut ui = Gui::new(app.root_layout());
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
    fn render(&mut self, renderer: &mut Renderer, viewport_rect: Rect);
    fn init(&mut self, ui: &mut Gui);

    fn title(&self) -> &str;
    fn root_layout(&self) -> Layout;

    fn on_resize(&mut self, size: Vec2) {}
    fn on_mousemove(&mut self, pos: Vec2) {}
    fn on_mouseover(&mut self, element: Element, cx: AppContext) {}
    fn on_mouseleave(&mut self, element: Element, cx: AppContext) {}
    fn on_mousedown(&mut self, element: Element, cx: AppContext) {}
    fn on_mouseup(&mut self, element: Element, cx: AppContext) {}
    fn on_dragstart(&mut self, element: Element, cx: AppContext) {}
    fn on_dragend(&mut self, element: Element, cx: AppContext) {}
    fn on_dragmove(&mut self, element: Element, cx: AppContext, delta: Vec2, over: Option<Element>) {}
    fn on_layout(&mut self, element: Element, placement: &Placement) {}
}

struct AppRunner<'a> {
    app: &'a mut dyn AppHandler,
    state: AppState,
    ui: Gui,
}

impl<'a> Client for AppRunner<'a> {
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
                self.app.render(renderer, viewport.rect());
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

impl<'a> GuiHandler for Proxy<'a> {
    fn on_mouse_move(&mut self, pos: Vec2) {
        self.app.on_mousemove(pos);
    }

    fn on_mouse_enter(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_mouseover(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_leave(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_mouseleave(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_down(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_mousedown(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_mouse_up(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_mouseup(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_drag_update(
        &mut self, element: Element, gui_cx: GuiContext, delta: Vec2, hovered: Option<Element>,
    ) {
        self.app.on_dragmove(
            element,
            AppContext {
                graphics: self.graphics,
                renderer: self.renderer,
                gui_cx,
            },
            delta,
            hovered,
        );
    }

    fn on_drag_start(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_dragstart(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_drag_end(&mut self, element: Element, gui_cx: GuiContext) {
        self.app.on_dragend(element, AppContext {
            graphics: self.graphics,
            renderer: self.renderer,
            gui_cx,
        });
    }

    fn on_resize(&mut self, size: Vec2) {
        self.app.on_resize(size);
    }

    fn on_element_layout(&mut self, element: Element, placement: &Placement) {
        self.app.on_layout(element, placement);
    }
}

pub struct AppContext<'a> {
    pub graphics: &'a mut WindowGraphics<'static>,
    pub renderer: &'a mut Renderer,
    pub gui_cx: GuiContext<'a>,
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
        title: app.title(),
        ..Default::default()
    }).unwrap()
}
