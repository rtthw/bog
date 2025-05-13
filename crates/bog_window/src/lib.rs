//! # Bog Window
//!
//! A set of types and abstractions useful for managing [`Window`]s through [`WindowingSystem`]s.



#[cfg(feature = "x11")]
pub mod x11;

use std::sync::Arc;

use bog_event::{KeyCode, WindowEvent};
use bog_math::{vec2, Vec2};

pub use winit::raw_window_handle as rwh;
pub use winit::{
    error::{EventLoopError as WindowManagerError, OsError as WindowError},
    event::{ElementState, Event as WindowManagerEvent},
    window::{CursorIcon, WindowId},
};



/// A reference to a managed window.
///
/// You can safely clone this object and access clones the same as would the original. When the
/// last reference to this window is dropped, the window will be closed.
#[derive(Clone, Debug)]
pub struct Window(Arc<winit::window::Window>);

impl std::ops::Deref for Window {
    type Target = Arc<winit::window::Window>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl rwh::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        self.0.window_handle()
    }
}

impl rwh::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        self.0.display_handle()
    }
}



/// A reference to a physical monitor display.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Monitor(winit::monitor::MonitorHandle);

impl Monitor {
    /// Get the name of this monitor. If this monitor doesn't exist (has been disconnected since
    /// the creation of this reference), this returns `None`.
    #[inline]
    pub fn name(&self) -> Option<String> {
        self.0.name()
    }

    /// Get the resolution of this monitor, in physical pixels.
    #[inline]
    #[doc(alias = "size")]
    pub fn resolution(&self) -> Vec2 {
        let (x, y) = self.0.size().into();
        Vec2::new(x, y)
    }

    /// Get the position of this monitor, in physical pixels relative to the origin of the user's
    /// setup.
    ///
    /// This means that if the user has a 2 monitor setup, then the left one would return `(0, 0)`
    /// and the right one would return `(<left_monitor_width>, 0)`.
    #[inline]
    pub fn position(&self) -> Vec2 {
        let (x, y) = self.0.size().into();
        Vec2::new(x, y)
    }
}



/// Wndowing clients can be registered through the [`WindowingSystem`] and create windows through
/// the [`WindowManager`]. Once registered, the client will receive events from the windowing
/// system that it can then use to perform some behavior.
pub trait WindowingClient {
    /// Called when this client's connection to the [`WindowManager`] is first established. This
    /// can be used to create an initial window if the client is some sort of application.
    ///
    /// It should be noted that the client will also receive a resume event after this one.
    fn on_startup(&mut self, wm: WindowManager);
    /// Called when this client is resumed.
    ///
    /// There is no guarantee that a client will recieve suspend and resume events in any
    /// particular order.
    ///
    /// On certain platforms, this may only be called when the [`WindowManager`] is first
    /// initialized. See [`WindowingClient::on_startup`].
    fn on_resume(&mut self, wm: WindowManager);
    /// Called when this client is suspended.
    ///
    /// There is no guarantee that a client will recieve suspend and resume events in any
    /// particular order.
    ///
    /// On certain platforms, this may never be called.
    fn on_suspend(&mut self, wm: WindowManager);
    /// Called when one of this client's windows receives a [`WindowEvent`].
    fn on_event(&mut self, wm: WindowManager, id: WindowId, event: WindowEvent);
}

struct ClientProxy<'a, C: WindowingClient> {
    client: &'a mut C,
}

impl<'a, C: WindowingClient> winit::application::ApplicationHandler for ClientProxy<'a, C> {
    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match cause {
            winit::event::StartCause::Init => {
                self.client.on_startup(WindowManager { event_loop });
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client.on_resume(WindowManager { event_loop })
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client.on_suspend(WindowManager { event_loop })
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
        winit::keyboard::KeyCode::Digit0 => KeyCode::AN_0,
        winit::keyboard::KeyCode::Digit1 => KeyCode::AN_1,
        winit::keyboard::KeyCode::Digit2 => KeyCode::AN_2,
        winit::keyboard::KeyCode::Digit3 => KeyCode::AN_3,
        winit::keyboard::KeyCode::Digit4 => KeyCode::AN_4,
        winit::keyboard::KeyCode::Digit5 => KeyCode::AN_5,
        winit::keyboard::KeyCode::Digit6 => KeyCode::AN_6,
        winit::keyboard::KeyCode::Digit7 => KeyCode::AN_7,
        winit::keyboard::KeyCode::Digit8 => KeyCode::AN_8,
        winit::keyboard::KeyCode::Digit9 => KeyCode::AN_9,

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

        winit::keyboard::KeyCode::Backquote => KeyCode::AN_TILDE,
        winit::keyboard::KeyCode::Minus => KeyCode::AN_MINUS,
        winit::keyboard::KeyCode::Equal => KeyCode::AN_EQUAL,
        winit::keyboard::KeyCode::BracketLeft => KeyCode::AN_LBRACKET,
        winit::keyboard::KeyCode::BracketRight => KeyCode::AN_RBRACKET,
        winit::keyboard::KeyCode::Backslash => KeyCode::AN_BACKSLASH,
        winit::keyboard::KeyCode::Semicolon => KeyCode::AN_SEMICOLON,
        winit::keyboard::KeyCode::Quote => KeyCode::AN_APOSTROPHE,
        winit::keyboard::KeyCode::Comma => KeyCode::AN_COMMA,
        winit::keyboard::KeyCode::Period => KeyCode::AN_DOT,
        winit::keyboard::KeyCode::Slash => KeyCode::AN_SLASH,

        winit::keyboard::KeyCode::ControlLeft => KeyCode::C_LCTRL,
        winit::keyboard::KeyCode::ControlRight => KeyCode::C_RCTRL,
        winit::keyboard::KeyCode::ShiftLeft => KeyCode::C_LSHIFT,
        winit::keyboard::KeyCode::ShiftRight => KeyCode::C_RSHIFT,
        winit::keyboard::KeyCode::AltLeft => KeyCode::C_LALT,
        winit::keyboard::KeyCode::AltRight => KeyCode::C_RALT,
        winit::keyboard::KeyCode::SuperLeft => KeyCode::C_LMETA,
        winit::keyboard::KeyCode::SuperRight => KeyCode::C_RMETA,

        winit::keyboard::KeyCode::Space => KeyCode::C_SPACE,
        winit::keyboard::KeyCode::Backspace => KeyCode::C_BACKSPACE,
        winit::keyboard::KeyCode::Tab => KeyCode::C_TAB,
        winit::keyboard::KeyCode::Enter => KeyCode::C_ENTER,
        winit::keyboard::KeyCode::Escape => KeyCode::C_ESCAPE,
        winit::keyboard::KeyCode::ContextMenu => KeyCode::C_MENU,

        winit::keyboard::KeyCode::Insert => KeyCode::C_INSERT,
        winit::keyboard::KeyCode::Delete => KeyCode::C_DELETE,
        winit::keyboard::KeyCode::Home => KeyCode::C_HOME,
        winit::keyboard::KeyCode::End => KeyCode::C_END,
        winit::keyboard::KeyCode::PageUp => KeyCode::C_PAGEUP,
        winit::keyboard::KeyCode::PageDown => KeyCode::C_PAGEDOWN,
        winit::keyboard::KeyCode::ArrowUp => KeyCode::C_ARROWUP,
        winit::keyboard::KeyCode::ArrowDown => KeyCode::C_ARROWDOWN,
        winit::keyboard::KeyCode::ArrowLeft => KeyCode::C_ARROWLEFT,
        winit::keyboard::KeyCode::ArrowRight => KeyCode::C_ARROWRIGHT,

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



/// A reference to the platform's windowing system.
///
/// The windowing system for a platform is responsible for creating, destroying, and positioning
/// windows. Sometimes (usually on mobile and web platforms), the windowing system is also
/// responsible for managing the "lifecycle" of its clients. This takes the form of resume,
/// suspend, and wakeup events that are passed to the clients managing the affected window(s).
///
/// To access a windowing system, you create a [`Client`] object that will be capable of creating
/// windows through the [`WindowManager`] object.
pub struct WindowingSystem {
    event_loop: winit::event_loop::EventLoop<()>,
}

impl WindowingSystem {
    pub fn new() -> Result<Self, WindowManagerError> {
        Ok(Self {
            event_loop: winit::event_loop::EventLoop::new()?,
        })
    }

    pub fn run_client(self, client: &mut impl WindowingClient) -> Result<(), WindowManagerError> {
        self.event_loop.run_app(&mut ClientProxy { client })
    }
}



/// A reference to the platform's window manager.
///
/// Not to be confused with the [`WindowingSystem`], the window manager is responsible for the
/// actual window management behavior associated with a windowing system. You can think of this
/// object as a sort of "dispatcher" for requests to the system.
///
/// This window manager object is tied to the [`WindowingClient`] you provided to the windowing
/// system. Calling [`WindowManager::exit`] will ask the windowing system to shutdown the client.
pub struct WindowManager<'a> {
    event_loop: &'a winit::event_loop::ActiveEventLoop,
}

impl<'a> WindowManager<'a> {
    /// Attempt to create a new [`Window`] as defined by the given [`WindowDescriptor`].
    ///
    /// This could return an error if the platform does not support windowing, there is no
    /// available memory for the window, or the platform denies the request.
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

    /// Stop processing window manager events.
    pub fn exit(&self) {
        self.event_loop.exit();
    }

    /// Get the user's primary [`Monitor`]. If the user hasn't explicitly defined one, this returns
    /// `None`.
    pub fn primary_monitor(&self) -> Option<Monitor> {
        self.event_loop.primary_monitor().map(|monitor_handle| Monitor(monitor_handle))
    }

    /// Get an iterator over the available [`Monitor`]s for a user's system.
    pub fn available_monitors(&self) -> impl Iterator<Item = Monitor> {
        self.event_loop.available_monitors().map(|monitor_handle| Monitor(monitor_handle))
    }
}



/// The initial description of a [`Window`] used by [`WindowManager::create_window`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDescriptor<'a> {
    /// The window's title.
    ///
    /// Does nothing on platforms that don't manage window titles (most notably, mobile platforms).
    pub title: &'a str,
    /// The inner size of the window. Some platforms add decorations to windows, so this is the
    /// size of the actual display surface.
    pub inner_size: Vec2,
    /// Whether the window is active.
    pub active: bool,
    /// Whether the window is maximized.
    pub maximized: bool,
    /// Whether the window is visible.
    pub visible: bool,
    /// Whether the window is transparent.
    pub transparent: bool,
    /// Whether the window is blurred.
    pub blurred: bool,
    /// Whether the window is decorated.
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
