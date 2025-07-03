//! User interfacing



use std::collections::VecDeque;

use bog_core::{vec2, Color, InputEvent, Key, KeyCode, ModifierKey, ModifierMask, MouseButton, Rect, Vec2, Xy};



#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    Resize {
        node: Node,
    },
    Focus {
        old: Option<Node>,
        new: Node,
    },
    MouseMove {
        delta: Vec2,
    },
    MouseEnter {
        node: Node,
    },
    MouseLeave {
        node: Node,
    },
    /// The user has moved his mouse pointer over this node, and left it there long enough
    /// (default is 500 milliseconds) to trigger a hover request.
    ///
    /// This event signals an intentional request to "learn more" about the node, and you can use
    /// it to perform some hover action (e.g. show a tooltip).
    ///
    /// Only nodes with the [hover event mask](EventMask::HOVER) will trigger this event. If two
    /// nodes with this mask intersect, the topmost node will be the only one to receive this
    /// event.
    ///
    /// If you need to trigger some callback immediately after the user's mouse moves into an
    /// element, use [`Event::MouseEntered`] instead.
    Hover {
        node: Node,
    },
    MouseDown {
        node: Node,
    },
    MouseUp {
        node: Node,
    },
    /// Only nodes with the [click event mask](EventMask::CLICK) will trigger this event. If two
    /// nodes with this mask intersect, the topmost node will be the only one to receive this
    /// event.
    Click {
        node: Node,
    },
    DoubleClick {
        node: Node,
    },
    RightClick {
        node: Node,
    },
    MoveNode {
        node: Node,
        old_parent: Option<Node>,
        new_parent: Option<Node>,
    },
    DeleteNode {
        node: Node,
    },
}



#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct EventMask(u8);

bitflags::bitflags! {
    impl EventMask: u8 {
        const HOVER = 1 << 0;
        const CLICK = 1 << 1;
        const FOCUS = 1 << 2;
    }
}

impl core::fmt::Debug for EventMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EventMask {{")?;
        if self.hoverable() {
            write!(f, " hover")?;
        }
        if self.clickable() {
            write!(f, " click")?;
        }
        if self.focusable() {
            write!(f, " focus")?;
        }
        write!(f, " }}")
    }
}

impl EventMask {
    #[inline]
    pub fn hoverable(&self) -> bool {
        self.contains(Self::HOVER)
    }

    #[inline]
    pub fn clickable(&self) -> bool {
        self.contains(Self::CLICK)
    }

    #[inline]
    pub fn focusable(&self) -> bool {
        self.contains(Self::FOCUS)
    }
}



slotmap::new_key_type! { pub struct Node; }

pub struct UserInterface {
    root: Node,
    elements: slotmap::SlotMap<Node, ElementInfo>,
    children: slotmap::SecondaryMap<Node, Vec<Node>>,
    parents: slotmap::SecondaryMap<Node, Option<Node>>,

    events: VecDeque<Event>,

    mouse_pos: Vec2,
    mouse_over: Vec<Node>,
    key_modifiers: ModifierMask,
    focused: Option<Node>,
}

impl UserInterface {
    pub fn new(root: Element, area: Rect) -> Self {
        let mut elements = slotmap::SlotMap::with_capacity_and_key(16);
        let mut children = slotmap::SecondaryMap::with_capacity(16);
        let mut parents = slotmap::SecondaryMap::with_capacity(16);

        fn digest(
            element: Element,
            area: Rect,
            parent: Option<Node>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo>,
            children: &mut slotmap::SecondaryMap<Node, Vec<Node>>,
            parents: &mut slotmap::SecondaryMap<Node, Option<Node>>,
        ) -> Node {
            let child_orientation = element.style.orient_children;
            let node = elements.insert(ElementInfo {
                area,
                event_mask: element.event_mask,
                style: element.style,
            });
            let _ = parents.insert(node, parent);

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

                let child_node = digest(child, child_area, Some(node), elements, children, parents);
                element_children.push(child_node);
            }

            let _ = children.insert(node, element_children);

            node
        }

        let root_node = digest(root, area, None, &mut elements, &mut children, &mut parents);

        Self {
            root: root_node,
            elements,
            children,
            parents,

            events: VecDeque::new(),

            mouse_pos: Vec2::ZERO,
            mouse_over: Vec::new(),
            key_modifiers: ModifierMask::empty(),
            focused: None,
        }
    }

    pub fn crawl(&self, func: &mut impl FnMut(&UserInterface, Node)) {
        fn inner(
            ui: &UserInterface,
            node: Node,
            func: &mut impl FnMut(&UserInterface, Node),
        ) {
            func(ui, node);
            for child in ui.children[node].clone() {
                inner(ui, child, func);
            }
        }

        inner(self, self.root, func);
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn update_style(&mut self, node: Node, func: impl FnOnce(&mut Style)) {
        func(&mut self.elements[node].style)
    }

    pub fn bounds(&self, node: Node) -> Rect {
        self.elements[node].area
    }

    pub fn style(&self, node: Node) -> &Style {
        &self.elements[node].style
    }
}

impl UserInterface {
    pub fn handle_input(&mut self, event: InputEvent) {
        match event {
            InputEvent::Resize { width, height } => {
                self.handle_resize(vec2(width as _, height as _));
            }
            InputEvent::MouseMove { x, y } => {
                self.handle_mouse_move(vec2(x, y));
            }
            InputEvent::MouseEnter => {
                // Handled by mouse move.
            }
            InputEvent::MouseLeave => {
                self.events.extend(self.mouse_over.drain(..)
                    .map(|node| Event::MouseLeave { node }));
            }
            InputEvent::MouseDown { button } => {
                self.handle_mouse_down(button);
            }
            InputEvent::MouseUp { button } => {
                self.handle_mouse_up(button);
            }
            InputEvent::KeyDown { code, repeat } => {
                self.handle_key_down(code, repeat);
            }
            InputEvent::KeyUp { code } => {
                self.handle_key_up(code);
            }
            _ => {} // TODO
        }
    }

    pub fn handle_resize(&mut self, size: Vec2) {
        let area = Rect::new(Vec2::ZERO, size);

        // FIXME: Maybe don't early return here? (Only saves one allocation?)
        if self.elements[self.root].area == area {
            return;
        }

        fn inner(
            node: Node,
            area: Rect,
            events: &mut VecDeque<Event>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo>,
            children: &mut slotmap::SecondaryMap<Node, Vec<Node>>,
        ) {
            if elements[node].area == area {
                return;
            }
            elements[node].area = area;

            events.push_back(Event::Resize { node });

            let child_orientation = elements[node].style.orient_children;
            let available = match child_orientation {
                Axis::Horizontal => area.w,
                Axis::Vertical => area.h,
            };
            let lengths = children[node].iter()
                .map(|n| {
                    match child_orientation {
                        Axis::Horizontal => elements[*n].style.sizing.x,
                        Axis::Vertical => elements[*n].style.sizing.y,
                    }
                })
                .collect::<Vec<_>>();
            let sizes = resolve_lengths(available, lengths);

            let mut length_acc = 0.0;
            for (child_node, child_length) in children[node].clone().into_iter()
                .zip(sizes.into_iter())
            {
                let child_area = match child_orientation {
                    Axis::Horizontal => Rect::new(
                        vec2(area.x + length_acc, area.y),
                        vec2(child_length, area.h),
                    ),
                    Axis::Vertical => Rect::new(
                        vec2(area.x, area.y + length_acc),
                        vec2(area.w, child_length),
                    ),
                };
                length_acc += child_length;

                inner(child_node, child_area, events, elements, children);
            }
        }

        inner(self.root, area, &mut self.events, &mut self.elements, &mut self.children);
    }

    pub fn handle_mouse_move(&mut self, position: Vec2) {
        if self.mouse_pos == position {
            return;
        }

        let delta = position - self.mouse_pos;
        // TODO: Setting for not reporting mouse movements to avoid clogging event queue?
        self.events.push_back(Event::MouseMove { delta });

        fn inner(
            node: Node,
            position: Vec2,
            mouse_over: &mut Vec<Node>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo>,
            children: &mut slotmap::SecondaryMap<Node, Vec<Node>>,
        ) {
            if !elements[node].area.contains(position) {
                return;
            }
            mouse_over.push(node);
            for child in children[node].clone() {
                inner(child, position, mouse_over, elements, children);
            }
        }

        let mut new_mouse_over = Vec::new();
        inner(self.root, position, &mut new_mouse_over, &mut self.elements, &mut self.children);

        if self.mouse_over != new_mouse_over {
            for node in &self.mouse_over {
                if !new_mouse_over.contains(node) {
                    self.events.push_back(Event::MouseLeave { node: *node });
                }
            }
            for node in &new_mouse_over {
                if !self.mouse_over.contains(node) {
                    self.events.push_back(Event::MouseEnter { node: *node });
                }
            }
            self.mouse_over = new_mouse_over;
        }
    }

    // TODO: Settings for act on press/release and double click timing.
    pub fn handle_mouse_down(&mut self, button: MouseButton) {
        if self.mouse_over.is_empty() {
            return;
        }

        if let Some(node) = self.mouse_over.iter()
            .rev()
            .find(|node| self.elements[**node].event_mask.clickable())
        {
            match button {
                MouseButton::Left => self.events.push_back(Event::Click { node: *node }),
                MouseButton::Right => self.events.push_back(Event::RightClick { node: *node }),
                _ => {}
            }
        }
        if let Some(node) = self.mouse_over.iter()
            .rev()
            .find(|node| self.elements[**node].event_mask.focusable())
        {
            match button {
                MouseButton::Left => {
                    let old = self.focused.take();
                    self.focused = Some(*node);

                    self.events.push_back(Event::Focus { old, new: *node });
                }
                _ => {}
            }
        }
    }

    pub fn handle_mouse_up(&mut self, _button: MouseButton) {
        // TODO
    }

    pub fn handle_key_down(&mut self, code: KeyCode, _repeat: bool) {
        match Key::from((code, self.key_modifiers.has_shift())) {
            Key::Modifier(mod_key) => {
                match mod_key {
                    ModifierKey::Control => {
                        self.key_modifiers.insert(ModifierMask::CTRL);
                    }
                    ModifierKey::Shift => {
                        self.key_modifiers.insert(ModifierMask::SHIFT);
                    }
                    ModifierKey::Alt => {
                        self.key_modifiers.insert(ModifierMask::ALT);
                    }
                    ModifierKey::Super => {
                        self.key_modifiers.insert(ModifierMask::SUPER);
                    }
                }
            }
            Key::Char(_ch) => {
                // TODO: Handle input, keybinds, etc.
            }
            Key::Left => {
                // self.focus_left()
            }
            Key::Right => {
                // self.focus_right()
            }
            Key::Up => {
                // self.focus_up()
            }
            Key::Down => {
                // self.focus_down()
            }
            _ => {}
        }
    }

    pub fn handle_key_up(&mut self, code: KeyCode) {
        match Key::from((code, self.key_modifiers.has_shift())) {
            Key::Modifier(mod_key) => {
                match mod_key {
                    ModifierKey::Control => {
                        self.key_modifiers.remove(ModifierMask::CTRL);
                    }
                    ModifierKey::Shift => {
                        self.key_modifiers.remove(ModifierMask::SHIFT);
                    }
                    ModifierKey::Alt => {
                        self.key_modifiers.remove(ModifierMask::ALT);
                    }
                    ModifierKey::Super => {
                        self.key_modifiers.remove(ModifierMask::SUPER);
                    }
                }
            }
            _ => {}
        }
    }
}

impl UserInterface {
    pub fn insert(&mut self, parent: Option<Node>, child: Node) {
        let old_parent = self.parents[child];
        if old_parent == parent {
            return;
        }
        if let Some(old_parent) = old_parent {
            self.children[old_parent].retain(|node| node != &child);
        }
        if let Some(parent) = parent {
            self.children[parent].push(child);
        }
        self.parents[child] = parent;

        self.events.push_back(Event::MoveNode {
            node: child,
            old_parent,
            new_parent: parent,
        });
    }

    pub fn delete(&mut self, node: Node) {
        self.events.push_back(Event::DeleteNode { node }); // TODO
    }
}



pub struct Element {
    pub style: Style,
    pub event_mask: EventMask,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            event_mask: EventMask::empty(),
            children: Vec::new(),
        }
    }

    pub fn with_style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn with_event_mask(mut self, event_mask: EventMask) -> Self {
        self.event_mask = event_mask;
        self
    }

    pub fn with_children(mut self, children: impl Into<Vec<Element>>) -> Self {
        self.children = children.into();
        self
    }
}



pub type Sizing = Xy<Length>;

pub struct Style {
    pub sizing: Sizing,
    pub orient_children: Axis,
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            sizing: Xy::new(Length::Auto, Length::Auto),
            orient_children: Axis::Vertical,
            background_color: Color::NONE,
            border_color: Color::NONE,
            border_width: 0.0,
        }
    }
}

impl Style {
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.orient_children = Axis::Horizontal;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orient_children = Axis::Vertical;
        self
    }

    pub fn width(mut self, length: Length) -> Self {
        self.sizing.x = length;
        self
    }

    pub fn height(mut self, length: Length) -> Self {
        self.sizing.y = length;
        self
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
    event_mask: EventMask,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut ui = UserInterface::new(Element::new(), Rect::NONE);
        ui.handle_input(InputEvent::MouseMove { x: 1.0, y: 1.0 });
        let mut event_num = 0;
        while let Some(event) = ui.next_event() {
            match event {
                Event::MouseMove { .. } => {
                    ui.delete(ui.root);
                    assert!(event_num == 0);
                    event_num += 1;
                }
                Event::DeleteNode { .. } => {
                    assert!(event_num == 1);
                    event_num += 1;
                }
                _ => {}
            }
        }
        assert!(event_num == 2);
    }

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
