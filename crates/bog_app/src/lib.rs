//! Application handling



use std::any::Any;

use bog_collections::NoHashMap;
use bog_layout::{Layout, LayoutMap};



pub struct Element {
    object: Option<Box<dyn Any>>,
    layout: Layout,
    children: Vec<Element>,

    mouse_down_listener: Option<MouseDownListener>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            object: None,
            layout: Layout::default(),
            children: Vec::new(),

            mouse_down_listener: None,
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
}



struct ElementProxy {
    object: Option<Box<dyn Any>>,
    on_mouse_down: Option<MouseDownListener>,
}

pub struct View {
    elements: NoHashMap<u64, ElementProxy>,
    root_node: u64,
}

impl View {
    pub fn new(root: Element, layout_map: &mut LayoutMap) -> Self {
        let mut elements = NoHashMap::with_capacity(16);
        layout_map.clear();
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
            on_mouse_down: element.mouse_down_listener,
        });

        push_elements_to_map(element_map, layout_map, element.children, node);
    }
}



pub trait AppHandler {
    fn build(&mut self, layout_map: &mut LayoutMap) -> View;
}

pub struct AppContext {}

impl AppContext {
    pub fn request_rebuild(&mut self) {}
}

pub struct MouseDownEvent {}

type MouseDownListener = Box<dyn Fn(&mut dyn Any, &MouseDownEvent, &mut AppContext) + 'static>;



#[cfg(test)]
mod tests {
    use super::*;

    struct CustomObject {}

    struct CustomApp {}

    impl AppHandler for CustomApp {
        fn build(&mut self, layout_map: &mut LayoutMap) -> View {
            let root = Element::new()
                .object(CustomObject {})
                .on_mouse_down(|_obj, _event, app| {
                    app.request_rebuild();
                })
                .child(Element::new());

            View::new(root, layout_map)
        }
    }

    #[test]
    fn app_builder_works() {
        let mut layout_map = LayoutMap::new();
        let _root = CustomApp {}.build(&mut layout_map);
    }
}
