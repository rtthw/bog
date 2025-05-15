//! Bog View



use bog_collections::NoHashMap;
use bog_layout::{Layout, Placement};
use bog_math::{Rect, Vec2};
use bog_render::{Render as _, Renderer};



pub trait View {
    fn build(&mut self) -> Model<Self> where Self: Sized;
}



/// A view model really just a tree of [`Element`]s that have been attached to the [`View`].
pub struct Model<V: View> {
    elements: NoHashMap<u64, Option<Box<dyn Object<View = V>>>>,
    root_node: u64,
}

impl<V: View> Model<V> {
    pub fn grab(&mut self, node: u64) -> Option<Box<dyn Object<View = V>>> {
        self.elements.insert(node, None).and_then(|mut o| o.take())
    }
}



/// An element is essentially just a way of attaching an [`Object`] to a [`Model`].
pub struct Element<V: View> {
    object: Option<Box<dyn Object<View = V>>>,
    layout: Layout,
    children: Vec<Element<V>>,
}



#[allow(unused)]
pub trait Object {
    type View: View;

    fn render(&mut self, cx: RenderContext<Self::View>) {}
    fn pre_render(&mut self, cx: RenderContext<Self::View>) {}
    fn post_render(&mut self, cx: RenderContext<Self::View>) {}

    fn on_mouse_down(&mut self, cx: MouseDownContext<Self::View>) {}
    fn on_mouse_up(&mut self, cx: MouseUpContext<Self::View>) {}
    fn on_mouse_enter(&mut self, cx: MouseEnterContext<Self::View>) {}
    fn on_mouse_leave(&mut self, cx: MouseLeaveContext<Self::View>) {}

    fn on_drag_move(&mut self, cx: DragMoveContext<Self::View>) {}
    fn on_drag_start(&mut self, cx: DragStartContext<Self::View>) {}
    fn on_drag_end(&mut self, cx: DragEndContext<Self::View>) {}
}



pub struct RenderContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub renderer: &'a mut Renderer,
    pub placement: Placement<'a>,
}

pub fn render_view<V: View>(
    model: &mut Model<V>,
    view: &mut V,
    renderer: &mut Renderer,
    root_placement: Placement,
    viewport_rect: Rect,
) {
    renderer.clear();
    renderer.start_layer(viewport_rect);
    render_placement(root_placement, model, view, renderer);
    renderer.end_layer();
}

fn render_placement<V: View>(
    placement: Placement,
    model: &mut Model<V>,
    view: &mut V,
    renderer: &mut Renderer,
) {
    for child_placement in placement.children() {
        if let Some(mut obj) = model.grab(child_placement.node()) {
            obj.pre_render(RenderContext {
                view,
                model,
                renderer,
                placement: child_placement,
            });
            obj.render(RenderContext {
                view,
                model,
                renderer,
                placement: child_placement,
            });
            model.elements.insert(child_placement.node(), Some(obj));
        }

        render_placement(child_placement, model, view, renderer);

        if let Some(mut obj) = model.grab(child_placement.node()) {
            obj.post_render(RenderContext {
                view,
                model,
                renderer,
                placement: child_placement,
            });
            model.elements.insert(child_placement.node(), Some(obj));
        }
    }
}



pub struct MouseDownContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
}

pub struct MouseUpContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
}

pub struct MouseEnterContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
}

pub struct MouseLeaveContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
}

pub struct DragMoveContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
    pub over: Option<u64>,
    pub delta: Vec2,
}

pub struct DragStartContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
}

pub struct DragEndContext<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub node: u64,
    pub over: Option<u64>,
}
