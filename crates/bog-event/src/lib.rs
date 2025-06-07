//! Bog Events

#![no_std]



pub mod key;
pub use key::*;



pub enum WindowEvent {
    CloseRequest,
    RedrawRequest,
    Input(InputEvent),
}

pub enum InputEvent {
    Resize {
        width: u32,
        height: u32,
    },

    FocusIn,
    FocusOut,

    KeyDown {
        code: KeyCode,
        repeat: bool,
    },
    KeyUp {
        code: KeyCode,
    },

    MouseMove {
        x: f32,
        y: f32,
    },
    MouseDown {
        code: u8,
    },
    MouseUp {
        code: u8,
    },

    WheelMove(WheelMovement),
}

#[derive(Clone, Copy, Debug)]
pub enum WheelMovement {
    Lines {
        x: f32,
        y: f32,
    },
    Pixels {
        x: f32,
        y: f32,
    },
}
