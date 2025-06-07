//! Event types



use crate::KeyCode;



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
        button: MouseButton,
    },
    MouseUp {
        button: MouseButton,
    },

    WheelMove(WheelMovement),
}

/// A button on the user's mouse.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
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
