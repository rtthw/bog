//! Input handling



use std::time::Instant;

use crate::{key::ModifierKey, vec2, InputEvent, Key, KeyCode, MouseButton, Rect, Vec2, WheelMovement};



pub struct EventParser {
    key: KeyEventParser,
    mouse: MouseEventParser,
}

impl EventParser {
    pub fn new(root_area: InputArea) -> Self {
        Self {
            key: KeyEventParser::default(),
            mouse: MouseEventParser::new(root_area),
        }
    }

    pub fn parse_event(&mut self, event: InputEvent) -> Vec<Input> {
        match event {
            InputEvent::Resize { width, height } => {
                vec![Input::Resize {
                    new_size: vec2(width as _, height as _),
                }]
            }
            InputEvent::FocusIn => {
                vec![Input::Focus { focus: true }]
            }
            InputEvent::FocusOut => {
                vec![Input::Focus { focus: false }]
            }
            InputEvent::KeyDown { code, repeat } => {
                self.key.handle_key_down(code, repeat).into_iter().map(Input::Key).collect()
            }
            InputEvent::KeyUp { code } => {
                self.key.handle_key_up(code).into_iter().map(Input::Key).collect()
            }
            InputEvent::MouseMove { x, y } => {
                self.mouse.handle_mouse_move(vec2(x, y)).into_iter().map(Input::Mouse).collect()
            }
            InputEvent::MouseDown { button } => {
                self.mouse.handle_mouse_down(button).into_iter().map(Input::Mouse).collect()
            }
            InputEvent::MouseUp { button } => {
                self.mouse.handle_mouse_up(button).into_iter().map(Input::Mouse).collect()
            }
            InputEvent::WheelMove(movement) => {
                self.mouse.handle_wheel_move(movement).into_iter().map(Input::Mouse).collect()
            }
            InputEvent::MouseEnter => { Vec::new() } // TODO
            InputEvent::MouseLeave => { Vec::new() } // TODO
        }
    }

    /// Get a reference to the root [`InputArea`].
    pub fn root_area(&self) -> &InputArea {
        &self.mouse.root_area
    }

    /// Get a mutable reference to the root [`InputArea`].
    pub fn root_area_mut(&mut self) -> &mut InputArea {
        &mut self.mouse.root_area
    }

    pub fn update_areas(&mut self, new_root_area: InputArea) -> Vec<Input> {
        self.mouse.update_areas(new_root_area).into_iter().map(Input::Mouse).collect()
    }

    pub fn for_each_area(&self, func: &mut impl FnMut(&InputArea)) {
        self.mouse.root_area.crawl(func);
    }

    pub fn for_each_area_mut(&mut self, func: &mut impl FnMut(&mut InputArea)) {
        self.mouse.root_area.crawl_mut(func);
    }

    #[inline]
    pub const fn is_control_key_down(&self) -> bool {
        self.key.control_down
    }

    #[inline]
    pub const fn is_shift_key_down(&self) -> bool {
        self.key.shift_down
    }

    #[inline]
    pub const fn is_alt_key_down(&self) -> bool {
        self.key.alt_down
    }

    #[inline]
    pub const fn is_super_key_down(&self) -> bool {
        self.key.super_down
    }
}

/// Translates raw key [`InputEvent`]s into more intuitive [`KeyInput`]s.
#[derive(Debug, Default)]
pub struct KeyEventParser {
    control_down: bool,
    shift_down: bool,
    alt_down: bool,
    super_down: bool,
    codes_down: rustc_hash::FxHashSet<KeyCode>,
}

impl KeyEventParser {
    pub fn handle_key_down(&mut self, code: KeyCode, repeat: bool) -> Vec<KeyInput> {
        let key = Key::from((code, self.shift_down));
        match key {
            Key::Modifier(ModifierKey::Control) => { self.control_down = true; }
            Key::Modifier(ModifierKey::Shift) => { self.shift_down = true; }
            Key::Modifier(ModifierKey::Alt) => { self.alt_down = true; }
            Key::Modifier(ModifierKey::Super) => { self.super_down = true; }
            _ => {}
        }
        // let repeat = !self.codes_down.insert(code) || repeat;
        let _ = self.codes_down.insert(code);

        vec![KeyInput::Press { key, repeat }]
    }

    pub fn handle_key_up(&mut self, code: KeyCode) -> Vec<KeyInput> {
        let key = Key::from((code, self.shift_down));
        match key {
            Key::Modifier(ModifierKey::Control) => { self.control_down = false; }
            Key::Modifier(ModifierKey::Shift) => { self.shift_down = false; }
            Key::Modifier(ModifierKey::Alt) => { self.alt_down = false; }
            Key::Modifier(ModifierKey::Super) => { self.super_down = false; }
            _ => {}
        }
        let valid = self.codes_down.remove(&code);

        vec![KeyInput::Release{ key, valid }]
    }
}

/// Translates raw mouse [`InputEvent`]s into more intuitive [`MouseInput`]s.
pub struct MouseEventParser {
    root_area: InputArea,
    mouse_pos: Vec2,
    hovered: Vec<&'static str>,
    buttons_down: MouseButtonMask,
    is_dragging: bool,
    drag_start_pos: Option<Vec2>,
    drag_start_area: Option<&'static str>,
    drag_start_time: Option<Instant>,
}

impl MouseEventParser {
    pub fn new(root_area: InputArea) -> Self {
        Self {
            root_area,
            mouse_pos: Vec2::ZERO,
            hovered: Vec::new(),
            buttons_down: MouseButtonMask::empty(),
            is_dragging: false,
            drag_start_pos: None,
            drag_start_area: None,
            drag_start_time: None,
        }
    }

    pub fn handle_mouse_move(&mut self, new_position: Vec2) -> Vec<MouseInput> {
        let move_delta = new_position - self.mouse_pos;
        self.mouse_pos = new_position;
        let mut inputs = vec![MouseInput::Movement { delta: move_delta }];

        let new_hovered = self.root_area.list_under(new_position);
        if self.hovered != new_hovered {
            for area in &self.hovered {
                if !new_hovered.contains(area) {
                    inputs.push(MouseInput::Leave { area });
                }
            }
            for area in &new_hovered {
                if !self.hovered.contains(area) {
                    inputs.push(MouseInput::Enter { area });
                }
            }
            self.hovered = new_hovered;
        }
        // TODO: Dragging with other mouse buttons.
        if let Some(topmost_hovered_area) = self.hovered.last() {
            if self.buttons_down.left() {
                if !self.is_dragging {
                    self.is_dragging = true;
                    self.drag_start_pos = Some(self.mouse_pos);
                    self.drag_start_area = Some(*topmost_hovered_area);
                    inputs.push(MouseInput::DragStart {
                        pos: self.mouse_pos,
                        area: *topmost_hovered_area,
                    });
                    // if let Some(start_time) = self.drag_start_time {
                    //     let dur_since = Instant::now().duration_since(start_time);
                    //     if dur_since.as_secs_f64() > 0.1 {
                    //     }
                    // }
                }
                // NOTE: We check twice here because it could change in the first check.
                if let Some(start_area) = &self.drag_start_area {
                    inputs.push(MouseInput::DragMove {
                        delta: move_delta,
                        area: *start_area,
                    });
                }
            }
        }

        inputs
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) -> Vec<MouseInput> {
        match button {
            MouseButton::Left => {
                self.drag_start_time = Some(Instant::now());
                self.buttons_down.insert(MouseButtonMask::LEFT);
            }
            MouseButton::Right => self.buttons_down.insert(MouseButtonMask::RIGHT),
            MouseButton::Middle => self.buttons_down.insert(MouseButtonMask::MIDDLE),
            _ => {}
        }

        self.hovered.clone()
            .into_iter()
            .map(|name| MouseInput::Press { area: name, button })
            .collect()
    }

    pub fn handle_mouse_up(&mut self, button: MouseButton) -> Vec<MouseInput> {
        let mut inputs: Vec<MouseInput> = self.hovered.clone()
            .into_iter()
            .map(|name| MouseInput::Release { area: name, button })
            .collect();

        match button {
            MouseButton::Left => {
                // FIXME: I'm not a fan of this indentation.
                if self.is_dragging {
                    self.is_dragging = false;
                    self.drag_start_time = None;
                    if let Some(start_pos) = self.drag_start_pos.take() {
                        if let Some(start_area) = self.drag_start_area.take() {
                            inputs.push(MouseInput::Drag {
                                start_pos,
                                end_pos: self.mouse_pos,
                                start_area,
                                end_area: self.hovered.last().map(|name| *name),
                            });
                        }
                    }
                }
                self.buttons_down.remove(MouseButtonMask::LEFT);
            }
            MouseButton::Right => self.buttons_down.remove(MouseButtonMask::RIGHT),
            MouseButton::Middle => self.buttons_down.remove(MouseButtonMask::MIDDLE),
            _ => {}
        }

        inputs
    }

    pub fn handle_wheel_move(&mut self, movement: WheelMovement) -> Vec<MouseInput> {
        match movement {
            WheelMovement::Lines { y, .. } => { // TODO: Horizontal mouse scrolling.
                if y.is_sign_negative() {
                    vec![MouseInput::ScrollDown { lines: y.abs() }]
                } else {
                    vec![MouseInput::ScrollUp { lines: y }]
                }
            }
            WheelMovement::Pixels { x, y } => {
                vec![MouseInput::Pan { delta: vec2(x, y) }]
            }
        }
    }

    pub fn update_areas(&mut self, new_root_area: InputArea) -> Vec<MouseInput> {
        self.root_area = new_root_area;
        let mut inputs = vec![];
        let new_hovered = self.root_area.list_under(self.mouse_pos);
        if self.hovered != new_hovered {
            for area in &self.hovered {
                if !new_hovered.contains(area) {
                    inputs.push(MouseInput::Leave { area });
                }
            }
            for area in &new_hovered {
                if !self.hovered.contains(area) {
                    inputs.push(MouseInput::Enter { area });
                }
            }
            self.hovered = new_hovered;
        }

        inputs
    }
}



#[derive(Clone, Debug)]
pub enum Input {
    /// A key input occurred.
    Key(KeyInput),
    /// A mouse input occurred.
    Mouse(MouseInput),
    /// The root area was resized to `new_size`.
    Resize {
        new_size: Vec2,
    },
    /// The root area gained/lost focus.
    Focus {
        focus: bool,
    },
}

#[derive(Clone, Debug)]
pub enum KeyInput {
    /// A key down event.
    Press {
        key: Key,
        repeat: bool,
    },
    /// A key up event.
    Release{
        key: Key,
        valid: bool,
    },
    /// A valid key up event after a valid key down event.
    FullPress(Key),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MouseInput {
    /// The user's mouse position changed.
    Movement {
        /// The change from the last known mouse position to the current one.
        delta: Vec2,
    },
    /// The user's mouse just entered this area.
    Enter {
        area: &'static str,
    },
    /// The user's mouse just left this area.
    Leave {
        area: &'static str,
    },
    /// Some mouse button was just pressed down over this area.
    Press {
        area: &'static str,
        button: MouseButton,
    },
    /// Some mouse button was just released over this area.
    Release {
        area: &'static str,
        button: MouseButton,
    },
    Drag {
        start_pos: Vec2,
        end_pos: Vec2,
        start_area: &'static str,
        end_area: Option<&'static str>,
    },
    DragStart {
        pos: Vec2,
        area: &'static str,
    },
    DragMove {
        delta: Vec2,
        area: &'static str,
    },
    ScrollUp {
        lines: f32,
    },
    ScrollDown {
        lines: f32,
    },
    Pan {
        delta: Vec2,
    },
}



#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct MouseButtonMask(u8);

bitflags::bitflags! {
    impl MouseButtonMask: u8 {
        /// The left mouse button (LMB).
        const LEFT = 1 << 0;
        /// The right mouse button (RMB).
        const RIGHT = 1 << 1;
        /// The middle mouse button (MMB).
        const MIDDLE = 1 << 2;
    }
}

impl core::fmt::Debug for MouseButtonMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MouseButtonMask {{")?;
        if self.left() {
            write!(f, " left")?;
        }
        if self.right() {
            write!(f, " right")?;
        }
        if self.middle() {
            write!(f, " middle")?;
        }
        write!(f, " }}")
    }
}

impl MouseButtonMask {
    #[inline]
    pub fn left(&self) -> bool {
        self.contains(Self::LEFT)
    }

    #[inline]
    pub fn right(&self) -> bool {
        self.contains(Self::RIGHT)
    }

    #[inline]
    pub fn middle(&self) -> bool {
        self.contains(Self::MIDDLE)
    }
}



// ---



pub struct InputArea {
    rect: Rect,
    pub name: &'static str,
    children: Vec<InputArea>,
}

// Builder.
impl InputArea {
    pub fn new(rect: Rect, name: &'static str) -> Self {
        Self {
            rect,
            name,
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<InputArea>) -> Self {
        self.children = children;
        self
    }
}

impl InputArea {
    #[inline]
    pub const fn rect(&self) -> Rect {
        self.rect
    }

    #[inline]
    pub fn children(&self) -> &[InputArea] {
        &self.children
    }

    /// Get the names of all areas underneath the provided point.
    pub fn list_under(&self, point: Vec2) -> Vec<&'static str> {
        if !self.rect.contains(point) {
            return vec![];
        }

        fn inner(
            current: &InputArea,
            list: &mut Vec<&'static str>,
            point: Vec2,
        ) {
            for child_area in current.children.iter() {
                if !child_area.rect.contains(point) {
                    continue;
                }
                list.push(child_area.name);
                inner(child_area, list, point);
            }
        }

        let mut list = vec![self.name];
        inner(self, &mut list, point);

        list
    }

    fn crawl(&self, func: &mut impl FnMut(&InputArea)) {
        func(self);
        for child in self.children.iter() {
            child.crawl(func);
        }
    }

    fn crawl_mut(&mut self, func: &mut impl FnMut(&mut InputArea)) {
        func(self);
        for child in self.children.iter_mut() {
            child.crawl_mut(func);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::vec2;

    use super::*;

    #[test]
    fn input_area_tree_basics() {
        let root = Rect::at_origin(vec2(40.0, 50.0));
        let (left, right) = root.split_portion_h(0.5);
        let (top, bottom) = right.split_portion_v(0.5);

        let root = InputArea::new(root, "root")
            .with_children(vec![
                InputArea::new(left, "left"),
                InputArea::new(right, "right")
                    .with_children(vec![
                        InputArea::new(top, "top"),
                        InputArea::new(bottom, "bottom"),
                    ]),
            ]);

        assert!(root.list_under(vec2(2.0, 30.0)) == vec!["root", "left"]);
        assert!(root.list_under(vec2(19.0, 3.0)) == vec!["root", "left"]);
        assert!(root.list_under(vec2(20.0, 3.0)) == vec!["root", "right", "top"]);
        assert!(root.list_under(vec2(39.0, 30.0)) == vec!["root", "right", "bottom"]);
        assert!(root.list_under(vec2(40.0, 30.0)) == Vec::<&'static str>::new());
    }

    #[test]
    fn input_area_hovering() {
        let root = Rect::at_origin(vec2(40.0, 50.0));
        let (left, right) = root.split_portion_h(0.5);
        let (top, bottom) = right.split_portion_v(0.5);

        let root = InputArea::new(root, "root")
            .with_children(vec![
                InputArea::new(left, "left"),
                InputArea::new(right, "right")
                    .with_children(vec![
                        InputArea::new(top, "top"),
                        InputArea::new(bottom, "bottom"),
                    ]),
            ]);

        let mut mouse_parser = MouseEventParser::new(root);
        assert_eq!(
            mouse_parser.handle_mouse_move(vec2(2.0, 30.0)),
            vec![
                MouseInput::Movement { delta: vec2(2.0, 30.0) },
                MouseInput::Enter { area: "root" },
                MouseInput::Enter { area: "left" },
            ],
        );
        assert_eq!(
            mouse_parser.handle_mouse_move(vec2(20.0, 3.0)),
            vec![
                MouseInput::Movement { delta: vec2(18.0, -27.0) },
                MouseInput::Leave { area: "left" },
                MouseInput::Enter { area: "right" },
                MouseInput::Enter { area: "top" },
            ],
        );
    }

    #[test]
    fn input_area_crawling() {
        let root = Rect::at_origin(vec2(40.0, 50.0));
        let (left, right) = root.split_portion_h(0.5);
        let (top, bottom) = right.split_portion_v(0.5);

        let root = InputArea::new(root, "root")
            .with_children(vec![
                InputArea::new(left, "left"),
                InputArea::new(right, "right")
                    .with_children(vec![
                        InputArea::new(top, "top"),
                        InputArea::new(bottom, "bottom"),
                    ]),
            ]);

        let mut areas = Vec::with_capacity(5);
        root.crawl(&mut |area| {
            areas.push(area.name);
        });

        assert_eq!(areas, vec!["root", "left", "right", "top", "bottom"])
    }
}
