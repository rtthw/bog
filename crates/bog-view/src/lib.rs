//! Bog view



pub mod style;
pub mod tree;


use core::{any::TypeId, borrow::Borrow};

use bog_core::InputEvent;
use bog_render::RenderPass;
use tree::ViewTree;



pub trait Element: 'static {
    #[allow(unused_variables)]
    fn render(&self, render: RenderContext) {}

    #[allow(unused_variables)]
    fn input(&mut self, input: InputContext) {}

    fn tag(&self) -> Tag {
        Tag::null()
    }

    fn diff(&self, node: &mut Node) {
        node.children.clear();
    }

    fn children(&self) -> Vec<Node> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tag(TypeId);

impl Default for Tag {
    fn default() -> Self {
        Self::null()
    }
}

impl Tag {
    pub fn null() -> Self {
        Self(TypeId::of::<()>())
    }
}

pub struct RenderContext<'a> {
    pub pass: &'a mut RenderPass<'a>,
}

pub struct InputContext<'a> {
    pub event: InputEvent,
    pub propagate: &'a mut bool,
}



pub fn render_view(
    tree: &mut ViewTree,
    root: &mut impl Element,
) {
}



pub struct Node {
    pub tag: Tag,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new<'a, T: Borrow<dyn Element> + 'a>(element: T) -> Self {
        let element = element.borrow();

        Self {
            tag: element.tag(),
            children: element.children(),
        }
    }

    pub fn diff<'a, T: Borrow<dyn Element> + 'a>(&mut self, new: T) {
        if self.tag == new.borrow().tag() {
            new.borrow().diff(self);
        } else {
            *self = Self::new(new);
        }
    }

    pub fn diff_children<'a, T: Borrow<dyn Element> + 'a>(&mut self, new_children: &[T]) {
        self.diff_children_custom(
            new_children,
            |node, element| node.diff(element.borrow()),
            |element| Self::new(element.borrow()),
        );
    }

    pub fn diff_children_custom<T>(
        &mut self,
        new_children: &[T],
        diff: impl Fn(&mut Node, &T),
        new_state: impl Fn(&T) -> Self,
    ) {
        if self.children.len() > new_children.len() {
            self.children.truncate(new_children.len());
        }

        for (child_state, new) in self.children.iter_mut().zip(new_children.iter()) {
            diff(child_state, new);
        }

        if self.children.len() < new_children.len() {
            self.children.extend(
                new_children[self.children.len()..].iter().map(new_state),
            );
        }
    }
}

pub trait ElementProxy {
    // TODO
}



#[cfg(test)]
mod tests {
    use super::*;

    struct TestElement;
    struct OtherElement;

    impl Element for TestElement {}

    impl Element for OtherElement {}
}
