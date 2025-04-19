//! Bog Events



pub mod key;
pub use key::*;



pub enum EventType {
    /// ## Generation
    /// When the user presses a keyboard key, a *keydown* event is generated.
    ///
    /// ## Dispatch
    /// A *keydown* event is always dispatched in descending order of the current focus level.
    KeyDown,
    /// ## Generation
    /// When the user releases a keyboard key, a *keyup* event is generated.
    ///
    /// ## Dispatch
    /// A *keyup* event is always dispatched in descending order of the current focus level.
    KeyUp,

    /// ## Generation
    /// ## Dispatch
    MouseMove,
    /// ## Generation
    /// ## Dispatch
    MouseEnter,
    /// ## Generation
    /// ## Dispatch
    MouseLeave,
    /// ## Generation
    /// ## Dispatch
    MouseDown,
    /// ## Generation
    /// ## Dispatch
    MouseUp,

    /// ## Generation
    /// If the target of a *mouseup* event is the same target as that of the preceding *mousedown*
    /// event, then a *click* event must be generated if the button of both events is the user's
    /// **primary mouse button**.
    ///
    /// There must be some allowance for minor mouse movements during the click event, so it is
    /// not guaranteed that the events will occur in exactly *mousedown*, *mouseup*, *click* order.
    ///
    /// ## Dispatch
    /// A *click* event must always be dispatched to the topmost event target, indicated by the
    /// pointer's position at the time of the *mouseup* event generation.
    Click,
    /// ## Generation
    /// ## Dispatch
    AuxClick,
    /// ## Generation
    /// A *dblclick* event must be preceded by a *click* event. The first subsequent *click* event
    /// that would be generated within some set time frame (usually, 500ms) following the first
    /// *click* event instead generates a *dblclick* event.
    ///
    /// No second *click* event is ever generated or dispatched.
    ///
    /// It is up to the implementor to determine the timing tolerance between a *click* event and
    /// its subsequent *dblclick* event.
    /// ## Dispatch
    /// Like the *click* event, a *dblclick* event must be dispatched to the topmost event target,
    /// as indicated by the pointer's position at the time of the **second** *mouseup* event.
    DblClick,

    /// ## Generation
    /// ## Dispatch
    FocusIn,
    /// ## Generation
    /// ## Dispatch
    FocusOut,
}

#[allow(unused)]
pub trait EventTarget {
    type Context;

    fn on_keydown(&mut self, ctx: Self::Context) {}
    fn on_keyup(&mut self, ctx: Self::Context) {}
    fn on_mousemove(&mut self, ctx: Self::Context) {}
    fn on_mouseenter(&mut self, ctx: Self::Context) {}
    fn on_mouseleave(&mut self, ctx: Self::Context) {}
    fn on_mousedown(&mut self, ctx: Self::Context) {}
    fn on_mouseup(&mut self, ctx: Self::Context) {}
    fn on_click(&mut self, ctx: Self::Context) {}
    fn on_dblclick(&mut self, ctx: Self::Context) {}
    fn on_auxclick(&mut self, ctx: Self::Context) {}
    fn on_focusin(&mut self, ctx: Self::Context) {}
    fn on_focusout(&mut self, ctx: Self::Context) {}
}



pub enum RawEvent {
    KeyDown {
        code: KeyCode,
        repeat: bool,
    },
    KeyUp {
        code: KeyCode,
    },
}
