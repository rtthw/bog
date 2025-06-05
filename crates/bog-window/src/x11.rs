//! X11-specific windowing types



use core::{num::NonZeroU32, ptr::NonNull};

use bog_core::Arc;
use winit::raw_window_handle::*;
use x11rb::xcb_ffi::XCBConnection;



#[derive(Clone, Debug)]
pub struct X11Window {
    window: XcbWindowHandle,
    conn: Arc<XCBConnection>,
    screen: i32,
}

impl X11Window {
    /// # Panics
    ///
    /// If the provided values are invalid (e.g. if `xproto_window` == 0).
    pub fn new(
        xproto_window: u32,
        xproto_visual: Option<u32>,
        conn: Arc<XCBConnection>,
        screen: i32,
    ) -> Self {
        let mut window = XcbWindowHandle::new(NonZeroU32::new(xproto_window).unwrap());
        window.visual_id = xproto_visual.and_then(|n| NonZeroU32::new(n));

        Self {
            window,
            conn,
            screen,
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
            DisplayHandle::borrow_raw(RawDisplayHandle::Xcb(XcbDisplayHandle::new(
                NonNull::new(self.conn.get_raw_xcb_connection()),
                self.screen,
            )))
        })
    }
}
