//! New Renderer



use three_d::{Mat3, Mat4, SquareMatrix as _, Vec2};



pub struct Mesh2D {
    context: three_d::Context,
    indices: three_d::IndexBuffer,
    positions: three_d::VertexBuffer<three_d::Vec2>,
    transform: three_d::Mat4,
    aabb: three_d::AxisAlignedBoundingBox,
}

impl three_d::Geometry for Mesh2D {
    fn aabb(&self) -> three_d::AxisAlignedBoundingBox {
        self.aabb.transformed(self.transform)
    }

    fn draw(
        &self,
        viewer: &dyn three_d::Viewer,
        program: &three_d::Program,
        render_states: three_d::RenderStates,
    ) {
        program.use_uniform("view_projection", viewer.projection() * viewer.view());
        program.use_uniform("model", self.transform);
        program.use_vertex_attribute("position", &self.positions);

        match &self.indices {
            three_d::IndexBuffer::None => program.draw_arrays(
                render_states,
                viewer.viewport(),
                self.positions.vertex_count(),
            ),
            three_d::IndexBuffer::U8(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer)
            }
            three_d::IndexBuffer::U16(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer)
            }
            three_d::IndexBuffer::U32(element_buffer) => {
                program.draw_elements(render_states, viewer.viewport(), element_buffer)
            }
        }
    }

    fn vertex_shader_source(&self) -> String {
        include_str!("mesh2d.vert").to_string()
    }

    fn id(&self) -> three_d::GeometryId {
        three_d::GeometryId(1)
    }

    fn render_with_material(
        &self,
        material: &dyn three_d::Material,
        viewer: &dyn three_d::Viewer,
        lights: &[&dyn three_d::Light],
    ) {
        three_d::render_with_material(&self.context, viewer, self, material, lights);
    }

    fn render_with_effect(
        &self,
        effect: &dyn three_d::Effect,
        viewer: &dyn three_d::Viewer,
        lights: &[&dyn three_d::Light],
        color_texture: Option<three_d::ColorTexture>,
        depth_texture: Option<three_d::DepthTexture>,
    ) {
        three_d::render_with_effect(
            &self.context,
            viewer,
            self,
            effect,
            lights,
            color_texture,
            depth_texture,
        );
    }
}



pub trait ToMesh2D {
    fn to_mesh2d(self, context: &three_d::Context) -> Mesh2D;
}



/// A heavily optimized [`Mesh`] for rendering rectangles.
pub struct RectMesh2D {
    positions: Vec<Vec2>,
    transform: Mat3,
}

impl RectMesh2D {
    pub fn new() -> Self {
        let mut rect = Self::square();
        rect.transform = Mat3::identity() * Mat3::from_scale(0.5);
        rect
    }

    pub fn square() -> Self {
        let positions = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];

        Self {
            positions,
            transform: Mat3::identity(),
        }
    }

    pub const fn indices() -> [u8; 6] {
        [0, 1, 2, 2, 3, 0]
    }
}

impl ToMesh2D for RectMesh2D {
    fn to_mesh2d(self, context: &three_d::Context) -> Mesh2D {
        let mut aabb = three_d::AxisAlignedBoundingBox::EMPTY;
        aabb.expand(&[three_d::Vec3::new(1.0, 1.0, 0.0), three_d::Vec3::new(-1.0, -1.0, 0.0)]);

        Mesh2D {
            context: context.clone(),
            indices: three_d::IndexBuffer::None,
            positions: three_d::VertexBuffer::new_with_data(context, &self.positions),
            transform: Mat4::new(
                self.transform.x.x,
                self.transform.x.y,
                0.0,
                self.transform.x.z,
                self.transform.y.x,
                self.transform.y.y,
                0.0,
                self.transform.y.z,
                0.0,
                0.0,
                1.0,
                0.0,
                self.transform.z.x,
                self.transform.z.y,
                0.0,
                self.transform.z.z,
            ),
            aabb,
        }
    }
}
