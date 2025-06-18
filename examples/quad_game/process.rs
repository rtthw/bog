


use bog_core::Vec2;

use crate::state::State;



#[allow(unused)]
pub trait Process {
    fn update(&mut self, state: &mut State, dt: f64) {}
}



pub struct PlayerMovement {
    pub target: Option<Vec2>,
}

impl Process for PlayerMovement {
    fn update(&mut self, state: &mut State, dt: f64) {
        if state.input.mouse_buttons_down.right() {
            self.target = Some(state.input.mouse_pos);
        }
        if let Some(move_target) = self.target {
            if state.player.position.distance(move_target) > 5.0 {
                state.player.position = state.player.position
                    .move_towards(move_target, dt as f32 * state.player.move_speed);
            } else {
                self.target = None;
            }
        }
    }
}
