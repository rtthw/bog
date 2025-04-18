//! Bog Events



pub enum EventType {
    /// ## Generation
    /// ## Dispatch
    MouseMove,
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
}

#[allow(unused)]
pub trait EventTarget {
    type Context;

    fn on_mousemove(&mut self, ctx: Self::Context) {}
    fn on_mousedown(&mut self, ctx: Self::Context) {}
    fn on_mouseup(&mut self, ctx: Self::Context) {}
    fn on_click(&mut self, ctx: Self::Context) {}
    fn on_auxclick(&mut self, ctx: Self::Context) {}
}
