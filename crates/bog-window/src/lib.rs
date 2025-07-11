//! # Bog Window
//!
//! A set of types and abstractions useful for managing [`Window`]s through [`WindowingSystem`]s.

// #![no_std]



#[cfg(feature = "x11")]
pub mod x11;

use std::sync::Arc;

use bog_core::{InputEvent, KeyCode, MouseButton, WheelMovement, WindowEvent};
use bog_core::{vec2, Vec2};

pub use winit::raw_window_handle as rwh;
pub use winit::{
    error::{EventLoopError as WindowManagerError, OsError as WindowError},
    window::{Cursor, CursorIcon, CustomCursor},
};



/// A reference to a managed window.
///
/// You can safely clone this object and access clones the same as you would the original. When the
/// last reference to this window is dropped, the window will be closed.
#[derive(Clone, Debug)]
pub struct Window(Arc<winit::window::Window>);

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

// Getters.
impl Window {
    /// Get the name of this window.
    #[inline]
    pub fn name(&self) -> String {
        self.0.title()
    }

    #[inline]
    pub fn scale(&self) -> f64 {
        self.0.scale_factor()
    }
}

// Misc.
impl Window {
    #[inline]
    pub fn request_redraw(&self) {
        self.0.request_redraw();
    }

    #[inline]
    pub fn set_cursor(&self, cursor: impl Into<Cursor>) {
        self.0.set_cursor(cursor);
    }
}

// Focusing.
impl Window {
    #[inline]
    pub fn has_focus(&self) -> bool {
        self.0.has_focus()
    }

    #[inline]
    pub fn steal_focus(&self) {
        self.0.focus_window();
    }
}

// Monitors.
impl Window {
    #[inline]
    pub fn monitor(&self) -> Option<Monitor> {
        self.0.current_monitor().map(|m| Monitor(m))
    }

    #[inline]
    pub fn primary_monitor(&self) -> Option<Monitor> {
        self.0.primary_monitor().map(|m| Monitor(m))
    }

    #[inline]
    pub fn available_monitors(&self) -> impl Iterator<Item = Monitor> {
        self.0.available_monitors().map(|m| Monitor(m))
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



#[derive(Clone, Copy, Debug)]
pub enum AppEvent<CustomEvent: 'static = ()> {
    Custom(CustomEvent),
    /// Called when this application's connection to the [`WindowManager`] is first established.
    ///
    /// This can be used to create an initial window.
    ///
    /// It should be noted that the application will also receive a resume event after this one.
    Init,
    /// Called when this application is suspended.
    ///
    /// There is no guarantee that an application will recieve suspend and resume events in any
    /// particular order.
    ///
    /// On certain platforms, this may never be called.
    Suspend,
    /// Called when this application is resumed.
    ///
    /// There is no guarantee that an application will recieve suspend and resume events in any
    /// particular order.
    ///
    /// On certain platforms, this may only be called when the [`WindowManager`] is first
    /// initialized. See [`AppEvent::Init`].
    Resume,
    /// Called when one of this application's windows receives a [`WindowEvent`].
    Window {
        /// The ID of the window targeted by this event.
        ///
        /// You can safely ignore this if your application only has one window.
        id: u64,
        event: WindowEvent,
    },
}

/// Applications (also known as *windowing clients*) can be registered through the
/// [`WindowingSystem`] and create windows through the [`WindowManager`]. Once registered, the
/// application will receive events from the windowing system that it can then use to perform some
/// behavior.
pub trait App {
    /// The parameter provided to [`AppEvent::Custom`].
    type CustomEvent: 'static;
    fn on_event(&mut self, wm: WindowManager, event: AppEvent<Self::CustomEvent>);
}

struct ClientProxy<'a, A: App<CustomEvent = E>, E: 'static> {
    client: &'a mut A,
}

impl<'a, A, E> winit::application::ApplicationHandler<E> for ClientProxy<'a, A, E>
where
    A: App<CustomEvent = E>,
    E: 'static,
{
    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        match cause {
            winit::event::StartCause::Init => {
                self.client.on_event(WindowManager { event_loop }, AppEvent::Init);
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client.on_event(WindowManager { event_loop }, AppEvent::Resume)
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.client.on_event(WindowManager { event_loop }, AppEvent::Suspend)
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: E) {
        self.client.on_event(WindowManager { event_loop }, AppEvent::Custom(event));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(event) = translate_window_event(event) {
            self.client.on_event(
                WindowManager { event_loop },
                AppEvent::Window { id: id.into(), event },
            );
        }
    }
}



fn translate_window_event(window_event: winit::event::WindowEvent) -> Option<WindowEvent> {
    match window_event {
        winit::event::WindowEvent::CloseRequested => Some(WindowEvent::CloseRequest),
        winit::event::WindowEvent::RedrawRequested => Some(WindowEvent::RedrawRequest),

        winit::event::WindowEvent::Resized(new_size) => {
            let (width, height) = new_size.into();
            Some(WindowEvent::Input(InputEvent::Resize { width, height }))
        }
        winit::event::WindowEvent::Focused(focused) => {
            Some(if focused {
                WindowEvent::Input(InputEvent::FocusIn)
            } else {
                WindowEvent::Input(InputEvent::FocusOut)
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
                        WindowEvent::Input(InputEvent::KeyDown {
                            code,
                            repeat,
                        })
                    } else {
                        WindowEvent::Input(InputEvent::KeyUp {
                            code,
                        })
                    })
                }
                winit::keyboard::PhysicalKey::Unidentified(_native_key_code) => {
                    // println!(
                    //     "[bog] TODO: Handle unknown native key codes, got {:?}.",
                    //     native_key_code,
                    // );
                    None
                }
            }
        }

        winit::event::WindowEvent::CursorMoved { position, .. } => {
            Some(WindowEvent::Input(InputEvent::MouseMove {
                x: position.x as _,
                y: position.y as _,
            }))
        }
        winit::event::WindowEvent::CursorEntered { .. } => {
            Some(WindowEvent::Input(InputEvent::MouseEnter))
        }
        winit::event::WindowEvent::CursorLeft { .. } => {
            Some(WindowEvent::Input(InputEvent::MouseLeave))
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            let button = translate_winit_mousebutton(button);
            Some(if state.is_pressed() {
                WindowEvent::Input(InputEvent::MouseDown { button })
            } else {
                WindowEvent::Input(InputEvent::MouseUp { button })
            })
        }
        // TODO: Handle touch inputs.
        winit::event::WindowEvent::MouseWheel { delta, phase: _, .. } => {
            let movement = match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                    WheelMovement::Lines { x, y }
                }
                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                    WheelMovement::Pixels { x: pos.x as _, y: pos.y as _ }
                }
            };
            Some(WindowEvent::Input(InputEvent::WheelMove(movement)))
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

const fn translate_winit_mousebutton(winit_button: winit::event::MouseButton) -> MouseButton {
    match winit_button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(n) => MouseButton::Other(n),
    }
}



/// A reference to the platform's windowing system.
///
/// The windowing system for a platform is responsible for creating, destroying, and positioning
/// windows. Sometimes (usually on mobile and web platforms), the windowing system is also
/// responsible for managing the "lifecycle" of its clients. This takes the form of resume,
/// suspend, and wakeup events that are passed to the clients managing the affected window(s).
///
/// To access a windowing system, you create an [`App`] object that will be capable of creating
/// windows through the [`WindowManager`] object.
pub struct WindowingSystem<E: 'static = ()> {
    event_loop: winit::event_loop::EventLoop<E>,
}

impl<E: 'static> WindowingSystem<E> {
    /// Create a new connection to the windowing system.
    pub fn new() -> Result<Self, WindowManagerError> {
        Ok(Self {
            event_loop: winit::event_loop::EventLoop::with_user_event().build()?,
        })
    }

    /// Create a [`WindowingSystemProxy`].
    pub fn create_proxy(&self) -> WindowingSystemProxy<E> {
        WindowingSystemProxy {
            event_loop: self.event_loop.create_proxy()
        }
    }

    /// Run the provided [`App`].
    pub fn run_app(self, app: &mut impl App<CustomEvent = E>) -> Result<(), WindowManagerError> {
        self.event_loop.run_app(&mut ClientProxy { client: app })
    }
}

/// A proxied reference to the platform's windowing system.
///
/// See [`WindowingSystem`] for more information.
#[derive(Clone)]
pub struct WindowingSystemProxy<E: 'static = ()> {
    event_loop: winit::event_loop::EventLoopProxy<E>,
}

impl<E: 'static> WindowingSystemProxy<E> {
    /// Send a custom event to the application.
    pub fn send(&self, event: E) -> bool {
        self.event_loop.send_event(event).is_ok()
    }
}



/// A reference to the platform's window manager.
///
/// Not to be confused with the [`WindowingSystem`], the window manager is responsible for the
/// actual window management behavior associated with a windowing system. You can think of this
/// object as a sort of "dispatcher" for requests to the system.
///
/// This window manager object is tied to the [`App`] you provided to the windowing system. Calling
/// [`WindowManager::exit`] will ask the windowing system to shutdown the application.
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
