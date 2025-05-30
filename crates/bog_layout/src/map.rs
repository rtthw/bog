


use bog_alloc::alloc::vec::Vec;
use bog_math::{vec2, Rect, Vec2};
use slotmap::Key as _;

use crate::{Layout, Placement};



/// A tree of nodes identified by `u64`s and positioned with [`Layout`]s.
#[derive(Debug)]
pub struct LayoutMap {
    nodes: slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
    children: slotmap::SlotMap<slotmap::DefaultKey, Vec<u64>>,
    parents: slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
}

impl Default for LayoutMap {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutMap {
    /// Create a new layout map.
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Create a new layout map that can hold at least `capacity` nodes before re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: slotmap::SlotMap::with_capacity(capacity),
            children: slotmap::SlotMap::with_capacity(capacity),
            parents: slotmap::SlotMap::with_capacity(capacity),
        }
    }

    /// Clear this map.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.children.clear();
        self.parents.clear();
    }

    /// Get the children of the given node.
    pub fn children(&self, node: u64) -> &[u64] {
        &self.children[slotmap::KeyData::from_ffi(node).into()]
    }

    /// Get an owned vector of the children for the given node.
    pub fn children_owned(&self, node: u64) -> Vec<u64> {
        self.children[slotmap::KeyData::from_ffi(node).into()].clone()
    }

    /// Get the parent of the given node.
    pub fn parent(&self, node: u64) -> Option<u64> {
        self.parents[slotmap::KeyData::from_ffi(node).into()]
    }

    /// Get the [`Placement`] of a node, with the provided absolute position.
    pub fn placement(&self, node: u64, position: Vec2) -> Placement {
        let offset = self.node_info(node.into()).offset;
        let size = self.node_info(node.into()).layout.size;
        Placement {
            node,
            position,
            offset,
            parent_rect: Rect::new(position, vec2(size.width, size.height)),
            layout: &self.node_info(node.into()).layout,
            map: self,
        }
    }

    /// Add a node with the given [`Layout`] to the map, returning the allocated ID.
    pub fn add_node(&mut self, layout: Layout) -> u64 {
        let id = self.nodes.insert(NodeInfo {
            style: layout.into(),
            layout: taffy::Layout::with_order(0),
            cache: taffy::Cache::new(),
            offset: Vec2::ZERO,
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

    /// Get the [`Layout`] for the given node.
    pub fn get_layout(&self, node: u64) -> Layout {
        self.nodes[slotmap::KeyData::from_ffi(node).into()].style.clone().into()
    }

    /// Set the [`Layout`] for the given node.
    pub fn update_layout(&mut self, node: u64, layout: Layout) {
        self.nodes[slotmap::KeyData::from_ffi(node).into()].style = layout.into();
        self.mark_dirty(node);
    }

    /// Set the offset for the given node.
    pub fn set_offset(&mut self, node: u64, offset: Vec2) {
        self.nodes[slotmap::KeyData::from_ffi(node).into()].offset = offset;
        // Offset doesn't affect the layout computation, so no need to recompute.
    }

    /// Compute the layout for this map.
    pub fn compute_layout(&mut self, node: u64, available_space: Vec2) {
        taffy::compute_root_layout(
            &mut LayoutMapProxy {
                map: self,
                context: (),
            },
            node.into(),
            taffy::Size {
                width: taffy::AvailableSpace::Definite(available_space.x),
                height: taffy::AvailableSpace::Definite(available_space.y),
            },
        );
    }

    /// Compute the layout for this map with the given object that implements [`LayoutContext`].
    pub fn compute_contextual_layout<T: LayoutContext>(
        &mut self,
        node: u64,
        available_space: Vec2,
        context: T,
    ) {
        taffy::compute_root_layout(
            &mut LayoutMapProxy {
                map: self,
                context,
            },
            node.into(),
            taffy::Size {
                width: taffy::AvailableSpace::Definite(available_space.x),
                height: taffy::AvailableSpace::Definite(available_space.y),
            },
        );
    }

    /// Clear the internal cached layout for the given node, and all of its descendents.
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

    /// Get the absolute position of the given node.
    pub fn absolute_position(&self, node: u64) -> Vec2 {
        fn update_pos_recursive(
            nodes: &slotmap::SlotMap<slotmap::DefaultKey, NodeInfo>,
            parents: &slotmap::SlotMap<slotmap::DefaultKey, Option<u64>>,
            node_key: slotmap::DefaultKey,
            positon: &mut Vec2,
        ) {
            let location = nodes[node_key].layout.location;
            *positon += Vec2::new(location.x, location.y);

            if let Some(Some(node)) = parents.get(node_key) {
                update_pos_recursive(
                    nodes,
                    parents,
                    slotmap::KeyData::from_ffi(*node).into(),
                    positon,
                );
            }
        }

        let mut position = Vec2::ZERO;
        update_pos_recursive(
            &self.nodes,
            &self.parents,
            slotmap::KeyData::from_ffi(node).into(),
            &mut position,
        );

        position
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



// --- PRIVATE: Taffy Implementations



#[derive(Debug)]
pub(crate) struct NodeInfo {
    style: taffy::Style,
    pub(crate) layout: taffy::Layout,
    cache: taffy::Cache,
    pub(crate) offset: Vec2,
}

impl LayoutMap {
    #[inline(always)]
    pub(crate) fn node_info(&self, node_id: taffy::NodeId) -> &NodeInfo {
        &self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }

    #[inline(always)]
    fn node_info_mut(&mut self, node_id: taffy::NodeId) -> &mut NodeInfo {
        &mut self.nodes[slotmap::KeyData::from_ffi(node_id.into()).into()]
    }
}

pub struct LayoutNodeChildIter<'a>(core::slice::Iter<'a, u64>);

impl<'a> Iterator for LayoutNodeChildIter<'a> {
    type Item = taffy::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(taffy::NodeId::from)
    }
}

struct LayoutMapProxy<'a, T: LayoutContext> {
    map: &'a mut LayoutMap,
    context: T,
}

impl<'a, T: LayoutContext> taffy::TraversePartialTree for LayoutMapProxy<'a, T> {
    type ChildIter<'b> = LayoutNodeChildIter<'b> where Self: 'b;

    fn child_ids(&self, node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        LayoutNodeChildIter(self.map.children(node_id.into()).iter())
    }

    fn child_count(&self, node_id: taffy::NodeId) -> usize {
        self.map.children(node_id.into()).len()
    }

    fn get_child_id(&self, node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        self.map.children(node_id.into())[index].into()
    }
}

impl<'a, T: LayoutContext> taffy::LayoutPartialTree for LayoutMapProxy<'a, T> {
    type CoreContainerStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.map.node_info(node_id).style
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
            let display_mode = tree.map.node_info(node_id).style.display;
            let has_children = tree.map
                .children[slotmap::KeyData::from_ffi(node_id.into()).into()]
                .len() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(tree, id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(tree, id, inputs),
                (taffy::Display::Flex, true) => taffy::compute_flexbox_layout(tree, id, inputs),
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(tree, id, inputs),
                (_, false) => {
                    let style = &tree.map.node_info(node_id).style;
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

impl<'a, T: LayoutContext> taffy::CacheTree for LayoutMapProxy<'a, T> {
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

impl<'a, T: LayoutContext> taffy::LayoutBlockContainer for LayoutMapProxy<'a, T> {
    type BlockContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type BlockItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.map.node_info(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.map.node_info(child_node_id).style
    }
}

impl<'a, T: LayoutContext> taffy::LayoutFlexboxContainer for LayoutMapProxy<'a, T> {
    type FlexboxContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type FlexboxItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.map.node_info(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.map.node_info(child_node_id).style
    }
}

impl<'a, T: LayoutContext> taffy::LayoutGridContainer for LayoutMapProxy<'a, T> {
    type GridContainerStyle<'b> = &'b taffy::Style where Self: 'b;
    type GridItemStyle<'b> = &'b taffy::Style where Self: 'b;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.map.node_info(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.map.node_info(child_node_id).style
    }
}
