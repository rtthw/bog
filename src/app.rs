//! Application



use std::any::Any;

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
    fn render(
        &mut self,
        view: &mut View,
        renderer: &mut Renderer,
        root_placement: Placement<'_>,
        viewport_rect: Rect,
    );
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
                    &mut Proxy { app: self.app, view: &mut self.view, graphics, renderer },
                    physical_size,
                );
                window.request_redraw();
            }
            // WindowEvent::KeyDown { code, repeat } => {}
            // WindowEvent::KeyUp { code } => {}
            WindowEvent::MouseMove { x, y } => {
                self.ui.handle_mouse_move(
                    &mut Proxy { app: self.app, view: &mut self.view, graphics, renderer },
                    vec2(x, y),
                );
            }
            WindowEvent::MouseDown { code } => {
                if code == 0 {
                    self.ui.handle_mouse_down(&mut Proxy {
                        app: self.app,
                        view: &mut self.view,
                        graphics,
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
    renderer: &'a mut Renderer,
}

#[allow(unused)] // FIXME: Temporary.
impl<'a> UserInterfaceHandler for Proxy<'a> {
    fn on_mouse_move(&mut self, _pos: Vec2) {}

    fn on_mouse_enter(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(callback) = &element.on_mouse_enter {
                (callback)(&mut element.object, AppContext {
                    graphics: self.graphics,
                    renderer: self.renderer,
                    gui_cx,
                })
            }
        }
    }

    fn on_mouse_leave(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(callback) = &element.on_mouse_leave {
                (callback)(&mut element.object, AppContext {
                    graphics: self.graphics,
                    renderer: self.renderer,
                    gui_cx,
                })
            }
        }
    }

    fn on_mouse_down(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(callback) = &element.on_mouse_down {
                (callback)(&mut element.object, AppContext {
                    graphics: self.graphics,
                    renderer: self.renderer,
                    gui_cx,
                })
            }
        }
    }

    fn on_mouse_up(&mut self, node: u64, gui_cx: UserInterfaceContext) {
        if let Some(element) = self.view.elements.get_mut(&node) {
            if let Some(callback) = &element.on_mouse_up {
                (callback)(&mut element.object, AppContext {
                    graphics: self.graphics,
                    renderer: self.renderer,
                    gui_cx,
                })
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
    }

    fn on_drag_end(&mut self, node: u64, gui_cx: UserInterfaceContext, over: Option<u64>) {
    }

    fn on_resize(&mut self, _size: Vec2) {}

    fn on_node_layout(&mut self, node: u64, placement: &Placement) {
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



// ---



pub struct Element {
    object: Option<Box<dyn Any>>,
    layout: Layout,
    children: Vec<Element>,

    render_callback: Option<RenderCallback>,
    render_begin_callback: Option<RenderBeginCallback>,
    render_end_callback: Option<RenderEndCallback>,

    mouse_down_listener: Option<MouseDownListener>,
    mouse_up_listener: Option<MouseUpListener>,
    mouse_enter_listener: Option<MouseEnterListener>,
    mouse_leave_listener: Option<MouseLeaveListener>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),

            render_callback: None,
            render_begin_callback: None,
            render_end_callback: None,
            mouse_down_listener: None,
            mouse_up_listener: None,
            mouse_enter_listener: None,
            mouse_leave_listener: None,
        }
    }

    pub fn object(mut self, object: impl Any) -> Self {
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

    pub fn on_render(
        mut self,
        callback: impl Fn(&mut Renderer, Placement) + 'static,
    ) -> Self {
        self.render_callback = Some(Box::new(move |renderer, placement| {
            (callback)(renderer, placement)
        }));
        self
    }

    pub fn on_render_begin(
        mut self,
        callback: impl Fn(&mut Renderer, Placement) + 'static,
    ) -> Self {
        self.render_begin_callback = Some(Box::new(move |renderer, placement| {
            (callback)(renderer, placement)
        }));
        self
    }

    pub fn on_render_end(
        mut self,
        callback: impl Fn(&mut Renderer, Placement) + 'static,
    ) -> Self {
        self.render_end_callback = Some(Box::new(move |renderer, placement| {
            (callback)(renderer, placement)
        }));
        self
    }

    pub fn on_mouse_down(
        mut self,
        listener: impl Fn(&mut dyn Any, AppContext) + 'static,
    ) -> Self {
        self.mouse_down_listener = Some(Box::new(move |obj, app| {
            (listener)(obj, app)
        }));
        self
    }

    pub fn on_mouse_up(
        mut self,
        listener: impl Fn(&mut dyn Any, AppContext) + 'static,
    ) -> Self {
        self.mouse_up_listener = Some(Box::new(move |obj, app| {
            (listener)(obj, app)
        }));
        self
    }

    pub fn on_mouse_enter(
        mut self,
        listener: impl Fn(&mut dyn Any, AppContext) + 'static,
    ) -> Self {
        self.mouse_enter_listener = Some(Box::new(move |obj, app| {
            (listener)(obj, app)
        }));
        self
    }

    pub fn on_mouse_leave(
        mut self,
        listener: impl Fn(&mut dyn Any, AppContext) + 'static,
    ) -> Self {
        self.mouse_leave_listener = Some(Box::new(move |obj, app| {
            (listener)(obj, app)
        }));
        self
    }
}



struct ElementProxy {
    object: Option<Box<dyn Any>>,
    on_render: Option<RenderCallback>,
    on_render_begin: Option<RenderBeginCallback>,
    on_render_end: Option<RenderEndCallback>,
    on_mouse_down: Option<MouseDownListener>,
    on_mouse_up: Option<MouseUpListener>,
    on_mouse_enter: Option<MouseEnterListener>,
    on_mouse_leave: Option<MouseLeaveListener>,
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
            on_render: element.render_callback,
            on_render_begin: element.render_begin_callback,
            on_render_end: element.render_end_callback,
            on_mouse_down: element.mouse_down_listener,
            on_mouse_up: element.mouse_up_listener,
            on_mouse_enter: element.mouse_enter_listener,
            on_mouse_leave: element.mouse_leave_listener,
        });

        push_elements_to_map(element_map, layout_map, element.children, node);
    }
}



type RenderCallback = Box<dyn Fn(&mut Renderer, Placement) + 'static>;
type RenderBeginCallback = Box<dyn Fn(&mut Renderer, Placement) + 'static>;
type RenderEndCallback = Box<dyn Fn(&mut Renderer, Placement) + 'static>;

type MouseDownListener = Box<dyn Fn(&mut dyn Any, AppContext) + 'static>;
type MouseUpListener = Box<dyn Fn(&mut dyn Any, AppContext) + 'static>;
type MouseEnterListener = Box<dyn Fn(&mut dyn Any, AppContext) + 'static>;
type MouseLeaveListener = Box<dyn Fn(&mut dyn Any, AppContext) + 'static>;



fn render_app(
    app: &mut dyn AppHandler,
    view: &mut View,
    renderer: &mut Renderer,
    root_placement: Placement,
    viewport_rect: Rect,
) {
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
        let mut try_end_callback = false;
        if let Some(proxy) = view.elements.get_mut(&child_placement.node()) {
            try_end_callback = proxy.on_render_end.is_some();
            if let Some(cb) = &proxy.on_render_begin {
                (cb)(renderer, child_placement)
            }
            if let Some(cb) = &proxy.on_render {
                (cb)(renderer, child_placement)
            }
        }

        render_placement(child_placement, app, view, renderer);

        if try_end_callback {
            if let Some(proxy) = view.elements.get_mut(&child_placement.node()) {
                let Some(cb) = &proxy.on_render_end else { continue; };
                (cb)(renderer, child_placement)
            }
        }
    }
}
