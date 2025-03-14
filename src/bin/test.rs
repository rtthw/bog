


use bog_types::*;
use env::*;



fn main() -> Result<()> {
    let mut conn = connect()?;

    let mut window = {
        let window_reply = conn.create_window("Testing Window")?;
        if window_reply.success {
            if let ReplyData::WindowCreated(id) = window_reply.data {
                Window::new(id)
            } else {
                panic!("ERROR: Window creation output invalid data");
            }
        } else {
            panic!("ERROR: Window creation failed");
        }
    }?;

    loop {
        match window.wait_for_input() {
            WindowInput::Closed => break,
            WindowInput::User(user_input) => {
                println!("Got {:?}", user_input);
            }
            WindowInput::Device(device_input) => {
                println!("Got {:?}", device_input);
            }
        }
    }

    Ok(())
}
