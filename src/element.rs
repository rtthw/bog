//! Element management



use core::borrow::Borrow as _;

use bog_layout::{tree::Placement, Layout, LayoutMap};
use bog_math::Vec2;
use bog_render::Renderer;



pub trait Element {
    fn render(&self, renderer: &mut Renderer, placement: Placement);
    fn layout(&self, state: &StateTree) -> Layout;

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

    pub fn new(element: &dyn Element) -> Self {
        Self {
            tag: element.tag(),
            children: element.children(),
        }
    }

    pub fn diff(&mut self, new: &dyn Element) {
        if self.tag == new.tag() {
            new.diff(self);
        } else {
            *self = Self::new(new);
        }
    }

    pub fn diff_children(&mut self, new_children: &[&dyn Element]) {
        self.diff_children_custom(
            new_children,
            |tree, element| tree.diff(element.borrow()),
            |element| Self::new(element.borrow()),
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

        for (child_state, new) in
            self.children.iter_mut().zip(new_children.iter())
        {
            diff(child_state, new);
        }

        if self.children.len() < new_children.len() {
            self.children.extend(
                new_children[self.children.len()..].iter().map(new_state),
            );
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
    pub fn new(root_element: R) -> Self {
        let mut layout_map = LayoutMap::new();
        let root_state = StateTree::new(&root_element);
        let layout = root_element.layout(&root_state);
        let root_node = layout_map.add_node(layout);

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
    }
}
