//! Bog Types
//!
//! The set of type definitions used in the bog environment and runtime.



pub use arrayvec::ArrayString;



#[derive(Debug)]
pub struct Message {
    pub code: [u8; 4],
    pub sender: u32,
    pub data: MessageData,
}

#[derive(Debug)]
pub enum MessageData {
    Info, // TODO: arrayvec::ArrayString
    Warning, // TODO: arrayvec::ArrayString
    Error, // TODO: arrayvec::ArrayString

    DeviceInput {
        device_id: u32,
    },
}

/// A request is a [message](crate::Message) that expects a response.
///
/// Because of this need for a synchronous response from the environment, they are sent across a
/// different channel from the normal messages.
#[derive(Debug)]
pub struct Request {
    pub code: [u8; 4],
    pub sender: u32,
    pub data: RequestData,
}

#[derive(Debug)]
pub enum RequestData {
    // TODO: There should be some sort of authentication here, and `sender` shouldn't just be the
    //       process ID.
    EstablishConnection {
        // key: [u8; 64],
    },
    CreateWindow {
        title: arrayvec::ArrayString<WINDOW_TITLE_MAX>,
    },
}

#[derive(Debug)]
pub struct Reply {
    pub success: bool,
    pub data: ReplyData,
}

#[derive(Debug)]
pub enum ReplyData {
    /// Null reply.
    Null,
    WindowCreated(WindowHandle),
}



pub const WINDOW_TITLE_MAX: usize = 64;

/// A raw handle to a window.
///
/// Currently, the fields map directly to the [raw-window-handle] definitions for each supported
/// platform. Fields *B* and *C* go unused if the platform's handle only requires 1 or 2 fields.
///
/// [raw-window-handle]: https://github.com/rust-windowing/raw-window-handle
#[derive(Debug)]
pub struct WindowHandle {
    /// Platform identifier.
    pub platform: u8,
    _padding: [u8; 7],
    /// Field A, see [type-level docs](crate::WindowHandle).
    pub a: u64,
    /// Field B, see [type-level docs](crate::WindowHandle).
    pub b: u64,
    /// Field C, see [type-level docs](crate::WindowHandle).
    pub c: u64,
}
