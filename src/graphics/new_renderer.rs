//! New Renderer



use three_d::{vec2, Srgba, Vec2};



pub struct Painter2D {
    program: three_d::Program,
    positions: three_d::VertexBuffer<[f32; 2]>,
    colors: three_d::VertexBuffer<[f32; 4]>,
    elements: three_d::ElementBuffer<u32>,
}

impl Painter2D {
    pub fn new(gl: three_d::Context) -> Self {
        let program = three_d::Program::from_source(
            &gl,
            include_str!("mesh2d.vert"),
            include_str!("mesh2d.frag"),
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

                let color: [f32; 4] = color.to_linear_srgb().into();
                out.colors.extend([color].repeat(4));
                out.positions.push(pos.into());
                out.positions.push([pos.x + size.x, pos.y]);
                out.positions.push([pos.x, pos.y + size.y]);
                out.positions.push([pos.x + size.x, pos.y + size.y]);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh2D {
    pub(crate) indices: Vec<u32>,
    pub(crate) positions: Vec<[f32; 2]>,
    pub(crate) colors: Vec<[f32; 4]>,
}

impl Mesh2D {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            positions: Vec::new(),
            colors: Vec::new(),
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
}

#[derive(Clone, Debug)]
pub struct Wireframe2D {
    pub(crate) indices: Vec<u32>,
    pub(crate) positions: Vec<[f32; 2]>,
}
