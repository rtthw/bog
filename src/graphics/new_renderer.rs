//! New Renderer



use three_d::{vec2, Mat3, SquareMatrix, Srgba, Vec2, Vec4};



pub struct Painter2D {
    program: three_d::Program,
    positions: three_d::VertexBuffer<Vec2>,
    colors: three_d::VertexBuffer<Vec4>,
    elements: three_d::ElementBuffer<u32>,
}

impl Painter2D {
    pub fn new(gl: three_d::Context) -> Self {
        let program = three_d::Program::from_source(
            &gl,
            include_str!("shaders/vert2d.glsl"),
            include_str!("shaders/frag2d.glsl"),
        ).unwrap();
        let positions = three_d::VertexBuffer::new(&gl);
        let colors = three_d::VertexBuffer::new(&gl);
        let elements = three_d::ElementBuffer::new(&gl);

        Self {
            program,
            positions,
            colors,
            elements,
        }
    }

    pub fn render(&mut self, viewport: three_d::Viewport, mesh: &Mesh2D) {
        self.positions.fill(&mesh.positions);
        self.colors.fill(&mesh.colors);
        self.elements.fill(&mesh.indices);
        self.program.use_uniform("u_screen_size", vec2(viewport.width as f32, viewport.height as f32));
        self.program.use_vertex_attribute("a_pos", &self.positions);
        self.program.use_vertex_attribute("a_color", &self.colors);
        self.program.draw_elements(three_d::RenderStates::default(), viewport, &self.elements);
    }
}



pub enum Shape {
    Rect {
        pos: Vec2,
        size: Vec2,
        color: Srgba,
    },
}

pub struct Tessellator;

impl Tessellator {
    pub fn tessellate_shape(&mut self, shape: Shape, out: &mut Mesh2D) {
        match shape {
            Shape::Rect { pos, size, color } => {
                let idx = out.positions.len() as u32;
                out.add_triangle(idx + 0, idx + 1, idx + 2);
                out.add_triangle(idx + 2, idx + 1, idx + 3);

                let color = color.to_linear_srgb();
                out.colors.extend([color].repeat(4));
                out.positions.push(pos.into());
                out.positions.push(vec2(pos.x + size.x, pos.y));
                out.positions.push(vec2(pos.x, pos.y + size.y));
                out.positions.push(vec2(pos.x + size.x, pos.y + size.y));
            }
        }
    }
}



pub struct Transform2D {
    pub(crate) inner: Mat3,
}

impl Transform2D {
    pub fn set(&mut self, transform: Mat3) {
        self.inner = transform;
    }
}



pub struct Bounds2D {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

impl Bounds2D {
    pub const EMPTY: Self = Self {
        min: vec2(0.0, 0.0),
        max: vec2(0.0, 0.0),
    };

    pub fn new(positions: &[Vec2]) -> Self {
        if let Some(first_pos) = positions.first() {
            let mut min = *first_pos;
            let mut max = *first_pos;

            for p in positions {
                min.x = min.x.min(p.x);
                min.y = min.y.min(p.y);

                max.x = max.x.max(p.x);
                max.y = max.y.max(p.y);
            }

            Self {
                min,
                max,
            }
        } else {
            Self::EMPTY
        }
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn top_left(&self) -> Vec2 {
        self.min
    }

    pub fn top_right(&self) -> Vec2 {
        vec2(self.max.x, self.min.y)
    }

    pub fn bottom_left(&self) -> Vec2 {
        vec2(self.min.x, self.max.y)
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.max
    }
}



/// Representation of a 2D object with vertex position and color data.
///
/// See [`Wireframe2D`] for a minimal version of this with no color data.
#[derive(Clone, Debug)]
pub struct Mesh2D {
    pub(crate) indices: Vec<u32>,
    pub(crate) positions: Vec<Vec2>,
    pub(crate) colors: Vec<Vec4>,
}

impl Mesh2D {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            positions: Vec::new(),
            colors: Vec::new(),
        }
    }

    /// See also: [`Wireframe2D::to_mesh`].
    pub fn from_wireframe(wireframe: Wireframe2D, color: Srgba) -> Self {
        let colors = [color.to_linear_srgb().into()].repeat(wireframe.indices.len());

        Self {
            indices: wireframe.indices,
            positions: wireframe.positions,
            colors,
        }
    }

    pub fn compute_info(&self) -> (Vec2, Vec2, Vec2) {
        if let Some(first_pos) = self.positions.first() {
            let mut min_pos = *first_pos;
            let mut max_pos = *first_pos;

            for p in &self.positions {
                min_pos.x = min_pos.x.min(p.x);
                min_pos.y = min_pos.y.min(p.y);

                max_pos.x = max_pos.x.max(p.x);
                max_pos.y = max_pos.y.max(p.y);
            }

            (max_pos - min_pos, min_pos, max_pos)
        } else {
            (vec2(0.0, 0.0), vec2(0.0, 0.0), vec2(0.0, 0.0))
        }
    }

    #[inline(always)]
    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }

    pub fn append(&mut self, other: Self) {
        if self.indices.is_empty() {
            *self = other;
        } else {
            self.append_ref(&other);
        }
    }

    pub fn append_ref(&mut self, other: &Self) {
        let index_offset = self.positions.len() as u32;
        self.indices.extend(other.indices.iter().map(|index| index + index_offset));
        self.positions.extend(other.positions.iter());
        self.colors.extend(other.colors.iter());
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        for pos in &mut self.positions {
            pos[0] += x;
            pos[1] += y;
        }
    }

    pub fn invert_y(&mut self) {
        for pos in &mut self.positions {
            pos[1] *= -1.0;
        }
    }

    pub fn change_color(&mut self, color: Srgba) {
        self.colors.fill(color.to_linear_srgb());
    }
}



/// A minimal representation of a 2D object.
///
/// Can be easily transformed to (and from) a [`Mesh2D`].
#[derive(Clone, Debug)]
pub struct Wireframe2D {
    pub(crate) indices: Vec<u32>,
    pub(crate) positions: Vec<Vec2>,
}

impl Wireframe2D {
    /// See also: [`Mesh2D::from_wireframe`].
    pub fn to_mesh(self, color: Srgba) -> Mesh2D {
        let colors = [color.to_linear_srgb()].repeat(self.indices.len());

        Mesh2D {
            indices: self.indices,
            positions: self.positions,
            colors,
        }
    }
}

impl From<Mesh2D> for Wireframe2D {
    fn from(value: Mesh2D) -> Self {
        Self {
            indices: value.indices,
            positions: value.positions,
        }
    }
}
