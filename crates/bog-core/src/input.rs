//! Input handling



use alloc::vec::Vec;

use crate::{vec, InputEvent, KeyCode, Rect, Vec2};



pub struct EventParser {
    key: KeyEventParser,
}

impl EventParser {
    pub fn parse_event(&mut self, event: InputEvent) -> Vec<Input> {
        match event {
            InputEvent::KeyDown { code, repeat } => {
                self.key.handle_key_down(code, repeat).into_iter().map(Input::Key).collect()
            }
            InputEvent::KeyUp { code } => {
                self.key.handle_key_up(code).into_iter().map(Input::Key).collect()
            }
            _ => Vec::new(),
        }
    }
}

/// Translates raw key [`InputEvent`]s into more intuitive [`KeyInput`]s.
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



#[derive(Clone, Debug)]
pub enum Input {
    Key(KeyInput),
}

#[derive(Clone, Debug)]
pub enum KeyInput {
    /// A valid key up event after a valid key down event.
    FullKeyPress(KeyCode),
    /// A repeated key down event.
    RepeatKeyPress(KeyCode),
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
