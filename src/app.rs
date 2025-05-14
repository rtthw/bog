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



pub fn run_app<A: AppHandler>(mut app: A) -> Result<()> {
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
    fn view(&mut self) -> Element<Self> where Self: Sized;
    fn window_desc(&self) -> WindowDescriptor;
}

impl AppHandler for () {
    fn view(&mut self) -> Element<()> {
        Element::new()
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor::default()
    }
}

struct AppRunner<'a, A: AppHandler> {
    app: &'a mut A,
    view: View<A>,
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

struct Proxy<'a, A: AppHandler> {
    app: &'a mut A,
    view: &'a mut View<A>,
    graphics: &'a mut WindowGraphics<'static>,
    window: &'a mut Window,
    renderer: &'a mut Renderer,
}

impl<'a, A: AppHandler> UserInterfaceHandler for Proxy<'a, A> {
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
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_drag_move(self.app, DragMoveEvent {
                    app_cx: AppContext {
                        graphics: self.graphics,
                        window: self.window,
                        renderer: self.renderer,
                        gui_cx,
                    },
                    node,
                    over,
                    delta,
                });
            }
        }
    }

    fn on_drag_start(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_drag_start(self.app, DragStartEvent {
                    app_cx: AppContext {
                        graphics: self.graphics,
                        window: self.window,
                        renderer: self.renderer,
                        gui_cx,
                    },
                    node,
                });
            }
        }
    }

    fn on_drag_end(&mut self, node: u64, gui_cx: UserInterfaceContext, over: Option<u64>) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(obj) = &mut element.object {
                obj.on_drag_end(self.app, DragEndEvent {
                    app_cx: AppContext {
                        graphics: self.graphics,
                        window: self.window,
                        renderer: self.renderer,
                        gui_cx,
                    },
                    node,
                    over,
                });
            }
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



// ---



pub struct Element<A: AppHandler> {
    object: Option<Box<dyn Object<App = A>>>,
    layout: Layout,
    children: Vec<Element<A>>,
}

impl<A: AppHandler> Element<A> {
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),
        }
    }

    pub fn object(mut self, object: impl Object<App = A> + 'static) -> Self {
        self.object = Some(Box::new(object));
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Element<A>>) -> Self {
        self.children.extend(children.into_iter());
        self
    }

    pub fn child(mut self, child: Element<A>) -> Self {
        self.children.push(child);
        self
    }
}



#[allow(unused)]
pub trait Object {
    type App: AppHandler;

    fn render(&mut self, renderer: &mut Renderer, placement: Placement, app: &mut Self::App) {}
    fn pre_render(&mut self, renderer: &mut Renderer, placement: Placement, app: &mut Self::App) {}
    fn post_render(&mut self, renderer: &mut Renderer, placement: Placement, app: &mut Self::App) {}

    // TODO: Should there be an associated type for the user's app here?
    fn on_mouse_down(&mut self, app: &mut Self::App, cx: AppContext) {}
    fn on_mouse_up(&mut self, app: &mut Self::App, cx: AppContext) {}
    fn on_mouse_enter(&mut self, app: &mut Self::App, cx: AppContext) {}
    fn on_mouse_leave(&mut self, app: &mut Self::App, cx: AppContext) {}

    fn on_drag_move(&mut self, app: &mut Self::App, event: DragMoveEvent) {}
    fn on_drag_start(&mut self, app: &mut Self::App, event: DragStartEvent) {}
    fn on_drag_end(&mut self, app: &mut Self::App, event: DragEndEvent) {}
}

impl Object for () {
    type App = ();
}



pub struct DragMoveEvent<'a> {
    pub app_cx: AppContext<'a>,
    pub node: u64,
    pub over: Option<u64>,
    pub delta: Vec2,
}

pub struct DragStartEvent<'a> {
    pub app_cx: AppContext<'a>,
    pub node: u64,
}

pub struct DragEndEvent<'a> {
    pub app_cx: AppContext<'a>,
    pub node: u64,
    pub over: Option<u64>,
}



struct ElementProxy<A: AppHandler> {
    object: Option<Box<dyn Object<App = A>>>,
}

pub struct View<A: AppHandler> {
    elements: NoHashMap<u64, ElementProxy<A>>,
    root_node: u64,
}

impl<A: AppHandler> View<A> {
    pub fn new(root: Element<A>, layout_map: &mut LayoutMap) -> Self {
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

fn push_elements_to_map<A: AppHandler>(
    element_map: &mut NoHashMap<u64, ElementProxy<A>>,
    layout_map: &mut LayoutMap,
    elements: Vec<Element<A>>,
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



fn render_app<A: AppHandler>(
    app: &mut A,
    view: &mut View<A>,
    renderer: &mut Renderer,
    root_placement: Placement,
    viewport_rect: Rect,
) {
    renderer.clear();
    renderer.start_layer(viewport_rect);
    render_placement(root_placement, app, view, renderer);
    renderer.end_layer();
}

fn render_placement<A: AppHandler>(
    placement: Placement,
    app: &mut A,
    view: &mut View<A>,
    renderer: &mut Renderer,
) {
    for child_placement in placement.children() {
        if let Some(obj) = view.elements.get_mut(&child_placement.node())
            .and_then(|e| e.object.as_mut())
        {
            obj.pre_render(renderer, child_placement, app);
            obj.render(renderer, child_placement, app);
        }

        render_placement(child_placement, app, view, renderer);

        if let Some(obj) = view.elements.get_mut(&child_placement.node())
            .and_then(|e| e.object.as_mut())
        {
            obj.post_render(renderer, child_placement, app);
        }
    }
}
