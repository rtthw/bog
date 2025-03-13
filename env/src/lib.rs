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

    pub fn create_window(&mut self, title: &str) -> Result<Reply> {
        if title.len() > WINDOW_TITLE_MAX {
            return Err(Error::WindowTitleTooLong);
        }

        self.request(Request {
            code: [0, 0, 0, 0],
            sender: self.id,
            data: RequestData::CreateWindow {
                // SAFETY: We just checked the length, and the only way this can error is if the
                //         length is too long for the backing array.
                title: ArrayString::from(title).unwrap(),
            },
        })
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
