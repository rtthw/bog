//! User interfacing



use std::collections::VecDeque;

use bog_core::{vec2, Color, ControlKey, InputEvent, Key, KeyCode, ModifierKey, ModifierMask, MouseButton, Rect, Vec2};



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
    /// A [`ControlKey`] was pressed.
    ControlKeyPress {
        /// The [`ControlKey`] that was pressed.
        key: ControlKey,
        /// A repeated key press event due to the user having the key held for long enough to
        /// trigger a repeat event.
        repeat: bool,
    },
    CharInput {
        /// The [`char`] that was pressed.
        ch: char,
        /// A repeated character due to the user having the key held for long enough to trigger a
        /// repeat event.
        repeat: bool,
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

pub struct UserInterface<T = ()> {
    root: Node,
    elements: slotmap::SlotMap<Node, ElementInfo<T>>,
    children: slotmap::SecondaryMap<Node, Vec<Node>>,
    parents: slotmap::SecondaryMap<Node, Option<Node>>,

    events: VecDeque<Event>,

    mouse_pos: Vec2,
    mouse_over: Vec<Node>,
    key_modifiers: ModifierMask,
    focused: Option<Node>,
}

// Core.
impl<T> UserInterface<T> {
    pub fn new(root: Element<T>, area: Rect) -> Self {
        let mut elements = slotmap::SlotMap::with_capacity_and_key(16);
        let mut children = slotmap::SecondaryMap::with_capacity(16);
        let mut parents = slotmap::SecondaryMap::with_capacity(16);

        fn digest<T>(
            element: Element<T>,
            area: Rect,
            parent: Option<Node>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo<T>>,
            children: &mut slotmap::SecondaryMap<Node, Vec<Node>>,
            parents: &mut slotmap::SecondaryMap<Node, Option<Node>>,
        ) -> Node {
            let child_orientation = element.style.orient_children;
            let node = elements.insert(ElementInfo {
                data: element.data,
                area,
                event_mask: element.event_mask,
                style: element.style,
            });
            let _ = parents.insert(node, parent);

            let child_areas = resolve_layout(
                area,
                child_orientation,
                element.children.iter().map(|c| Sizing::from(&c.style)).collect(),
            );

            let mut element_children = Vec::with_capacity(element.children.len());
            for (child, child_area) in element.children.into_iter().zip(child_areas.into_iter()) {
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

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

// Iterators, Accessors, & Mutators.
impl<T> UserInterface<T> {
    /// Apply the provided function to each element node in descending (back to front) order.
    pub fn crawl(&self, func: &mut impl FnMut(&UserInterface<T>, Node)) {
        fn inner<T>(
            ui: &UserInterface<T>,
            node: Node,
            func: &mut impl FnMut(&UserInterface<T>, Node),
        ) {
            func(ui, node);
            for child in ui.children[node].clone() {
                inner(ui, child, func);
            }
        }

        inner(self, self.root, func);
    }

    /// Get a reference to the node's associated custom data.
    pub fn data(&self, node: Node) -> &T {
        &self.elements[node].data
    }

    /// Get a mutable reference to the node's associated custom data.
    pub fn data_mut(&mut self, node: Node) -> &mut T {
        &mut self.elements[node].data
    }

    /// Get the current bounds of the node.
    pub fn bounds(&self, node: Node) -> Rect {
        self.elements[node].area
    }

    /// Get a reference to the node's [`Style`].
    pub fn style(&self, node: Node) -> &Style {
        &self.elements[node].style
    }

    /// Get a mutable reference to the node's [`Style`].
    pub fn style_mut(&mut self, node: Node) -> &mut Style {
        &mut self.elements[node].style
    }

    /// Apply the provided function to the node's [`Style`].
    pub fn update_style(&mut self, node: Node, func: impl FnOnce(&mut Style)) {
        func(&mut self.elements[node].style)
    }

    /// Add `child` to `parent`.
    ///
    /// If `None` is provided for `parent`, remove the child from the root tree. This will not free
    /// the node from memory. Use [`Self::delete`] to completely erase a node from the tree.
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

    /// Delete `node` from this UI.
    pub fn delete(&mut self, node: Node) {
        self.events.push_back(Event::DeleteNode { node }); // TODO
    }

    /// Get the position of the node **without** its margin, border, or padding offsets.
    ///
    /// See [`Self::border_position`], [`Self::inner_position`], [`Self::content_position`].
    #[inline]
    pub fn absolute_position(&self, node: Node) -> Vec2 {
        self.elements[node].absolute_position()
    }

    /// Get the position of the node relative to its margin (`absolute_position` + `margin_offset`).
    ///
    /// See [`Self::absolute_position`], [`Self::inner_position`], [`Self::content_position`].
    #[inline]
    pub fn border_position(&self, node: Node) -> Vec2 {
        self.elements[node].border_position()
    }

    /// Get the position of the node relative to its margin and border (`absolute_position` +
    /// `margin_offset` + `border_offset`).
    ///
    /// See [`Self::absolute_position`], [`Self::border_position`], [`Self::content_position`].
    #[inline]
    pub fn inner_position(&self, node: Node) -> Vec2 {
        self.elements[node].inner_position()
    }

    /// Get the position of the node relative to its margin, border, and padding
    /// (`absolute_position` + `margin_offset` + `border_offset` + `padding_offset`).
    ///
    /// See [`Self::absolute_pos`], [`Self::border_position`], [`Self::inner_position`].
    #[inline]
    pub fn content_position(&self, node: Node) -> Vec2 {
        self.elements[node].content_position()
    }
}

// Handlers.
impl<T> UserInterface<T> {
    /// Handle the given [`InputEvent`], mutating the UI's internal state.
    ///
    /// Be sure to drain the event queue with [`Self::next_event`] immediately after calling this
    /// function to respond to changes in this UI.
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
        let area = Rect::at_origin(size);

        // FIXME: Maybe don't early return here? (Only saves one allocation?)
        if self.elements[self.root].area == area {
            return;
        }

        fn inner<T>(
            node: Node,
            area: Rect,
            events: &mut VecDeque<Event>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo<T>>,
            children: &mut slotmap::SecondaryMap<Node, Vec<Node>>,
        ) {
            if elements[node].area == area {
                return;
            }
            elements[node].area = area;

            events.push_back(Event::Resize { node });

            let child_areas = resolve_layout(
                area,
                elements[node].style.orient_children,
                children[node].iter().map(|c| Sizing::from(&elements[*c].style)).collect(),
            );

            for (child_node, child_area) in children[node].clone().into_iter()
                .zip(child_areas.into_iter())
            {
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

        fn inner<T>(
            node: Node,
            position: Vec2,
            mouse_over: &mut Vec<Node>,
            elements: &mut slotmap::SlotMap<Node, ElementInfo<T>>,
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

    pub fn handle_key_down(&mut self, code: KeyCode, repeat: bool) {
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
            Key::Char(ch) => {
                // TODO: Keybinds?
                self.events.push_back(Event::CharInput { ch, repeat });
            }
            Key::Control(key) => {
                self.events.push_back(Event::ControlKeyPress { key, repeat });
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



pub struct Element<T> {
    pub data: T,
    pub style: Style,
    pub event_mask: EventMask,
    pub children: Vec<Element<T>>,
}

impl<T> Element<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            style: Style::default(),
            event_mask: EventMask::empty(),
            children: Vec::new(),
        }
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = data;
        self
    }

    pub fn with_style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn with_event_mask(mut self, event_mask: EventMask) -> Self {
        self.event_mask = event_mask;
        self
    }

    pub fn with_children(mut self, children: impl Into<Vec<Element<T>>>) -> Self {
        self.children = children.into();
        self
    }
}



pub struct Style {
    pub sizing: [Length; 2],
    pub orient_children: Axis,
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub padding: Edges,
    pub margin: Edges,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            sizing: [Length::Auto; 2],
            orient_children: Axis::Vertical,
            background_color: Color::NONE,
            border_color: Color::NONE,
            border_width: 0.0,
            padding: Edges::default(),
            margin: Edges::default(),
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

    pub fn padding(mut self, size: f32) -> Self {
        self.padding = Edges::all(size);
        self
    }

    pub fn padding2(mut self, left_right: f32, top_bottom: f32) -> Self {
        self.padding = Edges::two_value(left_right, top_bottom);
        self
    }

    pub fn margin(mut self, size: f32) -> Self {
        self.margin = Edges::all(size);
        self
    }

    pub fn margin2(mut self, left_right: f32, top_bottom: f32) -> Self {
        self.margin = Edges::two_value(left_right, top_bottom);
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
        self.sizing[0] = length;
        self
    }

    pub fn height(mut self, length: Length) -> Self {
        self.sizing[1] = length;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Edges {
    #[inline]
    pub const fn all(size: f32) -> Self {
        Self { left: size, right: size, top: size, bottom: size }
    }

    #[inline]
    pub const fn two_value(left_right: f32, top_bottom: f32) -> Self {
        Self { left: left_right, right: left_right, top: top_bottom, bottom: top_bottom }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Length {
    #[default]
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

#[derive(Clone, Debug, Default)]
struct Sizing {
    width: Length,
    height: Length,
    // NOTE: For now, we don't need to know about padding/margins/borders until after the initial
    //       size resolution.
}

impl From<&Style> for Sizing {
    fn from(value: &Style) -> Self {
        Self {
            width: value.sizing[0],
            height: value.sizing[1],
            // padding: value.padding,
            // margin: value.margin,
        }
    }
}

fn resolve_layout(available: Rect, axis: Axis, sizings: Vec<Sizing>) -> Vec<Rect> {
    let (main_axis_length, lengths): (f32, Vec<Length>) = match axis {
        Axis::Horizontal => (available.w, sizings.iter().map(|s| s.width).collect()),
        Axis::Vertical => (available.h, sizings.iter().map(|s| s.height).collect()),
    };

    let mut sizes = [0.0].repeat(lengths.len());
    let mut remaining = main_axis_length;
    let auto_count: usize = lengths.iter()
        .fold(0, |acc, len| if len.is_auto() { acc + 1 } else { acc });

    for (i, exact) in lengths.iter().enumerate().filter_map(|(i, s)| Some((i, s.exact()?))) {
        sizes[i] = exact;
        remaining -= exact;
    }

    for (i, portion) in lengths.iter().enumerate().filter_map(|(i, s)| Some((i, s.portion()?))) {
        let size = main_axis_length * portion;
        sizes[i] = size;
        remaining -= size;
    }

    let auto_size = remaining / auto_count as f32;
    for (i, _) in lengths.iter().enumerate().filter(|(_, s)| s.is_auto()) {
        sizes[i] = auto_size;
    }

    let mut size_acc = 0.0;
    sizes.into_iter()
        .map(|size| {
            let rect = match axis {
                Axis::Horizontal => Rect::new(
                    vec2(size_acc + available.x, available.y),
                    vec2(size, available.h),
                ),
                Axis::Vertical => Rect::new(
                    vec2(available.x, size_acc + available.y),
                    vec2(available.w, size),
                ),
            };
            size_acc += size;

            rect
        })
        .collect()
}



// ---



struct ElementInfo<T> {
    data: T,
    area: Rect,
    style: Style,
    event_mask: EventMask,
}

impl<T> ElementInfo<T> {
    #[inline]
    fn absolute_position(&self) -> Vec2 {
        self.area.position()
    }

    #[inline]
    fn border_position(&self) -> Vec2 {
        self.area.position()
            + vec2(self.style.margin.left, self.style.margin.top)
    }

    #[inline]
    fn inner_position(&self) -> Vec2 {
        self.area.position()
            + vec2(self.style.margin.left, self.style.margin.top)
            + vec2(self.style.border_width, self.style.border_width) // TODO: Use `Edges` here?
    }

    #[inline]
    fn content_position(&self) -> Vec2 {
        self.area.position()
            + vec2(self.style.margin.left, self.style.margin.top)
            + vec2(self.style.border_width, self.style.border_width) // TODO: Use `Edges` here?
            + vec2(self.style.padding.left, self.style.padding.top)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut ui = UserInterface::new(Element::new(()), Rect::NONE);
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
    fn layout_resolver_works() {
        let round_sizes = |length: f32, sizings: &[Length]| -> Vec<f32> {
            resolve_layout(
                Rect::at_origin(vec2(length, 0.0)),
                Axis::Horizontal,
                sizings.into_iter()
                    .map(|s| Sizing {
                        width: *s,
                        height: Length::Auto,
                        ..Default::default()
                    })
                    .collect(),
            )
                .into_iter()
                // NOTE: This is needed to account for floating point precision.
                .map(|r| r.with_size(vec2(
                    (r.size().x * 10.0).round() / 10.0,
                    (r.size().y * 10.0).round() / 10.0,
                )))
                .map(|r| r.size().x)
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
