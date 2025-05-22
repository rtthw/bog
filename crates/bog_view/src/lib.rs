//! Bog View



#[cfg(feature = "builtin-elements")]
pub mod elements;

use bog_collections::NoHashMap;
use bog_event::WheelMovement;
use bog_layout::{Layout, LayoutContext, LayoutMap, Placement};
use bog_math::{Rect, Vec2};
use bog_render::{Render as _, Renderer};
use bog_window::Window;



/// A view is a sort of top-level [`Object`] that owns and manages the other objects stored in the
/// [`Model`].
pub trait View {
    /// Build this view's associated [`Model`].
    fn build(&mut self, layout_map: &mut LayoutMap) -> Model<Self> where Self: Sized;
}



/// A view model really just a tree of [`Element`]s that have been attached to the [`View`].
pub struct Model<V: View> {
    elements: NoHashMap<u64, Option<Box<dyn Object<View = V>>>>,
    state: ModelState,
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
            state: ModelState {
                root_node,
                mouse_pos: Vec2::ZERO,
                viewport_size: Vec2::ZERO,
                hovered_node: None,
                is_dragging: false,
                drag_start_pos: None,
                drag_start_time: std::time::Instant::now(),
                drag_start_node: None,
                latest_wheel_movement: None,
            },
        }
    }

    pub fn state(&self) -> &ModelState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ModelState {
        &mut self.state
    }
}

impl ModelState {
    /// The node identifier for the root [`Element`] of this model.
    pub fn root_node(&self) -> u64 {
        self.root_node
    }

    /// The [`Placement`] of the root [`Element`] of this model.
    pub fn root_placement<'a>(&'a self, layout_map: &'a LayoutMap) -> Placement<'a> {
        layout_map.placement(self.root_node, Vec2::ZERO)
    }

    /// The current position of the user's mouse.
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_pos
    }

    /// The viewport's current [`Rect`].
    pub fn viewport_rect(&self) -> Rect {
        Rect::new(Vec2::ZERO, self.viewport_size)
    }

    /// The node currently being hovered, if any.
    pub fn hovered_node(&self) -> Option<u64> {
        self.hovered_node
    }

    /// The node currently being dragged, if any.
    pub fn dragged_node(&self) -> Option<u64> {
        self.drag_start_node
    }

    /// Returns `true` if the user is currently dragging a node.
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// The starting position of the user's drag, if any.
    pub fn drag_origin(&self) -> Option<Vec2> {
        self.drag_start_pos
    }

    /// The difference between the user's mouse position and drag origin.
    ///
    /// If the user is not dragging, this is `Vec2::ZERO`.
    pub fn drag_delta(&self) -> Vec2 {
        self.drag_start_pos.map(|pos| self.mouse_pos - pos)
            .unwrap_or(Vec2::ZERO)
    }

    /// The duration that the user has been dragging, if the user has been dragging.
    pub fn drag_duration(&self) -> Option<std::time::Duration> {
        if self.is_dragging {
            Some(std::time::Instant::now().duration_since(self.drag_start_time))
        } else {
            None
        }
    }

    /// Grab the latest wheel movement event, preventing node further down in the tree from
    /// accessing it.
    pub fn take_wheel_movement(&mut self) -> Option<WheelMovement> {
        self.latest_wheel_movement.take()
    }
}

pub struct ModelState {
    root_node: u64,
    mouse_pos: Vec2,
    viewport_size: Vec2,
    hovered_node: Option<u64>,
    is_dragging: bool,
    drag_start_pos: Option<Vec2>,
    drag_start_time: std::time::Instant,
    drag_start_node: Option<u64>,
    latest_wheel_movement: Option<WheelMovement>,
}

pub struct ModelProxy<'a, V: View> {
    pub view: &'a mut V,
    pub model: &'a mut Model<V>,
    pub layout_map: &'a mut LayoutMap,
    pub window: Option<&'a Window>,
    pub renderer: &'a mut Renderer,
}

impl<'a, V: View> ModelProxy<'a, V> {
    pub fn handle_resize(&mut self, new_size: Vec2) {
        if new_size == self.model.state.viewport_size {
            return;
        }
        self.model.state.viewport_size = new_size;
        let root_layout = self.layout_map.get_layout(self.model.state.root_node);
        self.layout_map.update_layout(
            self.model.state.root_node,
            root_layout.width(new_size.x).height(new_size.y),
        );
        self.layout_map.compute_contextual_layout(
            self.model.state.root_node,
            new_size,
            ModelProxyContext {
                // view: self.view,
                model: self.model,
                renderer: self.renderer,
            },
        );

        // Call `Object::on_placement` for all elements.
        // fn update_placements<'a, V: View>(
        //     placement: Placement,
        //     view: &'a mut V,
        //     model: &'a mut Model<V>,
        //     // window: &'a Window,
        //     renderer: &'a mut Renderer,
        // ) {
        //     if let Some(mut obj) = model.grab(placement.node()) {
        //         obj.on_placement(RenderContext {
        //             view,
        //             // model,
        //             // window,
        //             renderer,
        //             placement,
        //         });
        //         model.place(placement.node(), obj);
        //     }
        //     for placement in placement.children() {
        //         update_placements(placement, view, model, renderer);
        //     }
        // }
        // let root_placement = self.layout_map.placement(self.model.root_node, Vec2::ZERO);
        // update_placements(root_placement, self.view, self.model, self.renderer);
    }

    pub fn handle_mouse_move(&mut self, new_pos: Vec2) -> bool {
        if new_pos == self.model.state.mouse_pos {
            return false;
        }
        self.model.state.mouse_pos = new_pos;

        let mut should_redraw = false;
        let mut hovered = Vec::with_capacity(3);

        fn find_hovered(placement: Placement<'_>, hovered: &mut Vec<u64>, pos: Vec2) {
            if !placement.parent_rect().contains(pos) {
                return;
            }
            if !placement.offset_rect().contains(pos) {
                return;
            }

            hovered.push(placement.node());

            for child_placement in placement.children() {
                find_hovered(child_placement, hovered, pos);
            }
        }

        find_hovered(self.model.state.root_placement(self.layout_map), &mut hovered, new_pos);

        let topmost_hovered = hovered.last().copied();

        if let Some(_drag_origin_pos) = self.model.state.drag_start_pos {
            if let Some(drag_node) = self.model.state.drag_start_node {
                if !self.model.state.is_dragging {
                    let dur_since = std::time::Instant::now()
                        .duration_since(self.model.state.drag_start_time);
                    if dur_since.as_secs_f64() > 0.1 { // TODO: Custom hysteresis.
                        // User is likely dragging now.
                        self.model.state.is_dragging = true;
                        if let Some(Some(obj)) = self.model.elements.get_mut(&drag_node) {
                            obj.on_drag_start(EventContext {
                                node: drag_node,
                                view: self.view,
                                model: &mut self.model.state,
                                window: self.window,
                                renderer: self.renderer,
                                layout_map: self.layout_map,
                            });
                            // self.model.place(drag_node, obj);
                        }
                    }
                }
                // NOTE: We check twice here instead of just saying "else" because it could change
                //       in the first statement.
                if self.model.state.is_dragging {
                    if let Some(Some(obj)) = self.model.elements.get_mut(&drag_node) {
                        obj.on_drag_move(EventContext {
                            node: drag_node,
                            view: self.view,
                            model: &mut self.model.state,
                            window: self.window,
                            renderer: self.renderer,
                            layout_map: self.layout_map,
                        });
                        // self.model.place(drag_node, obj);
                    }
                }
                should_redraw = true;
            }
        }

        if self.model.state.hovered_node != topmost_hovered {
            if let Some(left_node) = self.model.state.hovered_node.take() {
                if let Some(Some(obj)) = self.model.elements.get_mut(&left_node) {
                    obj.on_mouse_leave(EventContext {
                        node: left_node,
                        view: self.view,
                        model: &mut self.model.state,
                        window: self.window,
                        renderer: self.renderer,
                        layout_map: self.layout_map,
                    });
                    // self.model.place(left_node, obj);
                }
            }
            if let Some(entered_node) = topmost_hovered {
                if let Some(Some(obj)) = self.model.elements.get_mut(&entered_node) {
                    obj.on_mouse_enter(EventContext {
                        node: entered_node,
                        view: self.view,
                        model: &mut self.model.state,
                        window: self.window,
                        renderer: self.renderer,
                        layout_map: self.layout_map,
                    });
                    // self.model.place(entered_node, obj);
                }
                self.model.state.hovered_node = Some(entered_node);
            }

            should_redraw = true;
        }

        should_redraw
    }

    pub fn handle_mouse_down(&mut self) {
        if let Some(node) = self.model.state.hovered_node {
            if let Some(Some(obj)) = self.model.elements.get_mut(&node) {
                obj.on_mouse_down(EventContext {
                    node,
                    view: self.view,
                    model: &mut self.model.state,
                    window: self.window,
                    renderer: self.renderer,
                    layout_map: self.layout_map,
                });
                // self.model.place(node, obj);
            }
        }
        self.model.state.drag_start_time = std::time::Instant::now();
        self.model.state.drag_start_pos = Some(self.model.state.mouse_pos);
        self.model.state.drag_start_node = self.model.state.hovered_node.clone();
    }

    pub fn handle_mouse_up(&mut self) {
        if let Some(node) = self.model.state.hovered_node {
            if let Some(Some(obj)) = self.model.elements.get_mut(&node) {
                obj.on_mouse_up(EventContext {
                    node,
                    view: self.view,
                    model: &mut self.model.state,
                    window: self.window,
                    renderer: self.renderer,
                    layout_map: self.layout_map,
                });
                // self.model.place(node, obj);
            }
        }
        self.model.state.drag_start_pos = None;
        if let Some(node) = self.model.state.drag_start_node.take() {
            if self.model.state.is_dragging {
                self.model.state.is_dragging = false;
                if let Some(Some(obj)) = self.model.elements.get_mut(&node) {
                    obj.on_drag_end(EventContext {
                        node,
                        view: self.view,
                        model: &mut self.model.state,
                        window: self.window,
                        renderer: self.renderer,
                        layout_map: self.layout_map,
                    });
                    // self.model.place(node, obj);
                }
            }
        }
    }

    pub fn handle_wheel_movement(&mut self, movement: WheelMovement) {
        self.model.state.latest_wheel_movement = Some(movement);
        if let Some(node) = self.model.state.hovered_node {
            if let Some(Some(obj)) = self.model.elements.get_mut(&node) {
                obj.on_wheel(EventContext {
                    node,
                    view: self.view,
                    model: &mut self.model.state,
                    window: self.window,
                    renderer: self.renderer,
                    layout_map: self.layout_map,
                });
                // self.model.place(node, obj);
            }
        }
    }
}

struct ModelProxyContext<'a, V: View> {
    // view: &'a mut V,
    model: &'a mut Model<V>,
    renderer: &'a mut Renderer,
}

impl<'a, V: View> LayoutContext for ModelProxyContext<'a, V> {
    fn measure_node(&mut self, node: u64, available_space: Vec2) -> Vec2 {
        let mut size = Vec2::ZERO;
        if let Some(Some(obj)) = self.model.elements.get_mut(&node) {
            size = obj.measure(available_space, self.renderer);
            // self.model.place(node, obj);
        }

        size
    }
}



/// An element is essentially just a way of attaching an [`Object`] to a [`Model`].
pub struct Element<V: View> {
    object: Option<Box<dyn Object<View = V>>>,
    layout: Layout,
    children: Vec<Element<V>>,

    mouseenter_listener: Option<EventListener<V>>,
    mouseleave_listener: Option<EventListener<V>>,
}

impl<V: View> Element<V> {
    /// Create an empty element with no associated [`Object`], the default [`Layout`], and no
    /// children.
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),

            mouseenter_listener: None,
            mouseleave_listener: None,
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
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<Element<V>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(|e| e.into()));
        self
    }

    /// Add the given child to this element.
    pub fn child(mut self, child: impl Into<Element<V>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<V: View> Element<V> {
    pub fn on_mouseenter<F>(mut self, listener: F) -> Self
    where
        F: Fn(EventContext<V>) + 'static,
    {
        self.mouseenter_listener = Some(Box::new(listener));
        self
    }

    pub fn on_mouseleave<F>(mut self, listener: F) -> Self
    where
        F: Fn(EventContext<V>) + 'static,
    {
        self.mouseleave_listener = Some(Box::new(listener));
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

    fn measure(&self, available_space: Vec2, renderer: &mut Renderer) -> Vec2 { Vec2::ZERO }

    /// This function is called during the object's render pass. Use it to render primitives with
    /// the [`Renderer`].
    fn render(&mut self, cx: RenderContext<Self::View>) {}
    /// This function is called immediately before the object's render pass. Use to alter the
    /// [`Renderer`] in ways that will affect this object's descendants.
    fn pre_render(&mut self, cx: RenderContext<Self::View>) {}
    /// This function will be called after all descendants of this object have finished their
    /// render passes.
    fn post_render(&mut self, cx: RenderContext<Self::View>) {}

    // /// This function is called every time this object's [`Placement`] is updated. This does not
    // /// necessarily mean that the area taken up by the object has changed, just that the node's
    // /// layout needed to be recomputed for some reason.
    // fn on_placement(&mut self, cx: RenderContext<Self::View>) {}

    /// This function is called when the user clicks down with the primary mouse button on this
    /// object.
    fn on_mouse_down(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called when the user releases a click with the primary mouse button on
    /// this object.
    fn on_mouse_up(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called immediately after the user's mouse pointer enters this object's
    /// [`Placement`] area.
    fn on_mouse_enter(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called immediately after the user's mouse pointer leaves this object's
    /// [`Placement`] area.
    fn on_mouse_leave(&mut self, cx: EventContext<Self::View>) {}

    fn on_wheel(&mut self, cx: EventContext<Self::View>) {}

    /// This function is called when the user's mouse pointer moves while this object is being
    /// dragged.
    fn on_drag_move(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called when the user begins dragging this object with the mouse pointer
    /// (when a mouse down event occurs followed by a definitive drag action).
    fn on_drag_start(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called when the user finishes dragging this object with the mouse pointer
    /// (a mouse up event occurs while dragging this object).
    fn on_drag_end(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called when the user's mouse pointer enters the [`Placement`] area of this
    /// object. This is similar to [`Object::on_mouse_enter`] with the added context of the
    /// currently dragged node.
    fn on_drag_over(&mut self, cx: EventContext<Self::View>) {}
    /// This function is called when the user finishes dragging an object while hovering this
    /// object.
    fn on_drag_drop(&mut self, cx: EventContext<Self::View>) {}
}



pub type EventListener<V> = Box<dyn Fn(EventContext<V>) + 'static>;



pub struct RenderContext<'a, V: View> {
    pub view: &'a mut V,
    // pub model: &'a mut Model<V>,
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
        if let Some(Some(obj)) = model.elements.get_mut(&child_placement.node()) {
            obj.pre_render(RenderContext {
                view,
                // model,
                renderer,
                placement: child_placement,
            });
            obj.render(RenderContext {
                view,
                // model,
                renderer,
                placement: child_placement,
            });
            // model.place(child_placement.node(), obj);
        }

        render_placement(child_placement, model, view, renderer);

        if let Some(Some(obj)) = model.elements.get_mut(&child_placement.node()) {
            obj.post_render(RenderContext {
                view,
                // model,
                renderer,
                placement: child_placement,
            });
            // model.place(child_placement.node(), obj);
        }
    }
}



pub struct EventContext<'a, V: View> {
    pub node: u64,
    pub view: &'a mut V,
    pub model: &'a mut ModelState,
    pub window: Option<&'a Window>,
    pub renderer: &'a mut Renderer,
    pub layout_map: &'a mut LayoutMap,
}

impl<'a, V: View> EventContext<'a, V> {
    pub fn get_layout(&self) -> Layout {
        self.layout_map.get_layout(self.node)
    }

    pub fn change_layout(&mut self, new_layout: Layout) {
        self.layout_map.update_layout(self.node, new_layout);
    }
}
