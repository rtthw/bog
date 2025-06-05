//! View tree



use bog_core::{vec2, Vec2};

use crate::style::Style;



pub struct Tree {
    nodes: slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
    children: slotmap::SecondaryMap<slotmap::DefaultKey, Vec<u64>>,
}

impl Tree {
    /// Returns the children of the given node.
    pub fn children(&self, node: u64) -> &[u64] {
        &self.children[slotmap::KeyData::from_ffi(node).into()]
    }
}



/// Any type that can be passed into [`LayoutMap::compute_contextual_layout`].
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



impl Tree {
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

struct TreeProxy<'a, T: LayoutContext> {
    map: &'a mut Tree,
    context: T,
}

impl<'a, T: LayoutContext> taffy::TraversePartialTree for TreeProxy<'a, T> {
    type ChildIter<'b> = TreeNodeChildIter<'b> where Self: 'b;

    fn child_ids(&self, node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        TreeNodeChildIter(self.map.children(node_id.into()).iter())
    }

    fn child_count(&self, node_id: taffy::NodeId) -> usize {
        self.map.children(node_id.into()).len()
    }

    fn get_child_id(&self, node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        self.map.children(node_id.into())[index].into()
    }
}

impl<'a, T: LayoutContext> taffy::LayoutPartialTree for TreeProxy<'a, T> {
    type CoreContainerStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.map.node_info(node_id).style.layout.0
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.map.node_info_mut(node_id).layout = *layout;
    }

    fn compute_child_layout(
        &mut self,
        node_id: taffy::NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        taffy::compute_cached_layout(self, node_id, inputs, |tree, id, inputs| {
            let display_mode = tree.map.node_info(node_id).style.layout.0.display;
            let has_children = tree.map
                .children[slotmap::KeyData::from_ffi(node_id.into()).into()]
                .len() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(tree, id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(tree, id, inputs),
                (taffy::Display::Flex, true) => taffy::compute_flexbox_layout(tree, id, inputs),
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(tree, id, inputs),
                (_, false) => {
                    let style = &tree.map.node_info(node_id).style.layout;
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

impl<'a, T: LayoutContext> taffy::CacheTree for TreeProxy<'a, T> {
    fn cache_get(
        &self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.map.node_info(node_id).cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.map.node_info_mut(node_id).cache.store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: taffy::NodeId) {
        self.map.node_info_mut(node_id).cache.clear();
    }
}

impl<'a, T: LayoutContext> taffy::LayoutBlockContainer for TreeProxy<'a, T> {
    type BlockContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type BlockItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.map.node_info(node_id).style.layout.0
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.map.node_info(child_node_id).style.layout.0
    }
}

impl<'a, T: LayoutContext> taffy::LayoutFlexboxContainer for TreeProxy<'a, T> {
    type FlexboxContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type FlexboxItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.map.node_info(node_id).style.layout.0
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.map.node_info(child_node_id).style.layout.0
    }
}

impl<'a, T: LayoutContext> taffy::LayoutGridContainer for TreeProxy<'a, T> {
    type GridContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type GridItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.map.node_info(node_id).style.layout.0
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.map.node_info(child_node_id).style.layout.0
    }
}
