//! Bog Environment



use bog_types::*;



/// A connection to the runtime environment.
pub struct Connection {
    id: u32,
    request_channel: ration::Array<Request>,
    reply_channel: ration::Array<Reply>,
}

impl Connection {
    /// The universal identifier for this connection.
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Connection {
    pub fn request(&mut self, request: Request) -> Result<Reply> {
        if !self.request_channel.push(request) {
            println!("ERROR: Could not send request");
        }

        // TODO: Request timeout.
        loop {
            if let Some(reply) = self.reply_channel.pop() {
                return Ok(reply);
            }
        }
    }
}

/// Attempt to create a connection to the environment.
pub fn connect() -> Result<Connection> {
    let id = std::process::id();

    let request_channel = ration::Array::open("/tmp/BOG_REQUESTS")?;
    let reply_channel = ration::Array::alloc(format!("/tmp/BOG_{}_REPLIES", id), 5)?;

    let mut conn = Connection {
        id,
        request_channel,
        reply_channel,
    };

    let reply = conn.request(Request {
        code: [0, 0, 0, 0],
        sender: id,
        data: RequestData::EstablishConnection {},
    })?;

    if !reply.success {
        return Err(Error::ConnectionDenied);
    }

    Ok(conn)
}



pub struct Window {
    id: u64,
    input_channel: ration::Array<WindowInput>,
}

impl Window {
    pub fn new(id: u64) -> Result<Self> {
        let input_channel = ration::Array::open(format!("/tmp/BOG_WINDOW_{}_INPUT", id))?;

        Ok(Self {
            id,
            input_channel,
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn wait_for_input(&mut self) -> WindowInput {
        loop {
            if let Some(input) = self.poll_for_input() {
                return input;
            }
        }
    }

    pub fn poll_for_input(&mut self) -> Option<WindowInput> {
        self.input_channel.pop()
    }
}



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Shm(ration::Error),

    ConnectionDenied,
    WindowTitleTooLong,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ration::Error> for Error {
    fn from(value: ration::Error) -> Self {
        Self::Shm(value)
    }
}
