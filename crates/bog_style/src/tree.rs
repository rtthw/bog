//! Node tree



use bog_alloc::alloc::vec::Vec;
use bog_math::{vec2, Vec2};

use crate::Style;



pub struct NodeTree {
    nodes: slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
    children: slotmap::SlotMap<slotmap::DefaultKey, Vec<u64>>,
    parents: slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
}

impl Default for NodeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeTree {
    /// Create a new node tree.
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Create a new node tree that can hold at least `capacity` nodes before re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: slotmap::SlotMap::with_capacity(capacity),
            children: slotmap::SlotMap::with_capacity(capacity),
            parents: slotmap::SlotMap::with_capacity(capacity),
        }
    }

    /// Clear all nodes from this tree.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Get the children of the given node.
    pub fn children(&self, node: u64) -> &[u64] {
        &self.children[slotmap::KeyData::from_ffi(node).into()]
    }
}



/// Any type that can be passed into [`NodeTree::compute_contextual_layout`].
pub trait LayoutContext {
    /// Measure the node's desired size given the available space left in the layout.
    fn measure_node(&mut self, node: u64, available_space: Vec2) -> Vec2;
}

impl LayoutContext for () {
    fn measure_node(&mut self, _node: u64, _available_space: Vec2) -> Vec2 {
        Vec2::ZERO
    }
}



// --- Private



struct NodeInfo {
    style: Style,

    cache: taffy::Cache,
    layout: taffy::Layout,
}



// --- Taffy Implementations



impl NodeTree {
    #[inline(always)]
    pub(crate) fn node_info(&self, node_id: taffy::NodeId) -> &NodeInfo {
        &self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }

    #[inline(always)]
    fn node_info_mut(&mut self, node_id: taffy::NodeId) -> &mut NodeInfo {
        &mut self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }
}

pub struct NodeTreeChildIter<'a>(core::slice::Iter<'a, u64>);

impl<'a> Iterator for NodeTreeChildIter<'a> {
    type Item = taffy::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(taffy::NodeId::from)
    }
}



struct NodeTreeProxy<'a, T: LayoutContext> {
    tree: &'a mut NodeTree,
    context: T,
}

impl<'a, T: LayoutContext> taffy::TraversePartialTree for NodeTreeProxy<'a, T> {
    type ChildIter<'b> = NodeTreeChildIter<'b> where Self: 'b;

    fn child_ids(&self, node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        NodeTreeChildIter(self.tree.children(node_id.into()).iter())
    }

    fn child_count(&self, node_id: taffy::NodeId) -> usize {
        self.tree.children(node_id.into()).len()
    }

    fn get_child_id(&self, node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        self.tree.children(node_id.into())[index].into()
    }
}

impl<'a, T: LayoutContext> taffy::LayoutPartialTree for NodeTreeProxy<'a, T> {
    type CoreContainerStyle<'b> = &'b Style where Self: 'b;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.tree.node_info(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.tree.node_info_mut(node_id).layout = *layout;
    }

    fn compute_child_layout(
        &mut self,
        node_id: taffy::NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        taffy::compute_cached_layout(self, node_id, inputs, |tree, id, inputs| {
            let display_mode = tree.tree.node_info(node_id).style.display;
            let has_children = tree.tree
                .children[slotmap::KeyData::from_ffi(node_id.into()).into()]
                .len() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(tree, id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(tree, id, inputs),
                (taffy::Display::Flex, true) => taffy::compute_flexbox_layout(tree, id, inputs),
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(tree, id, inputs),
                (_, false) => {
                    let style = &tree.tree.node_info(node_id).style;
                    taffy::compute_leaf_layout(inputs, style, |_dimensions, available_space| {
                        let size = tree.context.measure_node(id.into(), vec2(
                            // FIXME: This should be different based on whether avaible space is
                            //        min content or max content (probably zero for min and
                            //        infinity for max).
                            available_space.width.unwrap_or(f32::INFINITY),
                            available_space.height.unwrap_or(f32::INFINITY),
                        ));
                        taffy::Size {
                            width: size.x,
                            height: size.y,
                        }
                    })
                }
            }
        })
    }
}

impl<'a, T: LayoutContext> taffy::CacheTree for NodeTreeProxy<'a, T> {
    fn cache_get(
        &self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.tree.node_info(node_id).cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.tree.node_info_mut(node_id).cache.store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: taffy::NodeId) {
        self.tree.node_info_mut(node_id).cache.clear();
    }
}

impl<'a, T: LayoutContext> taffy::LayoutBlockContainer for NodeTreeProxy<'a, T> {
    type BlockContainerStyle<'b> = &'b Style where Self: 'b;
    type BlockItemStyle<'b> = &'b Style where Self: 'b;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.tree.node_info(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.tree.node_info(child_node_id).style
    }
}

impl<'a, T: LayoutContext> taffy::LayoutFlexboxContainer for NodeTreeProxy<'a, T> {
    type FlexboxContainerStyle<'b> = &'b Style where Self: 'b;
    type FlexboxItemStyle<'b> = &'b Style where Self: 'b;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.tree.node_info(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.tree.node_info(child_node_id).style
    }
}

impl<'a, T: LayoutContext> taffy::LayoutGridContainer for NodeTreeProxy<'a, T> {
    type GridContainerStyle<'b> = &'b Style where Self: 'b;
    type GridItemStyle<'b> = &'b Style where Self: 'b;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.tree.node_info(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.tree.node_info(child_node_id).style
    }
}



impl taffy::CoreStyle for Style {
    // TODO
}

impl taffy::BlockContainerStyle for Style {
    // TODO
}

impl taffy::BlockItemStyle for Style {
    // TODO
}

impl taffy::FlexboxContainerStyle for Style {
    // TODO
}

impl taffy::FlexboxItemStyle for Style {
    // TODO
}
