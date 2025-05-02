


struct LayoutNode {
    children: Vec<LayoutNode>,
    style: taffy::Style,
    layout: taffy::Layout,
    cache: taffy::Cache,
}



// --- Taffy Implementations



impl LayoutNode {
    fn node_from_node_id(&self, node_id: taffy::NodeId) -> &LayoutNode {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &self.children[idx]
        }
    }

    fn node_from_node_id_mut(&mut self, node_id: taffy::NodeId) -> &mut LayoutNode {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &mut self.children[idx]
        }
    }
}

struct ChildIter(std::ops::Range<usize>);

impl Iterator for ChildIter {
    type Item = taffy::NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(taffy::NodeId::from)
    }
}

impl taffy::TraversePartialTree for LayoutNode {
    type ChildIter<'a> = ChildIter;

    fn child_ids(&self, _node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        ChildIter(0..self.children.len())
    }

    fn child_count(&self, _node_id: taffy::NodeId) -> usize {
        self.children.len()
    }

    fn get_child_id(&self, _node_id: taffy::NodeId, index: usize) -> taffy::NodeId {
        taffy::NodeId::from(index)
    }
}

impl taffy::LayoutPartialTree for LayoutNode {
    type CoreContainerStyle<'a>
        = &'a taffy::Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_node_id(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.node_from_node_id_mut(node_id).layout = *layout;
    }

    fn compute_child_layout(
        &mut self, node_id: taffy::NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        taffy::compute_cached_layout(self, node_id, inputs, |parent, id, inputs| {
            let display_mode = parent.children[usize::from(id)].style.display;
            let has_children = parent.children.len() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(parent, id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(parent, id, inputs),
                (taffy::Display::Flex, true) => taffy::compute_flexbox_layout(parent, id, inputs),
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(parent, id, inputs),
                (_, false) => {
                    let style = &parent.children[usize::from(id)].style;
                    taffy::compute_leaf_layout(inputs, style, |_dimensions, _available_space| {
                        taffy::Size::ZERO
                    })
                }
            }
        })
    }
}

impl taffy::CacheTree for LayoutNode {
    fn cache_get(
        &self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.node_from_node_id(node_id).cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.node_from_node_id_mut(node_id).cache.store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: taffy::NodeId) {
        self.node_from_node_id_mut(node_id).cache.clear();
    }
}

impl taffy::LayoutBlockContainer for LayoutNode {
    type BlockContainerStyle<'a> = &'a taffy::Style where Self: 'a;
    type BlockItemStyle<'a> = &'a taffy::Style where Self: 'a;

    fn get_block_container_style(&self, node_id: taffy::NodeId) -> Self::BlockContainerStyle<'_> {
        &self.node_from_node_id(node_id).style
    }

    fn get_block_child_style(&self, child_node_id: taffy::NodeId) -> Self::BlockItemStyle<'_> {
        &self.node_from_node_id(child_node_id).style
    }
}

impl taffy::LayoutFlexboxContainer for LayoutNode {
    type FlexboxContainerStyle<'a> = &'a taffy::Style where Self: 'a;
    type FlexboxItemStyle<'a> = &'a taffy::Style where Self: 'a;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_node_id(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_node_id(child_node_id).style
    }
}

impl taffy::LayoutGridContainer for LayoutNode {
    type GridContainerStyle<'a> = &'a taffy::Style where Self: 'a;
    type GridItemStyle<'a> = &'a taffy::Style where Self: 'a;

    fn get_grid_container_style(&self, node_id: taffy::NodeId) -> Self::GridContainerStyle<'_> {
        &self.node_from_node_id(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: taffy::NodeId) -> Self::GridItemStyle<'_> {
        &self.node_from_node_id(child_node_id).style
    }
}
