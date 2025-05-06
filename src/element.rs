//! Element management



use bog_layout::{tree::Placement, Layout, LayoutMap};
use bog_math::Vec2;
use bog_render::Renderer;



pub trait RootElement {
    fn render(&self, renderer: &mut Renderer, placement: Placement);

    fn layout(&self) -> Layout {
        Layout::default()
    }

    fn children(&mut self) -> impl Iterator<Item = &mut dyn Element>;
}

pub trait Element {
    fn render(&self, renderer: &mut Renderer, placement: Placement);
}



pub struct ElementTree<R: RootElement> {
    root_node: u64,
    root_element: R,
    layout_map: LayoutMap,
}

impl<R: RootElement> ElementTree<R> {
    pub fn new(root_element: R) -> Self {
        let mut layout_map = LayoutMap::new();
        let layout = root_element.layout();
        let root_node = layout_map.add_node(layout);

        Self {
            root_node,
            root_element,
            layout_map,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.root_element.render(renderer, self.layout_map.placement(self.root_node, Vec2::ZERO));
    }
}
