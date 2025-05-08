//! Application handling



pub mod ui;

use std::any::Any;

use bog_collections::NoHashMap;
use bog_layout::{Layout, LayoutMap};



pub trait AppHandler {
    fn view(&mut self) -> View;
}



pub struct Element {
    object: Option<Box<dyn Any>>,
    layout: Layout,
    children: Vec<Element>,

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

    pub fn on_mouse_down(
        mut self,
        listener: impl Fn(&mut dyn Any, &MouseDownEvent, &mut AppContext) + 'static,
    ) -> Self {
        self.mouse_down_listener = Some(Box::new(move |obj, event, app| {
            (listener)(obj, event, app)
        }));
        self
    }

    pub fn on_mouse_up(
        mut self,
        listener: impl Fn(&mut dyn Any, &MouseUpEvent, &mut AppContext) + 'static,
    ) -> Self {
        self.mouse_up_listener = Some(Box::new(move |obj, event, app| {
            (listener)(obj, event, app)
        }));
        self
    }

    pub fn on_mouse_enter(
        mut self,
        listener: impl Fn(&mut dyn Any, &MouseEnterEvent, &mut AppContext) + 'static,
    ) -> Self {
        self.mouse_enter_listener = Some(Box::new(move |obj, event, app| {
            (listener)(obj, event, app)
        }));
        self
    }

    pub fn on_mouse_leave(
        mut self,
        listener: impl Fn(&mut dyn Any, &MouseLeaveEvent, &mut AppContext) + 'static,
    ) -> Self {
        self.mouse_leave_listener = Some(Box::new(move |obj, event, app| {
            (listener)(obj, event, app)
        }));
        self
    }
}



struct ElementProxy {
    object: Option<Box<dyn Any>>,
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
    pub fn new(root: Element) -> Self {
        let mut layout_map = LayoutMap::new();
        let mut elements = NoHashMap::with_capacity(16);
        let root_node = layout_map.add_node(root.layout);

        push_elements_to_map(&mut elements, &mut layout_map, root.children, root_node);

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
            on_mouse_down: element.mouse_down_listener,
            on_mouse_up: element.mouse_up_listener,
            on_mouse_enter: element.mouse_enter_listener,
            on_mouse_leave: element.mouse_leave_listener,
        });

        push_elements_to_map(element_map, layout_map, element.children, node);
    }
}



pub struct AppContext {}

impl AppContext {
    pub fn request_rebuild(&mut self) {}
}

pub struct MouseDownEvent {}
pub struct MouseUpEvent {}
pub struct MouseEnterEvent {}
pub struct MouseLeaveEvent {}

type MouseDownListener = Box<dyn Fn(&mut dyn Any, &MouseDownEvent, &mut AppContext) + 'static>;
type MouseUpListener = Box<dyn Fn(&mut dyn Any, &MouseUpEvent, &mut AppContext) + 'static>;
type MouseEnterListener = Box<dyn Fn(&mut dyn Any, &MouseEnterEvent, &mut AppContext) + 'static>;
type MouseLeaveListener = Box<dyn Fn(&mut dyn Any, &MouseLeaveEvent, &mut AppContext) + 'static>;



#[cfg(test)]
mod tests {
    use super::*;

    struct CustomObject {}

    struct CustomApp {}

    impl AppHandler for CustomApp {
        fn view(&mut self) -> View {
            let root = Element::new()
                .object(CustomObject {})
                .on_mouse_down(|_obj, _event, app| {
                    app.request_rebuild();
                })
                .child(Element::new());

            View::new(root)
        }
    }

    #[test]
    fn view_builder_works() {
        let _view = CustomApp {}.view();
    }
}
