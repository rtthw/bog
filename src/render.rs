//! Rendering



pub extern crate three_d;



#[derive(Clone)]
pub struct Renderer(pub(crate) three_d::Context);

impl std::ops::Deref for Renderer {
    type Target = three_d::Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
