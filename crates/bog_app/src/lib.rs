//! Application handling



use bog_layout::Layout;



pub trait Elemental {}

pub trait BoxableElement {
    fn into_box(self) -> ElementBox;
}

impl<T> BoxableElement for T where T: Elemental + 'static {
    fn into_box(self) -> ElementBox {
        ElementBox(Box::new(self))
    }
}



pub struct Element {
    layout: Layout,
    children: smallvec::SmallVec<[ElementBox; 2]>,

    mouse_down_listener: Option<MouseDownListener>,
}

impl Elemental for Element {}

// Builder functions.
impl Element {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            children: smallvec::SmallVec::new(),

            mouse_down_listener: None,
        }
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl BoxableElement>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_box()));
        self
    }

    pub fn child(mut self, child: impl BoxableElement) -> Self {
        self.children.push(child.into_box());
        self
    }

    pub fn on_mouse_down(
        mut self,
        listener: impl Fn(&MouseDownEvent, &mut dyn AppHandler) + 'static,
    ) {
        self.mouse_down_listener = Some(Box::new(move |event, app| {
            (listener)(event, app)
        }));
    }
}



pub struct ElementBox(Box<dyn Elemental>);

impl std::ops::Deref for ElementBox {
    type Target = dyn Elemental;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}



pub trait AppHandler {
    fn build(&mut self) -> Element;
}

pub struct MouseDownEvent {}

type MouseDownListener = Box<dyn Fn(&MouseDownEvent, &mut dyn AppHandler) + 'static>;



#[cfg(test)]
mod tests {
    use super::*;

    struct CustomElement {}

    impl Elemental for CustomElement {}

    #[test]
    fn element_supports_disparate_children() {
        let _element = Element::new()
            .child(Element::new())
            .child(CustomElement {});
    }
}
