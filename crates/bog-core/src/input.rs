//! Input handling



use alloc::vec::Vec;

use crate::{vec, vec2, InputEvent, KeyCode, Rect, Vec2};



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
    hovered: Vec<&'static str>,
}

impl MouseEventParser {
    pub fn new(root_area: InputArea) -> Self {
        Self {
            root_area,
            hovered: Vec::new(),
        }
    }

    pub fn handle_mouse_move(&mut self, new_position: Vec2) -> Vec<MouseInput> {
        let mut inputs = vec![];

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

        inputs
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

#[derive(Clone, Debug)]
pub enum MouseInput {
    Enter {
        area: &'static str,
    },
    Leave {
        area: &'static str,
    },
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
    use alloc::vec;

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
}
