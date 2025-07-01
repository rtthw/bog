//! User interfacing



use bog_core::{vec2, Rect, Xy};



slotmap::new_key_type! { pub struct Node; }

pub struct UserInterface {
    elements: slotmap::SlotMap<Node, ElementInfo>,
    children: slotmap::SecondaryMap<Node, Vec<Node>>,
    parents: slotmap::SecondaryMap<Node, Option<Node>>,
}

impl Default for UserInterface {
    fn default() -> Self {
        Self {
            elements: slotmap::SlotMap::with_capacity_and_key(16),
            children: slotmap::SecondaryMap::with_capacity(16),
            parents: slotmap::SecondaryMap::with_capacity(16),
        }
    }
}

impl UserInterface {
    pub fn new(root: Element, area: Rect) -> Self {
        let mut ui = Self::default();

        fn digest(
            element: Element,
            area: Rect,
            parent: Option<Node>,
            ui: &mut UserInterface,
        ) -> Node {
            let child_orientation = element.style.orient_children;
            let node = ui.elements.insert(ElementInfo {
                area,
                style: element.style,
            });
            let _ = ui.parents.insert(node, parent);

            let available = match child_orientation {
                Axis::Horizontal => area.w,
                Axis::Vertical => area.h,
            };
            let lengths = element.children.iter()
                .map(|e| match child_orientation {
                    Axis::Horizontal => e.style.sizing.x,
                    Axis::Vertical => e.style.sizing.y,
                })
                .collect::<Vec<_>>();
            let sizes = resolve_lengths(available, lengths);

            let mut length_acc = 0.0;
            let mut element_children = Vec::with_capacity(element.children.len());
            for (child, child_length) in element.children.into_iter().zip(sizes.into_iter()) {
                let child_area = match child_orientation {
                    Axis::Horizontal =>
                        Rect::new(vec2(length_acc, 0.0), vec2(child_length, area.h)),
                    Axis::Vertical =>
                        Rect::new(vec2(0.0, length_acc), vec2(area.w, child_length)),
                };
                length_acc += child_length;

                let child_node = digest(child, child_area, Some(node), ui);
                element_children.push(child_node);
            }

            let _ = ui.children.insert(node, element_children);

            node
        }

        let _root_node = digest(root, area, None, &mut ui);

        ui
    }
}



pub struct Element {
    pub style: Style,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: impl Into<Vec<Element>>) -> Self {
        self.children = children.into();
        self
    }

    pub fn with_style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }
}



pub type Sizing = Xy<Length>;

pub struct Style {
    pub sizing: Sizing,
    pub orient_children: Axis,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            sizing: Xy::new(Length::Auto, Length::Auto),
            orient_children: Axis::Vertical,
        }
    }
}



#[derive(Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub enum Length {
    Auto,
    Exact(f32),
    Portion(f32),
}

impl Length {
    pub fn is_auto(&self) -> bool {
        matches!(self, Length::Auto)
    }

    pub fn exact(&self) -> Option<f32> {
        match self {
            Length::Exact(n) => Some(*n),
            _ => None,
        }
    }

    pub fn portion(&self) -> Option<f32> {
        match self {
            Length::Portion(n) => Some(*n),
            _ => None,
        }
    }
}

fn resolve_lengths(available: f32, lengths: Vec<Length>) -> Vec<f32> {
    let mut sizes = [available / lengths.len() as f32].repeat(lengths.len());
    let mut remaining = available;
    let auto_count: usize = lengths.iter()
        .fold(0, |acc, s| if s.is_auto() { acc + 1 } else { acc });

    for (i, exact) in lengths.iter().enumerate().filter_map(|(i, s)| Some((i, s.exact()?))) {
        sizes[i] = exact;
        remaining -= exact;
    }

    for (i, portion) in lengths.iter().enumerate().filter_map(|(i, s)| Some((i, s.portion()?))) {
        let size = available * portion;
        sizes[i] = size;
        remaining -= size;
    }

    let auto_size = remaining / auto_count as f32;
    for (i, _) in lengths.iter().enumerate().filter(|(_, s)| s.is_auto()) {
        sizes[i] = auto_size;
    }

    sizes
}



// ---



struct ElementInfo {
    area: Rect,
    style: Style,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizing_resolver_works() {
        // NOTE: This is needed to account for floating point precision.
        let round_sizes = |length: f32, sizings: &[Length]| -> Vec<f32> {
            resolve_lengths(length, sizings.to_vec())
                .into_iter()
                .map(|s| (s * 10.0).round() / 10.0)
                .collect()
        };

        assert_eq!(
            round_sizes(12.0, &[Length::Auto, Length::Exact(4.0), Length::Portion(0.5)]),
            vec![2.0, 4.0, 6.0],
        );
        assert_eq!(
            round_sizes(12.0, &[Length::Auto, Length::Auto, Length::Portion(0.5)]),
            vec![3.0, 3.0, 6.0],
        );
        assert_eq!(
            round_sizes(12.0, &[Length::Auto, Length::Auto, Length::Auto]),
            vec![4.0, 4.0, 4.0],
        );
        assert_eq!(
            round_sizes(12.0, &[Length::Portion(0.4), Length::Portion(0.3), Length::Portion(0.2)]),
            vec![4.8, 3.6, 2.4],
        );
    }
}
