//! X11-specific windowing types



use std::{ffi::{c_int, c_void}, num::NonZeroU32, ptr::NonNull};

use winit::raw_window_handle::*;



pub struct X11Window {
    window: XcbWindowHandle,
    display: XcbDisplayHandle,
}

impl X11Window {
    /// # Panics
    ///
    /// If the provided values are invalid (e.g. if `xproto_window` == 0).
    pub fn new(
        xproto_window: u32,
        xproto_visual: Option<u32>,
        conn_ptr: Option<NonNull<c_void>>,
        screen: c_int,
    ) -> Self {
        let mut window = XcbWindowHandle::new(NonZeroU32::new(xproto_window).unwrap());
        window.visual_id = xproto_visual.and_then(|n| NonZeroU32::new(n));
        let display = XcbDisplayHandle::new(conn_ptr, screen);

        Self {
            window,
            display,
        }
    }
}

impl HasWindowHandle for X11Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, winit::raw_window_handle::HandleError> {
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::Xcb(self.window))
        })
    }
}

impl HasDisplayHandle for X11Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Xcb(self.display))
        })
    }
}
