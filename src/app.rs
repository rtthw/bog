//! Application



use std::sync::Arc;

use bog_event::WindowEvent;
use bog_window::{Client, Window, WindowDescriptor, WindowId, WindowManager, WindowingSystem};

use crate::Result;



pub fn run_app(mut app: impl AppHandler) -> Result<()> {
    let windowing_system = WindowingSystem::new()?;
    let mut proxy = Proxy {
        app: &mut app,
        state: AppState::Suspended(None),
    };

    windowing_system.run_client(&mut proxy)?;

    Ok(())
}



/// A convenience trait for creating single-window programs.
pub trait AppHandler: 'static {
    fn title(&self) -> String {
        "Untitled".to_string()
    }
}

struct Proxy<'a> {
    app: &'a mut dyn AppHandler,
    state: AppState,
}

impl<'a> Client for Proxy<'a> {
    fn on_resume(&mut self, mut wm: WindowManager) {
        let AppState::Suspended(window) = &mut self.state else {
            return;
        };
        let window = window.take().unwrap_or_else(|| make_window(&mut wm, self.app));
        self.state = AppState::Active {
            window,
        };
    }

    // TODO: on_suspend

    fn on_event(&mut self, wm: WindowManager, _id: WindowId, event: WindowEvent) {
        let AppState::Active { window } = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::CloseRequest => {
                wm.exit();
            }
            _ => {}
        }
    }
}

enum AppState {
    Suspended(Option<Arc<Window>>),
    Active {
        window: Arc<Window>,
    },
}

fn make_window(wm: &mut WindowManager, app: &mut dyn AppHandler) -> Arc<Window> {
    Arc::new(wm.create_window(WindowDescriptor {
        title: &app.title(),
        ..Default::default()
    }).unwrap())
}
