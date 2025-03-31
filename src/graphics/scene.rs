//! Scene



use three_d::{ColorMaterial, Srgba};

use super::{mesh::Mesh, Render};



/// A scene is just a set of objects ready to be rendered.
#[derive(Default)]
pub struct Scene {
    geometries: Vec<Mesh>,
    materials: Vec<ColorMaterial>,
}

impl Render for Scene {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object> {
        self.geometries.iter().zip(self.materials.iter())
            .map(|(g, m)| three_d::Gm::new(&g.inner, m))
    }
}

// Object management.
impl Scene {
    pub fn geometries(&mut self) -> &mut Vec<Mesh> {
        &mut self.geometries
    }

    pub fn materials(&mut self) -> &mut Vec<ColorMaterial> {
        &mut self.materials
    }

    pub fn geometry(&mut self, id: usize) -> Option<&mut Mesh> {
        self.geometries.get_mut(id)
    }

    pub fn material(&mut self, id: usize) -> Option<&mut ColorMaterial> {
        self.materials.get_mut(id)
    }

    pub fn append(&mut self, g: Mesh, m: ColorMaterial) -> usize {
        let id = self.geometries.len();

        self.geometries.push(g);
        self.materials.push(m);

        id
    }
}



pub trait Scenic {
    fn mesh_mut(&mut self, id: usize) -> Option<&mut Mesh>;
    fn color_mut(&mut self, id: usize) -> Option<&mut Srgba>;
}
