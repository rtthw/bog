//! Layout



use bog_core::Xy;

use crate::Element;



pub struct LayoutCache {
    layout: taffy::Layout,
    cache: taffy::Cache,
}

impl LayoutCache {
    pub fn new() -> Self {
        Self {
            cache: taffy::Cache::new(),
            layout: taffy::Layout::new(), // TODO: Ordered?
        }
    }
}

pub struct Style {
    inner: taffy::Style,
}

// Builder.
impl Style {
    pub fn new() -> Self {
        Self {
            inner: taffy::Style::DEFAULT,
        }
    }
}



// --- Internal



struct Node<'e> {
    element: &'e mut Box<dyn Element>,
}

impl Node<'_> {
    fn from_id(&self, node_id: taffy::NodeId) -> &Box<dyn Element> {
        let index = usize::from(node_id);
        if index == usize::MAX {
            &self.element
        } else {
            self.element.child_at(index)
        }
    }

    fn from_id_mut(&mut self, node_id: taffy::NodeId) -> &mut Box<dyn Element> {
        let index = usize::from(node_id);
        if index == usize::MAX {
            self.element
        } else {
            self.element.child_at_mut(index)
        }
    }
}

struct ChildIter(core::ops::Range<usize>);

impl Iterator for ChildIter {
    type Item = taffy::NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(taffy::NodeId::from)
    }
}

impl<'e> taffy::TraversePartialTree for Node<'e> {
    type ChildIter<'a> = ChildIter where Self: 'a;

    fn child_ids(&self, _parent_node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        ChildIter(0..self.element.child_count())
    }

    fn child_count(&self, _parent_node_id: taffy::NodeId) -> usize {
        self.element.child_count()
    }

    fn get_child_id(&self, _parent_node_id: taffy::NodeId, child_index: usize) -> taffy::NodeId {
        taffy::NodeId::from(child_index)
    }
}

impl<'e> taffy::LayoutPartialTree for Node<'e> {
    type CoreContainerStyle<'a> = &'a taffy::Style where Self: 'a;

    fn get_core_container_style(&self, node_id: taffy::NodeId) -> Self::CoreContainerStyle<'_> {
        &self.from_id(node_id).style().inner
    }

    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        self.from_id_mut(node_id).layout_cache_mut().layout = *layout;
    }

    fn resolve_calc_value(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }

    fn compute_child_layout(
        &mut self,
        id: taffy::NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput
    {
        taffy::compute_cached_layout(self, id, inputs, |parent, node_id, inputs| {
            let mut node = Node {
                element: parent.from_id_mut(id),
            };
            let style = node.element.style();
            let display_mode = style.inner.display;
            let has_children = node.element.child_count() > 0;

            match (display_mode, has_children) {
                (taffy::Display::None, _) => {
                    taffy::compute_hidden_layout(&mut node, id)
                }
                (taffy::Display::Flex, true) => {
                    taffy::compute_flexbox_layout(&mut node, node_id, inputs)
                }
                // (taffy::Display::Grid, true) => {
                //     taffy::compute_grid_layout(&mut node, node_id, inputs)
                // }
                _ => {
                    taffy::compute_leaf_layout(
                        inputs,
                        &style.inner,
                        |_value, _basis| 0.0,
                        |known_dimensions, available_space| {
                            let Some(xy) = node.element.measure(
                                Xy::new(known_dimensions.width, known_dimensions.height),
                                Xy::new(
                                    available_space.width.unwrap(),
                                    available_space.height.unwrap(),
                                ),
                            ) else {
                                return taffy::Size::ZERO;
                            };

                            taffy::Size::from_lengths(xy.x, xy.y).map(|v| v.value())
                        }
                    )
                }
            }
        })
    }
}

impl<'e> taffy::CacheTree for Node<'e> {
    fn cache_get(
        &self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.from_id(node_id).layout_cache().cache.get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: taffy::NodeId,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.from_id_mut(node_id).layout_cache_mut().cache.store(known_dimensions, available_space, run_mode, layout_output)
    }

    fn cache_clear(&mut self, node_id: taffy::NodeId) {
        self.from_id_mut(node_id).layout_cache_mut().cache.clear();
    }
}

impl<'e> taffy::LayoutFlexboxContainer for Node<'e> {
    type FlexboxContainerStyle<'a> = &'a taffy::Style where Self: 'a;
    type FlexboxItemStyle<'a> = &'a taffy::Style where Self: 'a;

    fn get_flexbox_container_style(&self, node_id: taffy::NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.from_id(node_id).style().inner
    }

    fn get_flexbox_child_style(&self, child_node_id: taffy::NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.from_id(child_node_id).style().inner
    }
}
