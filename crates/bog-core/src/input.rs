//! Input handling



use crate::{vec2, InputEvent, KeyCode, MouseButton, Rect, Vec2};



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
            _ => Vec::new(),
        }
    }

    pub fn update_areas(&mut self, root_area: InputArea) {
        self.mouse.update_areas(root_area)
    }
}

/// Translates raw key [`InputEvent`]s into more intuitive [`KeyInput`]s.
#[derive(Debug, Default)]
pub struct KeyEventParser {
    keys_down: rustc_hash::FxHashSet<KeyCode>,
}

impl KeyEventParser {
    pub fn handle_key_down(&mut self, code: KeyCode, repeat: bool) -> Vec<KeyInput> {
        if repeat {
            vec![KeyInput::RepeatKeyPress(code)]
        } else {
            let _was_actually_repeat = !self.keys_down.insert(code);
            Vec::new()
        }
    }

    pub fn handle_key_up(&mut self, code: KeyCode) -> Vec<KeyInput> {
        if self.keys_down.remove(&code) {
            vec![KeyInput::FullKeyPress(code)]
        } else {
            Vec::new()
        }
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
                    self.is_dragging = true; // TODO: Wait ~0.1 seconds before starting drag.
                    self.drag_start_pos = Some(self.mouse_pos);
                    self.drag_start_area = Some(*topmost_hovered_area);
                    inputs.push(MouseInput::DragStart {
                        pos: self.mouse_pos,
                        area: *topmost_hovered_area,
                    });
                }
                // NOTE: We check twice here because it could change in the first statement.
                if self.is_dragging {
                    inputs.push(MouseInput::DragMove {
                        delta: move_delta,
                        area: *topmost_hovered_area,
                    });
                }
            }
        }

        inputs
    }

    pub fn handle_mouse_down(&mut self, button: MouseButton) -> Vec<MouseInput> {
        match button {
            MouseButton::Left => self.buttons_down.insert(MouseButtonMask::LEFT),
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
        match button {
            MouseButton::Left => self.buttons_down.remove(MouseButtonMask::LEFT),
            MouseButton::Right => self.buttons_down.remove(MouseButtonMask::RIGHT),
            MouseButton::Middle => self.buttons_down.remove(MouseButtonMask::MIDDLE),
            _ => {}
        }

        self.hovered.clone()
            .into_iter()
            .map(|name| MouseInput::Release { area: name, button })
            .collect()
    }

    pub fn update_areas(&mut self, root_area: InputArea) {
        self.root_area = root_area;
    }
}



#[derive(Clone, Debug)]
pub enum Input {
    Key(KeyInput),
    Mouse(MouseInput),
}

#[derive(Clone, Debug)]
pub enum KeyInput {
    /// A valid key up event after a valid key down event.
    FullKeyPress(KeyCode),
    /// A repeated key down event.
    RepeatKeyPress(KeyCode),
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
    Press {
        area: &'static str,
        button: MouseButton,
    },
    Release {
        area: &'static str,
        button: MouseButton,
    },
    Drag {
        start_pos: Vec2,
        end_pos: Vec2,
        start_area: &'static str,
        end_area: &'static str,
    },
    DragStart {
        pos: Vec2,
        area: &'static str,
    },
    DragMove {
        delta: Vec2,
        area: &'static str,
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
}



#[cfg(test)]
mod tests {
    use crate::vec2;

    use super::*;

    #[test]
    fn input_area_tree_basics() {
        let root = Rect::new(Vec2::ZERO, vec2(40.0, 50.0));
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
        let root = Rect::new(Vec2::ZERO, vec2(40.0, 50.0));
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
                MouseInput::Enter { area: "root" },
                MouseInput::Enter { area: "left" },
            ],
        );
        assert_eq!(
            mouse_parser.handle_mouse_move(vec2(20.0, 3.0)),
            vec![
                MouseInput::Leave { area: "left" },
                MouseInput::Enter { area: "right" },
                MouseInput::Enter { area: "top" },
            ],
        );
    }
}
