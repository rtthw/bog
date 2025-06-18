


use bog_core::{MouseButtonMask, Vec2};



pub struct State {
    pub input: InputState,
    pub player: PlayerState,
}

pub struct InputState {
    pub mouse_pos: Vec2,
    pub mouse_buttons_down: MouseButtonMask,
}

pub struct PlayerState {
    pub position: Vec2,
    pub move_speed: f32,
}
