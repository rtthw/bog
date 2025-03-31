


pub use winit::event::{DeviceId as Device, MouseButton};

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
        let mut winit_window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(screen_width, screen_height))
            .build(&event_loop)
            .unwrap();
        let mut graphics = WindowGraphics::from_winit_window(
            &winit_window,
            GraphicsConfig::new(screen_width as u32, screen_height as u32),
        ).unwrap();


        event_loop.run(move |event, _target, control_flow| {
            let mut handle = WindowHandle {
                winit_window: &mut winit_window,
                graphics: &mut graphics,
                control_flow
            };
            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    if let Some(event) = translate_winit_window_event(event) {
                        window.on_event(&mut handle, event);
                    }
                }
                winit::event::Event::RedrawRequested(_) => {
                    window.on_event(&mut handle, WindowEvent::RenderRequest);
                }
                _ => {}
            }
        });
    }
}

pub fn translate_winit_window_event(event: winit::event::WindowEvent) -> Option<WindowEvent> {
    Some(match event {
        winit::event::WindowEvent::CloseRequested => WindowEvent::CloseRequest,
        winit::event::WindowEvent::Resized(size) => WindowEvent::Resize(size.into()),
        winit::event::WindowEvent::MouseInput { device_id, state, button, .. } => {
            if state == winit::event::ElementState::Pressed {
                WindowEvent::MouseButtonPress { device: device_id, button }
            } else {
                WindowEvent::MouseButtonRelease { device: device_id, button }
            }
        }
        _ => None?,
    })
}



pub trait Window {
    fn on_event(&mut self, handle: &mut WindowHandle, event: WindowEvent);
}

pub enum WindowEvent {
    RenderRequest,
    CloseRequest,
    Resize((f32, f32)),
    MouseButtonPress {
        device: Device,
        button: MouseButton,
    },
    MouseButtonRelease {
        device: Device,
        button: MouseButton,
    },
}

pub struct WindowHandle<'a> {
    winit_window: &'a mut winit::window::Window,
    graphics: &'a mut WindowGraphics,
    control_flow: &'a mut winit::event_loop::ControlFlow,
}

impl<'a> WindowHandle<'a> {
    pub fn graphics(&mut self) -> &mut WindowGraphics {
        &mut self.graphics
    }

    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title);
    }

    pub fn inner_size(&self) -> (f32, f32) {
        self.winit_window.inner_size().into()
    }

    pub fn close(&mut self) {
        self.control_flow.set_exit();
    }
}
