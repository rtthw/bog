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

    let mut layout_map = LayoutMap::new();
    let view = View::new(app.view(), &mut layout_map);
    let ui = UserInterface::new(layout_map, view.root_node);

    let mut runner = AppRunner {
        app: &mut app,
        view,
        state: AppState::Suspended(None),
        ui,
    };

    windowing_system.run_client(&mut runner)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
#[allow(unused_variables)]
pub trait AppHandler {
    fn view(&mut self) -> Element;
    fn window_desc(&self) -> WindowDescriptor;
}

struct AppRunner<'a> {
    app: &'a mut dyn AppHandler,
    view: View,
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
                render_app(
                    self.app,
                    &mut self.view,
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
                    &mut Proxy { app: self.app, view: &mut self.view, graphics, window, renderer },
                    physical_size,
                );
                window.request_redraw();
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                self.ui.handle_mouse_move(
                    &mut Proxy { app: self.app, view: &mut self.view, graphics, window, renderer },
                    vec2(x, y),
                );
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.ui.handle_mouse_down(&mut Proxy {
                        app: self.app,
                        view: &mut self.view,
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
                        view: &mut self.view,
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

struct Proxy<'a> {
    app: &'a mut dyn AppHandler,
    view: &'a mut View,
    graphics: &'a mut WindowGraphics<'static>,
    window: &'a mut Window,
    renderer: &'a mut Renderer,
}

#[allow(unused)] // FIXME: Temporary.
impl<'a> UserInterfaceHandler for Proxy<'a> {
    fn on_mouse_move(&mut self, _pos: Vec2) {}

    fn on_mouse_enter(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_mouse_enter(self.app, AppContext {
                    graphics: self.graphics,
                    window: self.window,
                    renderer: self.renderer,
                    gui_cx,
                });
            }
        }
    }

    fn on_mouse_leave(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_mouse_leave(self.app, AppContext {
                    graphics: self.graphics,
                    window: self.window,
                    renderer: self.renderer,
                    gui_cx,
                });
            }
        }
    }

    fn on_mouse_down(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_mouse_down(self.app, AppContext {
                    graphics: self.graphics,
                    window: self.window,
                    renderer: self.renderer,
                    gui_cx,
                });
            }
        }
    }

    fn on_mouse_up(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_mouse_up(self.app, AppContext {
                    graphics: self.graphics,
                    window: self.window,
                    renderer: self.renderer,
                    gui_cx,
                });
            }
        }
    }

    fn on_drag_move(
        &mut self,
        node: u64,
        gui_cx: UserInterfaceContext,
        delta: Vec2,
        over: Option<u64>,
    ) {
    }

    fn on_drag_start(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_drag_start(self.app, AppContext {
                    graphics: self.graphics,
                    window: self.window,
                    renderer: self.renderer,
                    gui_cx,
                });
            }
        }
    }

    fn on_drag_end(&mut self, node: u64, gui_cx: UserInterfaceContext, over: Option<u64>) {
    }

    fn on_resize(&mut self, _size: Vec2) {}

    fn on_node_layout(&mut self, node: u64, placement: &Placement) {
    }
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



// ---



pub struct Element {
    object: Option<Box<dyn Object>>,
    layout: Layout,
    children: Vec<Element>,

    // render_callback: Option<RenderCallback>,
    // render_begin_callback: Option<RenderBeginCallback>,
    // render_end_callback: Option<RenderEndCallback>,

    // mouse_down_listener: Option<MouseDownListener>,
    // mouse_up_listener: Option<MouseUpListener>,
    // mouse_enter_listener: Option<MouseEnterListener>,
    // mouse_leave_listener: Option<MouseLeaveListener>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),

            // render_callback: None,
            // render_begin_callback: None,
            // render_end_callback: None,
            // mouse_down_listener: None,
            // mouse_up_listener: None,
            // mouse_enter_listener: None,
            // mouse_leave_listener: None,
        }
    }

    pub fn object(mut self, object: impl Object + 'static) -> Self {
        self.object = Some(Box::new(object));
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Element>) -> Self {
        self.children.extend(children.into_iter());
        self
    }

    pub fn child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }
}



#[allow(unused)]
pub trait Object {
    fn render(&mut self, renderer: &mut Renderer, placement: Placement) {}
    fn pre_render(&mut self, renderer: &mut Renderer, placement: Placement) {}
    fn post_render(&mut self, renderer: &mut Renderer, placement: Placement) {}

    // TODO: Should there be an associated type for the user's app here?
    fn on_mouse_down(&mut self, app: &mut dyn AppHandler, cx: AppContext) {}
    fn on_mouse_up(&mut self, app: &mut dyn AppHandler, cx: AppContext) {}
    fn on_mouse_enter(&mut self, app: &mut dyn AppHandler, cx: AppContext) {}
    fn on_mouse_leave(&mut self, app: &mut dyn AppHandler, cx: AppContext) {}

    fn on_drag_start(&mut self, app: &mut dyn AppHandler, cx: AppContext) {}
}

impl Object for () {}



struct ElementProxy {
    object: Option<Box<dyn Object>>,
}

pub struct View {
    elements: NoHashMap<u64, ElementProxy>,
    root_node: u64,
}

impl View {
    pub fn new(root: Element, layout_map: &mut LayoutMap) -> Self {
        layout_map.clear();
        let mut elements = NoHashMap::with_capacity(16);
        let root_node = layout_map.add_node(root.layout);

        push_elements_to_map(&mut elements, layout_map, root.children, root_node);

        Self {
            elements,
            root_node,
        }
    }
}

fn push_elements_to_map(
    element_map: &mut NoHashMap<u64, ElementProxy>,
    layout_map: &mut LayoutMap,
    elements: Vec<Element>,
    parent_node: u64,
) {
    for element in elements {
        let node = layout_map.add_node(element.layout);
        layout_map.add_child_to_node(parent_node, node);
        element_map.insert(node, ElementProxy {
            object: element.object,
        });

        push_elements_to_map(element_map, layout_map, element.children, node);
    }
}



fn render_app(
    app: &mut dyn AppHandler,
    view: &mut View,
    renderer: &mut Renderer,
    root_placement: Placement,
    viewport_rect: Rect,
) {
    renderer.clear();
    renderer.start_layer(viewport_rect);
    render_placement(root_placement, app, view, renderer);
    renderer.end_layer();
}

fn render_placement(
    placement: Placement,
    app: &mut dyn AppHandler,
    view: &mut View,
    renderer: &mut Renderer,
) {
    for child_placement in placement.children() {
        if let Some(obj) = view.elements.get_mut(&child_placement.node())
            .and_then(|e| e.object.as_mut())
        {
            obj.pre_render(renderer, child_placement);
            obj.render(renderer, child_placement);
        }

        render_placement(child_placement, app, view, renderer);

        if let Some(obj) = view.elements.get_mut(&child_placement.node())
            .and_then(|e| e.object.as_mut())
        {
            obj.post_render(renderer, child_placement);
        }
    }
}
