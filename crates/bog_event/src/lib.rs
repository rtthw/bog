//! Bog Events

#![no_std]



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



pub enum WindowEvent {
    CloseRequest,
    RedrawRequest,

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



#[derive(Clone, Copy, Eq, PartialEq)]
pub struct EventMask(u8);

bitflags::bitflags! {
    impl EventMask: u8 {
        /// Associated target responds to `mouseenter` and `mouseleave` events.
        const HOVER = 0;
        /// Associated target responds to `mousedown` and `mouseup` events.
        const CLICK = 1 << 0;
        /// Associated target responds to `focusin` and `focusout` events.
        const FOCUS = 1 << 1;
        /// Associated target responds to `dragmove`, `dragstart`, `dragend`, and `dragdrop` events.
        const DRAG = 1 << 2;
    }
}

impl core::fmt::Debug for EventMask {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EventMask {{")?;
        if self.clickable() {
            write!(f, " click")?;
        }
        if self.draggable() {
            write!(f, " drag")?;
        }
        if self.focusable() {
            write!(f, " focus")?;
        }
        write!(f, " }}")
    }
}

impl EventMask {
    #[inline]
    pub fn clickable(&self) -> bool {
        self.contains(Self::CLICK)
    }

    #[inline]
    pub fn draggable(&self) -> bool {
        self.contains(Self::DRAG)
    }

    #[inline]
    pub fn focusable(&self) -> bool {
        self.contains(Self::FOCUS)
    }
}
