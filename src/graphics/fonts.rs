//! Font management



use std::collections::HashMap;



#[derive(thiserror::Error, Debug)]
pub enum Error {

}



#[derive(Clone, Default)]
pub struct Fonts {
    map: HashMap<String, Font>,
}

impl Fonts {
    pub fn load_font(&mut self, name: &str, bytes: &[u8], size: f32) -> Result<(), Error> {
        let font = Font::load(bytes, size)?;
        let _ = self.map.insert(name.to_string(), font);
        Ok(())
    }
}



#[derive(Clone)]
pub struct Font {}

impl Font {
    pub fn load(bytes: &[u8], size: f32) -> Result<Self, Error> {
        todo!()
    }
}
