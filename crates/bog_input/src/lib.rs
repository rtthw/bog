//! Input handling



pub trait InputHandler {
    fn on_input(&mut self, input: Input);
}

pub enum Input {}
