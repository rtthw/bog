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
    /// All zeroes.
    Null,
    WindowCreated(u32),
}



pub const WINDOW_TITLE_MAX: usize = 64;
