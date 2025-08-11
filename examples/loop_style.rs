


use std::sync::mpsc::{channel, Receiver, Sender};

use bog::prelude::*;



fn main() -> Result<()> {
    let (request_sender, request_receiver) = channel::<Request>();
    let (response_sender, response_receiver) = channel::<Response>();

    let window_system = WindowingSystem::<Request>::new()?;
    let app_proxy = window_system.create_proxy();

    // Spawn a secondary thread to receive requests sent from the `main_loop` to the `LoopStyleApp`
    // through the windowing system proxy.
    std::thread::spawn(move || {
        loop {
            if let Ok(request) = request_receiver.recv() {
                if !app_proxy.send(request) {
                    break;
                }
            } else {
                break;
            }
        }

        // If this point is reached, that means the channel disconnected (app closed).
    });
    std::thread::spawn(move || {
        main_loop(request_sender, response_receiver)
    });

    window_system.run_app(&mut LoopStyleApp {
        response_sender,
    })?;

    Ok(())
}

#[allow(unused_variables, unused_assignments)]
fn main_loop(requests: Sender<Request>, responses: Receiver<Response>) {
    requests
        .send(Request::CreateWindow(
            WindowDescriptor {
                title: "Bog - Loop Style App Example",
                ..Default::default()
            }
        ))
        .unwrap();

    let mut window: Option<Window> = None;

    loop {
        if let Ok(response) = responses.try_recv() {
            match response {
                Response::WindowCreated(new_window) => {
                    window = Some(new_window);

                    // Do something with the window...
                }
            }
        }
    }
}



enum Request {
    CreateWindow(WindowDescriptor<'static>),
}

enum Response {
    WindowCreated(Window),
}



struct LoopStyleApp {
    response_sender: Sender<Response>,
}

impl App for LoopStyleApp {
    type CustomEvent = Request;

    fn on_event(&mut self, wm: WindowManager, event: AppEvent<Request>) {
        match event {
            AppEvent::Custom(request) => match request {
                Request::CreateWindow(desc) => {
                    let window = wm.create_window(desc).unwrap();
                    self.response_sender
                        .send(Response::WindowCreated(window.clone()))
                        .unwrap();
                }
            }
            AppEvent::Window { id: _, event } => match event {
                WindowEvent::CloseRequest => wm.exit(),
                _ => {}
            }
            _ => {} // TODO: Handle more events and send more responses.
        }
    }
}
