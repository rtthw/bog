


use crate::graphics::{GraphicsConfig, WindowGraphics};



pub struct SingleWindowProgram<W: Window + 'static> {
    window: W,
    screen_size: (f32, f32),
    title: String,
}

impl<W: Window + 'static> SingleWindowProgram<W> {
    pub fn new(window: W) -> Self {
        Self {
            window,
            screen_size: (640.0, 360.0),
            title: "Untitled Window".to_string(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.screen_size = (width, height);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn run(self) -> ! {
        let SingleWindowProgram {
            mut window,
            screen_size: (screen_width, screen_height),
            title,
        } = self;

        let event_loop = winit::event_loop::EventLoop::new();
        let event_loop = winit::event_loop::EventLoop::new();
        let winit_window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(screen_width, screen_height))
            .build(&event_loop)
            .unwrap();
        let mut graphics = WindowGraphics::from_winit_window(
            &winit_window,
            GraphicsConfig::new(screen_width as u32, screen_height as u32),
        ).unwrap();

        let mut handle = WindowHandle {
            winit_window,
        };

        event_loop.run(move |event, _target, control_flow| {
            // Single window programs can ignore window IDs entirely.
            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        _ => {}
                    }
                }
                winit::event::Event::RedrawRequested(_) => {
                    window.render(&mut handle);
                }
                _ => {}
            }
        });
    }
}



pub trait Window {
    fn render(&mut self, handle: &mut WindowHandle);
}

pub struct WindowHandle {
    winit_window: winit::window::Window,
}

impl WindowHandle {
    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title);
    }
}
