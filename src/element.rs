//! Element management



use bog_layout::{tree::Placement, Layout, LayoutMap};
use bog_math::Vec2;
use bog_render::{Quad, Render, Renderer};



pub trait Element {
    fn render(&self, renderer: &mut Renderer, placement: Placement);
    fn update(&self, node: u64, state: &StateTree, map: &mut LayoutMap);

    fn tag(&self) -> Tag { Tag::empty() }
    fn children(&self) -> Vec<StateTree> { Vec::new() }
    fn diff(&self, tree: &mut StateTree) { tree.children.clear(); }
}

pub struct StateTree {
    pub tag: Tag,
    pub children: Vec<StateTree>,
}

impl StateTree {
    pub fn empty() -> Self {
        Self {
            tag: Tag::empty(),
            children: Vec::new(),
        }
    }

    pub fn new<E: Element + Sized>(element: &E) -> Self {
        Self {
            tag: element.tag(),
            children: element.children(),
        }
    }

    pub fn diff<E: Element + Sized>(&mut self, new: &E) {
        if self.tag == new.tag() {
            new.diff(self);
        } else {
            *self = Self::new(new);
        }
    }

    pub fn diff_children<E: Element + Sized>(&mut self, new_children: &[E]) {
        self.diff_children_custom(
            new_children,
            |tree, element| tree.diff(element),
            |element| Self::new(element),
        );
    }

    pub fn diff_children_custom<T>(
        &mut self,
        new_children: &[T],
        diff: impl Fn(&mut StateTree, &T),
        new_state: impl Fn(&T) -> Self,
    ) {
        if self.children.len() > new_children.len() {
            self.children.truncate(new_children.len());
        }

        for (child_state, new) in self.children.iter_mut().zip(new_children.iter()) {
            diff(child_state, new);
        }

        if self.children.len() < new_children.len() {
            self.children.extend(new_children[self.children.len()..].iter().map(new_state));
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tag(core::any::TypeId);

impl Tag {
    pub fn of<T>() -> Self
    where
        T: 'static,
    {
        Self(core::any::TypeId::of::<T>())
    }

    pub fn empty() -> Self {
        Self::of::<()>()
    }
}



pub struct ElementTree<R: Element> {
    root_node: u64,
    root_element: R,
    root_state: StateTree,
    layout_map: LayoutMap,
}

impl<R: Element> ElementTree<R> {
    pub fn new(root_element: R, size: Vec2) -> Self {
        let mut layout_map = LayoutMap::new();
        let root_state = StateTree::new(&root_element);
        let root_node = layout_map.add_node(Layout::default().width(size.x).height(size.y));

        root_element.update(root_node, &root_state, &mut layout_map);

        Self {
            root_node,
            root_element,
            root_state,
            layout_map,
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.root_element.render(renderer, self.layout_map.placement(self.root_node, Vec2::ZERO));
    }

    pub fn update(&mut self) {
        self.root_state.diff(&self.root_element);
        self.root_element.update(self.root_node, &self.root_state, &mut self.layout_map);
    }
}



pub struct Container {
    pub children: Vec<Box<dyn Element>>,
    pub background: Quad,
}

impl Element for Container {
    fn render(&self, renderer: &mut Renderer, placement: Placement) {
        renderer.start_layer(placement.rect());
        renderer.fill_quad(self.background);
        for (element, placement) in self.children.iter().zip(placement.children()) {
            element.render(renderer, placement);
        }
        renderer.end_layer();
    }

    fn update(&self, node: u64, state: &StateTree, map: &mut LayoutMap) {
        // node.change_layout(map, self.layout.clone());
        for ((element, node), state) in self.children.iter()
            .zip(map.children_owned(node).into_iter())
            .zip(state.children.iter())
        {
            element.update(node, state, map);
        }
    }

    fn children(&self) -> Vec<StateTree> {
        self.children.iter().map(StateTree::new).collect()
    }

    fn diff(&self, tree: &mut StateTree) {
        tree.diff_children(&self.children);
    }
}



impl Element for Box<dyn Element> {
    fn render(&self, renderer: &mut Renderer, placement: Placement) {
        self.as_ref().render(renderer, placement);
    }

    fn update(&self, node: u64, state: &StateTree, map: &mut LayoutMap) {
        self.as_ref().update(node, state, map);
    }

    fn tag(&self) -> Tag {
        self.as_ref().tag()
    }

    fn children(&self) -> Vec<StateTree> {
        self.as_ref().children()
    }

    fn diff(&self, tree: &mut StateTree) {
        self.as_ref().diff(tree);
    }
}
