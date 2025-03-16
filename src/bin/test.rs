


use bog::*;
use bog_types::*;



fn main() -> Result<()> {
    let mut conn = connect()?;

    let mut window = {
        let window_reply = conn.request(Request {
            code: [0, 0, 0, 0],
            sender: conn.id(),
            data: RequestData::CreateWindow {
                title: ArrayString::from("Testing Window").unwrap(),
            },
        })?;
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

    let mut sent_close_req = false;
    loop {
        match window.wait_for_input() {
            WindowInput::Closed => break,
            WindowInput::User(user_input) => {
                println!("Got {:?}", user_input);
            }
            WindowInput::Device(device_input) => {
                println!("Got {:?}", device_input);
                if !sent_close_req {
                    let reply = conn.request(Request {
                        code: [0, 0, 0, 0],
                        sender: conn.id(),
                        data: RequestData::CloseWindow(window.id()),
                    })?;
                    sent_close_req = true;
                    println!("Got reply: {reply:?}");
                }
            }
        }
    }

    Ok(())
}
