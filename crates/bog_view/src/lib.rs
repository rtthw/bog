//! Bog View



use bog_collections::NoHashMap;
use bog_layout::{Layout, LayoutMap, Placement};
use bog_math::{Rect, Vec2};
use bog_render::{Render as _, Renderer};



/// A view is a sort of top-level [`Object`] that owns and manages the other objects stored in the
/// [`Model`].
pub trait View {
    /// Build this view's associated [`Model`].
    fn build(&mut self) -> Model<Self> where Self: Sized;
}



/// A view model really just a tree of [`Element`]s that have been attached to the [`View`].
pub struct Model<V: View> {
    elements: NoHashMap<u64, Option<Box<dyn Object<View = V>>>>,
    root_node: u64,
}

impl<V: View> Model<V> {
    /// Create a new view model with it's element tree as defined by the given root [`Element`].
    /// The provided [`LayoutMap`] will also be updated to match this model.
    pub fn new(root: Element<V>, layout_map: &mut LayoutMap) -> Self {
        fn digest_elements<V: View>(
            element_map: &mut NoHashMap<u64, Option<Box<dyn Object<View = V>>>>,
            layout_map: &mut LayoutMap,
            elements: Vec<Element<V>>,
            parent_node: u64,
        ) {
            for element in elements {
                let node = layout_map.add_node(element.layout);
                layout_map.add_child_to_node(parent_node, node);
                if let Some(obj) = element.object { // Try to avoid allocating if possible.
                    let _ = element_map.insert(node, Some(obj));
                }

                digest_elements(element_map, layout_map, element.children, node);
            }
        }

        layout_map.clear();
        let mut elements = NoHashMap::with_capacity(16);
        let root_node = layout_map.add_node(root.layout);

        digest_elements(&mut elements, layout_map, root.children, root_node);

        Self {
            elements,
            root_node,
        }
    }

    /// The the node identifier for the root [`Element`] of this model.
    pub fn root_node(&self) -> u64 {
        self.root_node
    }

    /// Attempt to grab an [`Object`] out of this model. If you do not call [`Model::place`] after
    /// using the object, then the object will be dropped, and therefore inaccessible until
    /// replaced.
    pub fn grab(&mut self, node: u64) -> Option<Box<dyn Object<View = V>>> {
        self.elements.insert(node, None).and_then(|mut o| o.take())
    }

    /// Place an element into this model.
    pub fn place(&mut self, node: u64, obj: Box<dyn Object<View = V>>) {
        let _ = self.elements.insert(node, Some(obj));
    }
}



/// An element is essentially just a way of attaching an [`Object`] to a [`Model`].
pub struct Element<V: View> {
    object: Option<Box<dyn Object<View = V>>>,
    layout: Layout,
    children: Vec<Element<V>>,
}

impl<V: View> Element<V> {
    /// Create an empty element with no associated [`Object`], the default [`Layout`], and no
    /// children.
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),
        }
    }

    /// Associate the given [`Object`] with this element.
    pub fn object(mut self, object: impl Object<View = V> + 'static) -> Self {
        self.object = Some(Box::new(object));
        self
    }

    /// Make this element use the given [`Layout`].
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Add the given children to this element.
    pub fn children(mut self, children: impl IntoIterator<Item = Element<V>>) -> Self {
        self.children.extend(children.into_iter());
        self
    }

    /// Add the given child to this element.
    pub fn child(mut self, child: Element<V>) -> Self {
        self.children.push(child);
        self
    }
}



/// An object is just a set of callbacks for UI events.
///
/// You may choose to store persistent data in objects, or leave them as unit types. Note that if
/// you do store persistent information in objects, they may be wiped if you do not properly
/// synchronize (e.g. re-create) them in [`View::build`].
#[allow(unused)]
pub trait Object {
    type View: View;

    /// This function is called during the object's render pass. Use it to render primitives with
    /// the [`Renderer`].
    fn render(&mut self, cx: RenderContext<Self::View>) {}
    /// This function is called immediately before the object's render pass. Use to alter the
    /// [`Renderer`] in ways that will affect this object's descendants.
    fn pre_render(&mut self, cx: RenderContext<Self::View>) {}
    /// This function will be called after all descendants of this object have finished their
    /// render passes.
    fn post_render(&mut self, cx: RenderContext<Self::View>) {}

    /// This function is called when the user clicks down with the primary mouse button on this
    /// object.
    fn on_mouse_down(&mut self, cx: MouseDownContext<Self::View>) {}
    /// This function is called when the user releases a click with the primary mouse button on
    /// this object.
    fn on_mouse_up(&mut self, cx: MouseUpContext<Self::View>) {}
    /// This function is called immediately after the user's mouse pointer enters this object's
    /// [`Placement`] area.
    fn on_mouse_enter(&mut self, cx: MouseEnterContext<Self::View>) {}
    /// This function is called immediately after the user's mouse pointer leaves this object's
    /// [`Placement`] area.
    fn on_mouse_leave(&mut self, cx: MouseLeaveContext<Self::View>) {}

    /// This function is called when the user's mouse pointer moves while this object is being
    /// dragged.
    fn on_drag_move(&mut self, cx: DragMoveContext<Self::View>) {}
    /// This function is called when the user begins dragging this object with the mouse pointer
    /// (when a mouse down event occurs followed by a definitive drag action).
    fn on_drag_start(&mut self, cx: DragStartContext<Self::View>) {}
    /// This function is called when the user finished dragging this object with the mouse pointer
    /// (a mouse up event occurs while dragging this object).
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
            model.place(child_placement.node(), obj);
        }

        render_placement(child_placement, model, view, renderer);

        if let Some(mut obj) = model.grab(child_placement.node()) {
            obj.post_render(RenderContext {
                view,
                model,
                renderer,
                placement: child_placement,
            });
            model.place(child_placement.node(), obj);
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
