


mod process;
mod state;

use std::time::Instant;

use bog::prelude::*;

use process::*;
use state::*;



const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 40.0;
const BACKGROUND_COLOR: Color = Color::new(29, 29, 39, 255);
const TEXT_COLOR: Color = Color::new(163, 163, 173, 255);
const SHADOW_COLOR: Color = Color::new(13, 13, 23, 155);
const HEADER_TEXT_SIZE: f32 = 30.0;


fn main() -> Result<()> {
    run_simple_app(None, App {
        state: State {
            screen_size: vec2(1280.0, 720.0),
            screen_offset: Vec2::ZERO,
            input: InputState {
                mouse_pos: Vec2::ZERO,
                mouse_buttons_down: MouseButtonMask::empty(),
            },
            player: PlayerState {
                position: Vec2::ZERO,
                move_speed: 100.0,
            },
        },
        process: Game {
            menu: None,
            player_movement: PlayerMovement {
                target: None,
            },
        },
        last_frame_time: Instant::now(),
    })
}



struct App {
    state: State,
    process: Game,
    last_frame_time: Instant,
}

impl SimpleApp for App {
    type CustomEvent = ();

    fn render(&mut self, cx: AppContext, pass: &mut RenderPass) {
        cx.window.request_redraw();

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f64();
        self.last_frame_time = now;

        let screen_rect = cx.renderer.viewport_rect();
        self.state.screen_size = screen_rect.size();
        self.state.screen_offset = vec2(
            (self.state.screen_size.x / 2.0) - self.state.player.position.x,
            (self.state.screen_size.y / 2.0) - self.state.player.position.y,
        );

        self.process.update(&mut self.state, dt);

        pass.start_transform(Mat4::from_translation(vec3(
            self.state.screen_offset.x,
            self.state.screen_offset.y,
            0.0,
        )));

        // Background.
        {
            pass.start_layer(screen_rect);
            pass.fill_quad(Quad {
                bounds: screen_rect,
                bg_color: BACKGROUND_COLOR,
                ..Default::default()
            });
            pass.end_layer();
        }

        // Game objects.
        {
            pass.start_layer(screen_rect);
            pass.fill_quad(Quad {
                bounds: Rect::new(
                    self.state.player.position - vec2(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
                    vec2(PLAYER_WIDTH, PLAYER_HEIGHT),
                ),
                bg_color: Color::new(163, 163, 173, 255),
                border: Border {
                    radius: [7.0, 7.0, 2.0, 3.0],
                    ..Default::default()
                },
                shadow: Shadow {
                    color: SHADOW_COLOR,
                    offset: vec2(0.0, PLAYER_HEIGHT / 4.0),
                    blur_radius: 17.0,
                },
                ..Default::default()
            });
            pass.end_layer();
        }

        pass.end_transform();

        // Overlay.
        {
            pass.start_layer(screen_rect);
            if let Some(menu) = self.process.menu {
                match menu {
                    Menu::Pause => {
                        pass.fill_quad(Quad {
                            bounds: screen_rect,
                            bg_color: SHADOW_COLOR,
                            ..Default::default()
                        });
                        // Ensure text is rendered above blur.
                        pass.start_layer(screen_rect);
                        let header_text_bounds = cx.renderer.measure_text(&Text {
                            content: "PAUSED".into(),
                            bounds: screen_rect,
                            size: HEADER_TEXT_SIZE,
                            ..Default::default()
                        });
                        pass.fill_text(Text {
                            content: "PAUSED".into(),
                            bounds: Rect::new(
                                vec2(
                                    (self.state.screen_size.x / 2.0) - (header_text_bounds.x / 2.0),
                                    (self.state.screen_size.y / 2.0) - (header_text_bounds.y / 2.0),
                                ),
                                header_text_bounds,
                            ),
                            color: TEXT_COLOR,
                            size: HEADER_TEXT_SIZE,
                            ..Default::default()
                        });
                        pass.end_layer();
                    }
                    Menu::Start => {
                        pass.fill_quad(Quad {
                            bounds: screen_rect,
                            bg_color: BACKGROUND_COLOR,
                            ..Default::default()
                        });
                    }
                }
            }
            pass.end_layer();
        }
    }

    fn input(&mut self, _cx: AppContext, event: InputEvent) {
        match event {
            InputEvent::MouseMove { x, y } => {
                self.state.input.mouse_pos = vec2(x, y);
            }
            InputEvent::MouseDown { button } => match button {
                MouseButton::Left => {
                    self.state.input.mouse_buttons_down.insert(MouseButtonMask::LEFT);
                }
                MouseButton::Right => {
                    self.state.input.mouse_buttons_down.insert(MouseButtonMask::RIGHT);
                }
                MouseButton::Middle => {
                    self.state.input.mouse_buttons_down.insert(MouseButtonMask::LEFT);
                }
                _ => {}
            }
            InputEvent::MouseUp { button } => match button {
                MouseButton::Left => {
                    self.state.input.mouse_buttons_down.remove(MouseButtonMask::LEFT);
                }
                MouseButton::Right => {
                    self.state.input.mouse_buttons_down.remove(MouseButtonMask::RIGHT);
                }
                MouseButton::Middle => {
                    self.state.input.mouse_buttons_down.remove(MouseButtonMask::LEFT);
                }
                _ => {}
            }
            InputEvent::KeyDown { code, .. } => match code {
                KeyCode::C_SPACE => {
                    self.process.menu = match self.process.menu {
                        Some(Menu::Pause) => None,
                        None => Some(Menu::Pause),
                        other => other,
                    };
                }
                KeyCode::C_ESCAPE => {
                    self.process.menu = Some(Menu::Start);
                }
                _ => {}
            }
            _ => {}
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Quad Game Example",
            maximized: true,
            ..Default::default()
        }
    }
}



struct Game {
    menu: Option<Menu>,
    player_movement: PlayerMovement,
}

impl Process for Game {
    fn update(&mut self, state: &mut State, dt: f64) {
        if self.menu.is_none() {
            self.player_movement.update(state, dt);
        }
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Menu {
    Start, // TODO
    Pause,
}
