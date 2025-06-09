//! Bog view



pub mod style;
pub mod tree;


use core::{any::TypeId, marker::PhantomData};

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



#[derive(PartialEq)]
pub struct Node(InnerNode);

// TODO: Better debug repr.
impl core::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node::{}", match &self.0 {
            InnerNode::None => "None",
            InnerNode::Branch(_nodes) => "Branch",
            InnerNode::Leaf(_node) => "Leaf",
        })
    }
}

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

#[derive(PartialEq)]
enum InnerNode {
    Leaf(Box<dyn ElementNode>),
    Branch(Vec<InnerNode>),
    None,
}

impl InnerNode {
    /// ## Panics
    ///
    /// If the `tree` is not in sync with this node (i.e. this node does not have the same number
    /// of children as the `tree` believes it does in [`ViewTree::children`]).
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

trait ElementNode: 'static {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl PartialEq for Box<dyn ElementNode> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

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
            Node::branch([
                Node::leaf::<TestElement>(),
                Node::branch([
                    Node::leaf::<OtherElement>(),
                    Node::leaf::<TestElement>(),
                ]),
                Node::leaf::<OtherElement>(),
                Node::leaf::<OtherElement>(),
            ])
        }
    }

    impl Element for TestElement {
        type View = TestView;
    }

    impl Element for OtherElement {
        type View = TestView;
    }

    #[test]
    fn node_equality() {
        let a = Node::leaf::<TestElement>();
        let b = Node::leaf::<OtherElement>();
        let c = Node::leaf::<TestElement>();

        assert_ne!(a, b);
        assert_eq!(c, a);
    }
}
