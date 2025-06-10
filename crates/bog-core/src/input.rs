//! Input handling



use alloc::vec::Vec;

use crate::{vec, InputEvent, KeyCode};



/// Translates raw [`InputEvent`]s into more intuitive [`Input`]s.
pub struct InputHandler {
    keys_down: rustc_hash::FxHashSet<KeyCode>,
}

impl InputHandler {
    pub fn handle_event(&mut self, event: InputEvent) -> Vec<Input> {
        match event {
            InputEvent::KeyDown { code, repeat } => {
                if repeat {
                    vec![Input::RepeatKeyPress(code)]
                } else {
                    let _was_actually_repeat = !self.keys_down.insert(code);
                    Vec::new()
                }
            }
            InputEvent::KeyUp { code } => {
                if self.keys_down.remove(&code) {
                    vec![Input::FullKeyPress(code)]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }
}



#[derive(Clone, Debug)]
pub enum Input {
    /// A valid key up event after a valid key down event.
    FullKeyPress(KeyCode),
    /// A repeated key down event.
    RepeatKeyPress(KeyCode),
}
