//! "Drawing"-style graphics



use three_d::{CpuMesh, Mat3, Mat4, Srgba};

use super::{mesh::Mesh, Renderer};



pub struct Painter<'a> {
    pub paints: Vec<Paint<'a>>,
}

impl<'a> Painter<'a> {
    pub fn finish(self, renderer: &Renderer) {
        for paint in self.paints {
            match paint {
                Paint::Group(nested_paints) => {
                    for nested_paint in nested_paints {
                        match nested_paint {
                            Paint::ColoredShape(shape, color) => {
                                let Some(mesh) = mesh_for_shape(shape, renderer) else {
                                    println!("ERROR: Could not generate mesh for shape");
                                    continue;
                                };
                            }
                            Paint::Group(_) => panic!("Painter encountered nested paint group"),
                        }
                    }
                }
                Paint::ColoredShape(shape, color) => {}
            }
        }
    }
}

fn mesh_for_shape(shape: Shape, renderer: &Renderer) -> Option<Mesh> {
    match shape {
        Shape::Rectangle { transform } => {
            let mut cpu_mesh = CpuMesh::square();
            cpu_mesh.transform(Mat4::from_scale(0.5)).unwrap();
            let mut mesh = Mesh::new(renderer, &cpu_mesh);
            mesh.transform_2d(transform);

            Some(mesh)
        }
        Shape::Glyph { transform, font, id } => {
            let mut mesh = renderer.mesh_for_glyph(font, id)?;
            mesh.transform_2d(transform);

            Some(mesh)
        }
    }
}



pub enum Paint<'a> {
    /// A set of ordered paint commands.
    ///
    /// Groups must not contain other groups. The [`Painter`] will panic if it encounters a
    /// nested group.
    Group(Vec<Paint<'a>>),
    ColoredShape(Shape<'a>, Srgba),
}

pub enum Shape<'a> {
    Rectangle {
        transform: Mat3,
    },
    Glyph {
        transform: Mat3,
        font: &'a str,
        id: u16,
    },
}
