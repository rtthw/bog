//! Bog view



pub mod style;
pub mod tree;


use std::marker::PhantomData;

use bog_core::{InputEvent, NoHashMap};
use bog_render::RenderPass;
use tree::ViewTree;



pub trait View: 'static {
    fn build(&self) -> Node;
}

pub trait Element: 'static {
    type View: View;

    fn render(&self, render: RenderContext) {}
    fn input(&mut self, input: InputContext) {}
}

pub struct RenderContext<'a> {
    pub pass: &'a mut RenderPass<'a>,
}

pub struct InputContext<'a> {
    pub event: InputEvent,
    pub propagate: &'a mut bool,
}



pub fn render_view(
    view: &mut impl View,
    tree: &mut ViewTree,
    root_element: &mut impl Element,
    root_id: u64,
    elements: NoHashMap<u64, Box<dyn ElementProxy>>,
) {
    view.build().0.crawl(root_id, tree, &mut |id, tree| {
        todo!()
    });
}



pub struct Node(InnerNode);

impl Node {
    pub fn leaf<E: Element>() -> Self {
        Self(InnerNode::Leaf(Box::new(ElementNodeDef::<E>::new())))
    }

    pub fn branch(children: impl IntoIterator<Item = Node>) -> Self {
        Self(InnerNode::Branch(children.into_iter().map(|n| n.0).collect()))
    }

    pub const fn none() -> Self {
        Self(InnerNode::None)
    }
}

enum InnerNode {
    Leaf(Box<dyn ElementNode>),
    Branch(Vec<InnerNode>),
    None,
}

impl InnerNode {
    fn crawl(
        &self,
        id: u64,
        tree: &mut ViewTree,
        func: &mut impl FnMut(u64, &mut ViewTree),
    ) {
        match self {
            InnerNode::Branch(nodes) => {
                let ids = tree.children(id).to_owned();

                assert!(nodes.len() == ids.len());

                for (node, id) in nodes.iter().zip(ids) {
                    node.crawl(id, tree, func);
                }
            }
            InnerNode::Leaf(_node) => {
                func(id, tree)
            }
            InnerNode::None => {}
        }
    }
}

trait ElementNode {}

struct ElementNodeDef<E: Element> {
    _element: PhantomData<E>,
}

impl<E: Element> ElementNode for ElementNodeDef<E> {}

impl<E: Element> ElementNodeDef<E> {
    fn new() -> Self {
        Self {
            _element: PhantomData,
        }
    }
}

pub trait ElementProxy {
    // TODO
}



#[cfg(test)]
mod tests {
    use super::*;

    struct TestView;
    struct TestElement;
    struct OtherElement;

    impl View for TestView {
        fn build(&self) -> Node {
            Node::leaf::<TestElement>()
        }
    }

    impl Element for TestElement {
        type View = TestView;
    }

    impl Element for OtherElement {
        type View = TestView;
    }
}
