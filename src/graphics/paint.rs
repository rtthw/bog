//! "Drawing"-style graphics



use std::{collections::HashMap, ops::Range};

use three_d::{CpuMesh, Mat3, Mat4, Srgba};

use super::{mesh::Mesh, Render, Renderer};



pub struct Painter<'a> {
    pub paints: Vec<Paint<'a>>,
    pub current_group: Option<Vec<Paint<'a>>>,
}

impl<'a> Painter<'a> {
    pub fn start_group(&mut self) {
        self.current_group = Some(Vec::new());
    }

    pub fn end_group(&mut self) -> Option<usize> {
        if let Some(group) = self.current_group.take() {
            let id = self.paints.len();
            self.paints.push(Paint::Group(id, group));
            Some(id)
        } else {
            None
        }
    }

    pub fn finish(self, renderer: &Renderer) -> Painting {
        let mut painting = Painting::default();
        let mut id: usize = 0;
        for paint in self.paints {
            match paint {
                Paint::Group(group_id, nested_paints) => {
                    if nested_paints.is_empty() {
                        continue;
                    }

                    let group_start_id = id;
                    let mut group_end_id = id;
                    'inner: for nested_paint in nested_paints {
                        match nested_paint {
                            Paint::ColoredShape(shape, color) => {
                                let Some(mesh) = mesh_for_shape(shape, renderer) else {
                                    println!("ERROR: Could not generate mesh for shape");
                                    continue 'inner;
                                };
                                painting.meshes.push(mesh);
                                painting.colors.push(color);
                            }
                            Paint::Group(_, _) => panic!("encountered nested paint group"),
                        }
                        group_end_id = id;
                        id += 1;
                    }

                    painting.groups.insert(group_id, Range {
                        start: group_start_id,
                        end: group_end_id,
                    });
                }
                Paint::ColoredShape(shape, color) => {
                    let Some(mesh) = mesh_for_shape(shape, renderer) else {
                        println!("ERROR: Could not generate mesh for shape");
                        continue;
                    };
                    painting.meshes.push(mesh);
                    painting.colors.push(color);
                    id += 1;
                }
            }
        }

        painting
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
        Shape::Text { transform, font, text } => {
            let mut mesh = renderer.mesh_for_text(font, text, None)?;
            mesh.transform_2d(transform);

            Some(mesh)
        }
    }
}

impl<'a> Painter<'a> {
    pub fn push_paint(&mut self, paint: Paint<'a>) {
        if let Some(paints) = &mut self.current_group {
            paints.push(paint);
        } else {
            self.paints.push(paint);
        }
    }

    pub fn rectangle(&mut self, transform: Mat3, color: Srgba) {
        self.push_paint(Paint::ColoredShape(Shape::Rectangle { transform }, color));
    }

    pub fn glyph(&mut self, transform: Mat3, font: &'a str, id: u16, color: Srgba) {
        self.push_paint(Paint::ColoredShape(Shape::Glyph { transform, font, id }, color));
    }

    pub fn text(&mut self, transform: Mat3, font: &'a str, text: &'a str, color: Srgba) {
        self.push_paint(Paint::ColoredShape(Shape::Text { transform, font, text }, color));
    }
}



#[derive(Default)]
pub struct Painting {
    meshes: Vec<Mesh>,
    colors: Vec<Srgba>,
    groups: HashMap<usize, Range<usize>>,
}

impl Render for Painting {
    fn objects(&self) -> impl Iterator<Item = impl three_d::Object> {
        self.meshes.iter().zip(self.colors.iter())
            .map(|(mesh, color)| {
                three_d::Gm::new(
                    &mesh.inner,
                    three_d::ColorMaterial {
                        color: *color,
                        ..Default::default()
                    },
                )
            })
    }
}



pub enum Paint<'a> {
    /// A set of ordered paint commands.
    ///
    /// Groups must not contain other groups. The [`Painter`] will panic if it encounters a
    /// nested group.
    Group(usize, Vec<Paint<'a>>),
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
    Text {
        transform: Mat3,
        font: &'a str,
        text: &'a str,
    },
}
