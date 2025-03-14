


use bog_types::*;
use env::*;



fn main() -> Result<()> {
    let mut conn = connect()?;

    let window_reply = conn.create_window("Testing Window")?;
    if window_reply.success {
        if let ReplyData::WindowCreated(handle) = window_reply.data {
            let window = Window::new(handle);
        }
    }

    Ok(())
}
