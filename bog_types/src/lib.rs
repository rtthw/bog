//! Bog Types
//!
//! The set of type definitions used in the bog environment and runtime.



mod inclination;
pub use inclination::*;

pub use arrayvec::{ArrayString, ArrayVec};



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
    CloseWindow {
        id: u64,
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
    WindowCreated(u64),
}



#[derive(Debug)]
pub struct DeviceInput {
    pub source: u32,
    pub device: u32,
    pub code: u32,
    pub state: bool,
}

#[derive(Debug)]
pub enum UserInput {
    Move(Inclination),
}



pub const WINDOW_TITLE_MAX: usize = 64;

pub enum WindowInput {
    Closed,
    Device(DeviceInput),
    User(UserInput),
}
