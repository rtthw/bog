//! View tree



use bog_core::{vec2, Vec2};
use slotmap::Key as _;

use crate::style::{LayoutStyle, Style, VisualStyle};



pub struct ViewTree {
    nodes: slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
    children: slotmap::SlotMap<slotmap::DefaultKey, Vec<u64>>,
    parents: slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
}

impl ViewTree {
    /// Create a new view tree.
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Create a new view tree that can hold at least `capacity` nodes before re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: slotmap::SlotMap::with_capacity(capacity),
            children: slotmap::SlotMap::with_capacity(capacity),
            parents: slotmap::SlotMap::with_capacity(capacity),
        }
    }

    /// Clear all stored node information from this view tree.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Get the children of the given node.
    pub fn children(&self, node: u64) -> &[u64] {
        &self.children[slotmap::KeyData::from_ffi(node).into()]
    }

    /// Get the parent of the given node, if there is one.
    pub fn parent(&self, node: u64) -> Option<u64> {
        self.parents[slotmap::KeyData::from_ffi(node).into()]
    }

    /// Add a node with the given [`Style`] to the tree, returning the newly created node ID.
    pub fn add_node(&mut self, style: Style) -> u64 {
        let id = self.nodes.insert(NodeInfo {
            style,
            layout: taffy::Layout::with_order(0),
            cache: taffy::Cache::new(),
        });
        let _ = self.children.insert(Vec::with_capacity(0));
        let _ = self.parents.insert(None);

        id.data().as_ffi()
    }

    /// Add the `child` node to `parent`.
    pub fn add_child_to_node(&mut self, parent: u64, child: u64) {
        let parent_key = slotmap::KeyData::from_ffi(parent).into();
        let child_key = slotmap::KeyData::from_ffi(child).into();
        self.parents[child_key] = Some(parent.into());
        self.children[parent_key].push(child.into());
        self.mark_dirty(parent.into());
    }

    /// Set the [`VisualStyle`] for the given node.
    pub fn set_visual_style(&mut self, node: u64, style: VisualStyle) {
        self.nodes[slotmap::KeyData::from_ffi(node).into()].style.visual = style;
    }

    /// Set the [`LayoutStyle`] for the given node.
    pub fn set_layout_style(&mut self, node: u64, style: LayoutStyle) {
        self.nodes[slotmap::KeyData::from_ffi(node).into()].style.layout = style;
        self.mark_dirty(node);
    }

    /// Clear the internal cached layout for the given node, and all of its descendents.
    /// This will force a recompute of the node's layout on the next `compute_layout` call.
    pub fn mark_dirty(&mut self, node: u64) {
        fn mark_dirty_recursive(
            nodes: &mut slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
            parents: &slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
            node_key: slotmap::DefaultKey,
        ) {
            nodes[node_key].cache.clear();

            if let Some(Some(node)) = parents.get(node_key) {
                mark_dirty_recursive(nodes, parents, slotmap::KeyData::from_ffi(*node).into());
            }
        }

        mark_dirty_recursive(
            &mut self.nodes,
            &self.parents,
            slotmap::KeyData::from_ffi(node).into(),
        );
    }
}

impl ViewTree {
    /// Compute the layout for this view tree.
    pub fn compute_layout(&mut self, node: u64, available_space: Vec2) {
        taffy::compute_root_layout(
            &mut ViewTreeProxy {
                tree: self,
                context: (),
            },
            node.into(),
            taffy::Size {
                width: taffy::AvailableSpace::Definite(available_space.x),
                height: taffy::AvailableSpace::Definite(available_space.y),
            },
        );
    }

    /// Compute the layout for this tree with the given [`LayoutContext`].
    pub fn compute_contextual_layout<T: LayoutContext>(
        &mut self,
        node: u64,
        available_space: Vec2,
        context: T,
    ) {
        taffy::compute_root_layout(
            &mut ViewTreeProxy {
                tree: self,
                context,
            },
            node.into(),
            taffy::Size {
                width: taffy::AvailableSpace::Definite(available_space.x),
                height: taffy::AvailableSpace::Definite(available_space.y),
            },
        );
    }
}



/// Any type that can be passed into [`ViewTree::compute_contextual_layout`].
pub trait LayoutContext {
    /// Measure the node's desired size given the available space left in the layout.
    fn measure_node(&mut self, node: u64, available_space: Vec2) -> Vec2;
}

impl LayoutContext for () {
    fn measure_node(&mut self, _node: u64, _available_space: Vec2) -> Vec2 {
        Vec2::ZERO
    }
}



// ---



struct NodeInfo {
    style: Style,
    layout: taffy::Layout,
    cache: taffy::Cache,
}



impl ViewTree {
    #[inline(always)]
    fn node_info(&self, node_id: taffy::NodeId) -> &NodeInfo {
        &self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }

    #[inline(always)]
    fn node_info_mut(&mut self, node_id: taffy::NodeId) -> &mut NodeInfo {
        &mut self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }
}

pub struct TreeNodeChildIter<'a>(core::slice::Iter<'a, u64>);

impl<'a> Iterator for TreeNodeChildIter<'a> {
    type Item = taffy::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(taffy::NodeId::from)
    }
}

struct ViewTreeProxy<'a, T: LayoutContext> {
    tree: &'a mut ViewTree,
    context: T,
}

impl<'a, T: LayoutContext> taffy::TraversePartialTree for ViewTreeProxy<'a, T> {
    type ChildIter<'b> = TreeNodeChildIter<'b> where Self: 'b;

    fn child_ids(&self, node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        TreeNodeChildIter(self.tree.children(node_id.into()).iter())
    }

    fn child_count(&self, node_id: taffy::NodeId) -> usize {
        self.tree.children(node_id.into()).len()
    }

    fn get_child_id(&self, node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        self.tree.children(node_id.into())[index].into()
    }
}

impl<'a, T: LayoutContext> taffy::LayoutPartialTree for ViewTreeProxy<'a, T> {
    type CoreContainerStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.tree.node_info(node_id).style.layout.0
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
            let display_mode = tree.tree.node_info(node_id).style.layout.0.display;
            let has_children = tree.tree
                .children[slotmap::KeyData::from_ffi(node_id.into()).into()]
                .len() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(tree, id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(tree, id, inputs),
                (taffy::Display::Flex, true) => taffy::compute_flexbox_layout(tree, id, inputs),
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(tree, id, inputs),
                (_, false) => {
                    let style = &tree.tree.node_info(node_id).style.layout;
                    taffy::compute_leaf_layout(
                        inputs,
                        style,
                        |_val, _basis| 0.0,
                        |_dimensions, available_space| {
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
                        },
                    )
                }
            }
        })
    }
}

impl<'a, T: LayoutContext> taffy::CacheTree for ViewTreeProxy<'a, T> {
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

impl<'a, T: LayoutContext> taffy::LayoutBlockContainer for ViewTreeProxy<'a, T> {
    type BlockContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type BlockItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.tree.node_info(node_id).style.layout.0
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.tree.node_info(child_node_id).style.layout.0
    }
}

impl<'a, T: LayoutContext> taffy::LayoutFlexboxContainer for ViewTreeProxy<'a, T> {
    type FlexboxContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type FlexboxItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.tree.node_info(node_id).style.layout.0
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.tree.node_info(child_node_id).style.layout.0
    }
}

impl<'a, T: LayoutContext> taffy::LayoutGridContainer for ViewTreeProxy<'a, T> {
    type GridContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type GridItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.tree.node_info(node_id).style.layout.0
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.tree.node_info(child_node_id).style.layout.0
    }
}
