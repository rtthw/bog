//! Bog Window



use std::sync::Arc;

use bog_event::{KeyCode, WindowEvent};
use bog_math::{vec2, Vec2};
pub use winit::{
    error::{EventLoopError as WindowManagerError, OsError as WindowError},
    event::{ElementState, Event as WindowManagerEvent},
    monitor::MonitorHandle,
    window::{CursorIcon, WindowId},
};



#[derive(Clone, Debug)]
pub struct Window(Arc<winit::window::Window>);

impl std::ops::Deref for Window {
    type Target = Arc<winit::window::Window>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



pub trait Client {
    fn on_resume(&mut self, wm: WindowManager);
    fn on_event(&mut self, wm: WindowManager, id: WindowId, event: WindowEvent);
}

struct ClientProxy<'a, C: Client> {
    client: &'a mut C,
}

impl<'a, C: Client> winit::application::ApplicationHandler for ClientProxy<'a, C> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client.on_resume(WindowManager { event_loop })
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(raw_event) = translate_window_event(event) {
            self.client.on_event(WindowManager { event_loop }, id, raw_event);
        }
    }
}

fn translate_window_event(window_event: winit::event::WindowEvent) -> Option<WindowEvent> {
    match window_event {
        winit::event::WindowEvent::CloseRequested => Some(WindowEvent::CloseRequest),
        winit::event::WindowEvent::RedrawRequested => Some(WindowEvent::RedrawRequest),

        winit::event::WindowEvent::Resized(new_size) => {
            let (width, height) = new_size.into();
            Some(WindowEvent::Resize { width, height })
        }
        winit::event::WindowEvent::Focused(focused) => {
            Some(if focused {
                WindowEvent::FocusIn
            } else {
                WindowEvent::FocusOut
            })
        }

        winit::event::WindowEvent::KeyboardInput { event, .. } => {
            let winit::event::KeyEvent {
                physical_key,
                state,
                repeat,
                ..
            } = event;
            match physical_key {
                winit::keyboard::PhysicalKey::Code(key_code) => {
                    let code = translate_winit_keycode(key_code)?;

                    Some(if state.is_pressed() {
                        WindowEvent::KeyDown {
                            code,
                            repeat,
                        }
                    } else {
                        WindowEvent::KeyUp {
                            code,
                        }
                    })
                }
                winit::keyboard::PhysicalKey::Unidentified(native_key_code) => {
                    println!(
                        "[bog] TODO: Handle unknown native key codes, got {:?}.",
                        native_key_code,
                    );
                    None
                }
            }
        }

        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(WindowEvent::MouseMove { x: position.x as _, y: position.y as _ })
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            let code = translate_winit_mousebutton(button);
            Some(if state.is_pressed() {
                WindowEvent::MouseDown { code }
            } else {
                WindowEvent::MouseUp { code }
            })
        }

        _ => None,
    }
}

fn translate_winit_keycode(winit_code: winit::keyboard::KeyCode) -> Option<KeyCode> {
    Some(match winit_code {
        winit::keyboard::KeyCode::KeyA => KeyCode::AN_A,
        winit::keyboard::KeyCode::KeyB => KeyCode::AN_B,
        winit::keyboard::KeyCode::KeyC => KeyCode::AN_C,
        winit::keyboard::KeyCode::KeyD => KeyCode::AN_D,
        winit::keyboard::KeyCode::KeyE => KeyCode::AN_E,
        winit::keyboard::KeyCode::KeyF => KeyCode::AN_F,
        winit::keyboard::KeyCode::KeyG => KeyCode::AN_G,
        winit::keyboard::KeyCode::KeyH => KeyCode::AN_H,
        winit::keyboard::KeyCode::KeyI => KeyCode::AN_I,
        winit::keyboard::KeyCode::KeyJ => KeyCode::AN_J,
        winit::keyboard::KeyCode::KeyK => KeyCode::AN_K,
        winit::keyboard::KeyCode::KeyL => KeyCode::AN_L,
        winit::keyboard::KeyCode::KeyM => KeyCode::AN_M,
        winit::keyboard::KeyCode::KeyN => KeyCode::AN_N,
        winit::keyboard::KeyCode::KeyO => KeyCode::AN_O,
        winit::keyboard::KeyCode::KeyP => KeyCode::AN_P,
        winit::keyboard::KeyCode::KeyQ => KeyCode::AN_Q,
        winit::keyboard::KeyCode::KeyR => KeyCode::AN_R,
        winit::keyboard::KeyCode::KeyS => KeyCode::AN_S,
        winit::keyboard::KeyCode::KeyT => KeyCode::AN_T,
        winit::keyboard::KeyCode::KeyU => KeyCode::AN_U,
        winit::keyboard::KeyCode::KeyV => KeyCode::AN_V,
        winit::keyboard::KeyCode::KeyW => KeyCode::AN_W,
        winit::keyboard::KeyCode::KeyX => KeyCode::AN_X,
        winit::keyboard::KeyCode::KeyY => KeyCode::AN_Y,
        winit::keyboard::KeyCode::KeyZ => KeyCode::AN_Z,
        _ => None?,
    })
}

fn translate_winit_mousebutton(winit_button: winit::event::MouseButton) -> u8 {
    match winit_button {
        winit::event::MouseButton::Left => 0,
        winit::event::MouseButton::Right => 1,
        winit::event::MouseButton::Middle => 2,
        winit::event::MouseButton::Back => 3,
        winit::event::MouseButton::Forward => 4,
        winit::event::MouseButton::Other(n) => (n + 5) as u8,
    }
}



pub struct WindowingSystem {
    event_loop: winit::event_loop::EventLoop<()>,
}

impl WindowingSystem {
    pub fn new() -> Result<Self, WindowManagerError> {
        Ok(Self {
            event_loop: winit::event_loop::EventLoop::new()?,
        })
    }

    pub fn run_client(self, client: &mut impl Client) -> Result<(), WindowManagerError> {
        self.event_loop.run_app(&mut ClientProxy { client })
    }
}



pub struct WindowManager<'a> {
    event_loop: &'a winit::event_loop::ActiveEventLoop,
}

impl<'a> WindowManager<'a> {
    pub fn create_window(&self, desc: WindowDescriptor) -> Result<Window, WindowError> {
        let inner = self.event_loop.create_window(winit::window::WindowAttributes::default()
            .with_title(desc.title)
            .with_inner_size(
                winit::dpi::LogicalSize::new(desc.inner_size.x as f64, desc.inner_size.y as f64)
            )
            .with_active(desc.active)
            .with_maximized(desc.maximized)
            .with_visible(desc.visible)
            .with_transparent(desc.transparent)
            .with_blur(desc.blurred)
            .with_decorations(desc.decorated))?;

        Ok(Window(Arc::new(inner)))
    }

    pub fn exit(&self) {
        self.event_loop.exit();
    }

    pub fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.event_loop.primary_monitor()
    }

    pub fn available_monitors(&self) -> impl Iterator<Item = MonitorHandle> {
        self.event_loop.available_monitors()
    }
}



pub struct WindowDescriptor<'a> {
    pub title: &'a str,
    pub inner_size: Vec2,
    pub active: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blurred: bool,
    pub decorated: bool,
}

impl<'a> Default for WindowDescriptor<'a> {
    fn default() -> Self {
        Self {
            title: "Untitled Window",
            inner_size: vec2(1280.0, 720.0),
            active: true,
            maximized: false,
            visible: true,
            transparent: false,
            blurred: false,
            decorated: true,
        }
    }
}
