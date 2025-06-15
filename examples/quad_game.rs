


use std::time::Instant;

use bog::prelude::*;



const WORLD_WIDTH: f32 = 1200.0;
const WORLD_HEIGHT: f32 = 900.0;
const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 40.0;
const BACKGROUND_COLOR: Color = Color::new(29, 29, 39, 255);
const TEXT_COLOR: Color = Color::new(163, 163, 173, 255);
const SHADOW_COLOR: Color = Color::new(13, 13, 23, 155);
const HEADER_TEXT_SIZE: f32 = 30.0;


fn main() -> Result<()> {
    run_simple_app(Game {
        menu: None,
        last_frame_time: Instant::now(),
        player_pos: vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
        player_move: None,
        player_speed: 100.0,
        mouse_pos: Vec2::ZERO,
    })
}



struct Game {
    menu: Option<Menu>,
    last_frame_time: Instant,
    player_pos: Vec2,
    player_move: Option<Vec2>,
    player_speed: f32,
    mouse_pos: Vec2,
}

impl SimpleApp for Game {
    fn render(&mut self, cx: AppContext, pass: &mut RenderPass) {
        cx.window.request_redraw();

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f64();
        self.last_frame_time = now;

        let screen_rect = cx.renderer.viewport_rect();

        if let Some(move_target) = self.player_move {
            if self.player_pos.distance(move_target) > 5.0 {
                self.player_pos = self.player_pos
                    .move_towards(move_target, dt as f32 * self.player_speed);
            } else {
                self.player_move = None;
            }
        }

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
                    self.player_pos - vec2(PLAYER_WIDTH / 2.0, PLAYER_HEIGHT / 2.0),
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

        // Overlay.
        {
            pass.start_layer(screen_rect);
            if let Some(menu) = self.menu {
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
                                    (WORLD_WIDTH / 2.0) - (header_text_bounds.x / 2.0),
                                    (WORLD_HEIGHT / 2.0) - (header_text_bounds.y / 2.0),
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
                self.mouse_pos = vec2(x, y);
            }
            InputEvent::MouseDown { button } => match button {
                MouseButton::Left => {}
                MouseButton::Right => {
                    if self.menu.is_none() {
                        self.player_move = Some(self.mouse_pos);
                    }
                }
                _ => {}
            }
            InputEvent::KeyDown { code, .. } => match code {
                KeyCode::C_SPACE => {
                    self.menu = match self.menu {
                        Some(Menu::Pause) => None,
                        None => Some(Menu::Pause),
                        other => other,
                    };
                }
                KeyCode::C_ESCAPE => {
                    self.menu = Some(Menu::Start);
                }
                _ => {}
            }
            _ => {}
        }
    }

    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Bog - Quad Game Example",
            inner_size: Vec2::new(WORLD_WIDTH, WORLD_HEIGHT),
            ..Default::default()
        }
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Menu {
    Start, // TODO
    Pause,
}
