//! Input handling



use alloc::vec::Vec;

use crate::{vec, InputEvent, KeyCode};



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
