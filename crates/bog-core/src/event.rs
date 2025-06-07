//! Event types



use crate::KeyCode;



/// An event that can be passed from a windowing system to a window controller.
#[derive(Clone, Copy, Debug)]
pub enum WindowEvent {
    /// The user wants this window to close. Usually, this event is generated when the user presses
    /// the close button on the window header bar.
    CloseRequest,
    /// The window is requesting a re-render of its contents.
    RedrawRequest,
    /// A raw [`InputEvent`].
    Input(InputEvent),
}

/// A raw input event passed into an area.
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    /// This area was resized to these proportions, in physical pixels.
    Resize {
        width: u32,
        height: u32,
    },
    /// This area gained the user's focus.
    FocusIn,
    /// This area lost the user's focus.
    FocusOut,
    /// A [key](KeyCode) was pressed down while this area had the user's focus.
    KeyDown {
        code: KeyCode,
        repeat: bool,
    },
    /// A [key](KeyCode) was released while this area had the user's focus.
    KeyUp {
        code: KeyCode,
    },
    /// The user's mouse pointer moved while over this area.
    MouseMove {
        x: f32,
        y: f32,
    },
    /// A button on the user's mouse was pressed down while this area had the user's focus.
    MouseDown {
        button: MouseButton,
    },
    /// A button on the user's mouse was released while this area had the user's focus.
    MouseUp {
        button: MouseButton,
    },
    /// The user's wheel device moved in some way.
    ///
    /// The type of movement (lines or pixels) depends on the type of wheel device used. For a
    /// traditional mouse wheel, this will likely return [`WheelMovement::Lines`]. But for touch
    /// devices (like trackpads), this will likely return [`WheelMovement::Pixels`].
    WheelMove(WheelMovement),
}

/// A button on the user's mouse.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MouseButton {
    /// The left mouse button (LMB).
    Left,
    /// The right mouse button (RMB).
    Right,
    /// The middle mouse button (MMB).
    Middle,
    /// The back button.
    Back,
    /// The forward button.
    Forward,
    /// A mouse button that is not one of the main five mouse buttons (left, right, middle,
    /// forward, back).
    Other(u16),
}

/// A mouse wheel or trackpad scroll.
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
