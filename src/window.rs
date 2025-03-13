//! Window Management



use env::*;



/// A handle to a window.
#[derive(Debug)]
pub struct Window {}

pub fn create(env: &Connection) -> Result<Window> {
    Ok(Window {})
}
