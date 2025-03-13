//! Bog Types
//!
//! The set of type definitions used in the bog environment and runtime.



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
    EstablishConnection,
    CreateWindow, // TODO: arrayvec::ArrayString
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
