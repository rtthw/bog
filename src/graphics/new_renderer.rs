//! New Renderer



pub struct Mesh2D {
    context: three_d::Context,
    indices: three_d::IndexBuffer,
    positions: three_d::VertexBuffer<three_d::Vec2>,
    transform: three_d::Mat4,
    aabb: three_d::AxisAlignedBoundingBox,
}

impl Mesh2D {
    pub fn new(context: &three_d::Context, tri_mesh: &three_d::CpuMesh) -> Self {
        let aabb = tri_mesh.compute_aabb();

        Self {
            aabb,
        }
    }
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
