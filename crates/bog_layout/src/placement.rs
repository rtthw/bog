//! Placement type



use bog_math::{Rect, Vec2};

use crate::LayoutMap;



/// The placement for a node in a [`LayoutMap`].
#[derive(Clone, Copy, Debug)]
pub struct Placement<'a> {
    pub(crate) node: u64,
    pub(crate) position: Vec2,
    pub(crate) offset: Vec2,
    pub(crate) parent_rect: Rect,
    pub(crate) layout: &'a taffy::Layout,
    pub(crate) map: &'a LayoutMap,
}

impl<'a> Placement<'a> {
    /// Index of the node identified by this placement.
    pub fn node(&self) -> u64 {
        self.node
    }

    /// An iterator of placements for this node's children.
    pub fn children(&self) -> PlacementIter<'a> {
        PlacementIter {
            rect: self.rect(),
            offset: self.offset,
            children: self.map.children(self.node).iter(),
            map: &self.map,
        }
    }
}

// Positioning.
impl Placement<'_> {
    /// Get the absolute position of the node relative to the root of the layout tree.
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Get the absolute position of this node's content (position + border + padding).
    pub fn inner_position(&self) -> Vec2 {
        self.position + Vec2::new(
            self.layout.padding.left + self.layout.border.left,
            self.layout.padding.top + self.layout.border.top,
        )
    }

    /// Get the absolute position of this node's content (position + border + padding + offset).
    pub fn offset_position(&self) -> Vec2 {
        self.position + self.offset + Vec2::new(
            self.layout.padding.left + self.layout.border.left,
            self.layout.padding.top + self.layout.border.top,
        )
    }

    /// Whether this node is offset by any amount.
    pub fn has_offset(&self) -> bool {
        self.offset != Vec2::ZERO
    }

    /// Get the size allocated for this node (border + padding + content_size).
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.layout.size.width, self.layout.size.height)
    }

    /// Get the size of the area allocated for this node's content.
    /// This is often NOT the same as [`Self::content_size`].
    pub fn inner_size(&self) -> Vec2 {
        Vec2::new(self.layout.content_box_width(), self.layout.content_box_height())
    }

    /// Get the size taken up by this node's children.
    pub fn content_size(&self) -> Vec2 {
        Vec2::new(self.layout.content_size.width, self.layout.content_size.height)
    }

    /// A [`Rect`] representing ([`Self::position`], [`Self::size`]).
    pub fn rect(&self) -> Rect {
        Rect::new(self.position, self.size())
    }

    /// A [`Rect`] representing ([`Self::offset_position`], max([`Self::content_size`],
    /// [`Self::size`])).
    pub fn offset_rect(&self) -> Rect {
        Rect::new(self.offset_position(), self.content_size().max(self.size()))
    }

    /// A [`Rect`] representing ([`Self::inner_position`], [`Self::inner_size`]).
    pub fn inner_rect(&self) -> Rect {
        Rect::new(self.inner_position(), self.inner_size())
    }

    /// A [`Rect`] representing ([`Self::inner_position`], [`Self::content_size`]).
    pub fn content_rect(&self) -> Rect {
        Rect::new(self.inner_position(), self.content_size())
    }

    /// A [`Rect`] representing this node's parent.
    pub fn parent_rect(&self) -> Rect {
        self.parent_rect
    }
}

/// An iterator of placements for a node's children.
pub struct PlacementIter<'a> {
    rect: Rect,
    offset: Vec2,
    children: core::slice::Iter<'a, u64>,
    map: &'a LayoutMap,
}

impl<'a> Iterator for PlacementIter<'a> {
    type Item = Placement<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next().map(|id| {
            let location = self.map.node_info((*id).into()).layout.location;
            let offset = self.map.node_info((*id).into()).offset;
            Placement {
                node: *id,
                position: self.rect.position() + Vec2::new(location.x, location.y),
                offset: self.offset + offset,
                parent_rect: self.rect,
                layout: &self.map.node_info((*id).into()).layout,
                map: self.map,
            }
        })
    }
}
