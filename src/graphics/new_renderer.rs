//! New Renderer



use three_d::{context as gl, HasContext, Srgba, Vec2};



pub struct Painter2D {
    gl: three_d::Context,
    program: gl::Program,
    vao: gl::VertexArray,
    vbo: gl::Buffer,
    attrs: [(u32, i32, u32, i32, i32); 2],
    element_array_buffer: gl::Buffer,
}

impl Painter2D {
    pub fn new(gl: three_d::Context) -> Result<Self, String> {
        unsafe {
            let version = gl.get_parameter_string(gl::VERSION);
            let renderer = gl.get_parameter_string(gl::RENDERER);
            let vendor = gl.get_parameter_string(gl::VENDOR);
            println!(
                "\n[BOG] - INFO:\n\tOpenGl Version: {version}\n\tOpenGl Renderer: {renderer}\n\tOpenGl Vendor: {vendor}"
            );
        }

        #[cfg(not(target_arch = "wasm32"))]
        if gl.version().major < 2 {
            return Err("requires OpenGl 2.0+. ".to_string());
        }

        let header: &str = if gl.version().is_embedded {
            "#version 300 es
                #ifdef GL_FRAGMENT_PRECISION_HIGH
                    precision highp float;
                    precision highp int;
                    precision highp sampler2DArray;
                    precision highp sampler3D;
                #else
                    precision mediump float;
                    precision mediump int;
                    precision mediump sampler2DArray;
                    precision mediump sampler3D;
                #endif\n"
        } else {
            "#version 330 core\n"
        };
        let vertex_shader_source = format!("{}{}", header, include_str!("mesh2d.vert"));
        let fragment_shader_source = format!("{}{}", header, include_str!("mesh2d.frag"));

        unsafe {
            let vert_shader = gl
                .create_shader(three_d::context::VERTEX_SHADER)
                .expect("failed to create vertex shader");
            let frag_shader = gl
                .create_shader(three_d::context::FRAGMENT_SHADER)
                .expect("failed to create fragment shader");

            gl.shader_source(vert_shader, &vertex_shader_source);
            gl.shader_source(frag_shader, &fragment_shader_source);
            gl.compile_shader(vert_shader);
            gl.compile_shader(frag_shader);

            let program = gl.create_program().expect("failed to create program");
            gl.attach_shader(program, vert_shader);
            gl.attach_shader(program, frag_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                let log = gl.get_shader_info_log(vert_shader);
                if !log.is_empty() {
                    Err(log)?;
                }
                let log = gl.get_shader_info_log(frag_shader);
                if !log.is_empty() {
                    Err(log)?;
                }
                let log = gl.get_program_info_log(program);
                if !log.is_empty() {
                    Err(log)?;
                }
                Err("could not compile shader".to_string())?;
            }

            gl.detach_shader(program, vert_shader);
            gl.detach_shader(program, frag_shader);
            gl.delete_shader(vert_shader);
            gl.delete_shader(frag_shader);

            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer()?;
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(gl::ARRAY_BUFFER, Some(vbo));
            gl.bind_vertex_array(None);

            gl.error_check().unwrap();

            let pos_loc = gl.get_attrib_location(program, "a_pos").unwrap();
            // let uv_loc = gl.get_attrib_location(program, "a_uv").unwrap();
            let color_loc = gl.get_attrib_location(program, "a_color").unwrap();

            gl.error_check().unwrap();

            let stride = std::mem::size_of::<Vertex2D>() as i32;
            let attrs = [
                (pos_loc, 2, gl::FLOAT, stride, 0x0),
                (color_loc, 4, gl::FLOAT, stride, 0x8),
            ];
            // for (loc, vsize, dtype, stride, offset) in &attrs {
            //     gl.vertex_attrib_pointer_f32(
            //         *loc,
            //         *vsize,
            //         *dtype,
            //         false, // Normalized.
            //         *stride,
            //         *offset,
            //     );
            //     gl.enable_vertex_attrib_array(*loc);
            // }

            gl.error_check().unwrap();

            let element_array_buffer = gl.create_buffer()?;

            Ok(Self {
                gl,
                program,
                vao,
                vbo,
                attrs,
                element_array_buffer,
            })
        }
    }

    pub fn render<'a>(&mut self, width: i32, height: i32, meshes: impl Iterator<Item = &'a Mesh2D>) {
        unsafe {
            self.prepare(width, height);
        }
        self.paint_meshes(meshes);

        self.gl.error_check().unwrap();
    }

    pub unsafe fn prepare(&mut self, width_px: i32, height_px: i32) {
        unsafe {
            self.gl.enable(gl::SCISSOR_TEST);
            self.gl.disable(gl::CULL_FACE);
            self.gl.disable(gl::DEPTH_TEST);

            self.gl.error_check().unwrap();

            self.gl.color_mask(true, true, true, true);

            self.gl.enable(gl::BLEND);
            self.gl.blend_equation_separate(gl::FUNC_ADD, gl::FUNC_ADD);
            self.gl.blend_func_separate(
                gl::ONE,
                gl::ONE_MINUS_SRC_ALPHA,
                gl::ONE_MINUS_DST_ALPHA,
                gl::ONE,
            );

            // self.gl.disable(gl::FRAMEBUFFER_SRGB);

            self.gl.viewport(0, 0, width_px, height_px);
            self.gl.use_program(Some(self.program));

            self.gl.bind_buffer(gl::ARRAY_BUFFER, Some(self.vbo));
            self.gl.bind_vertex_array(Some(self.vao));
            for (loc, vsize, dtype, stride, offset) in &self.attrs {
                self.gl.vertex_attrib_pointer_f32(
                    *loc,
                    *vsize,
                    *dtype,
                    false, // Normalized.
                    *stride,
                    *offset,
                );
                self.gl.enable_vertex_attrib_array(*loc);
            }

            self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(self.element_array_buffer));
        }

        self.gl.error_check().unwrap();
    }

    pub fn paint_meshes<'a>(&self, meshes: impl Iterator<Item = &'a Mesh2D>) {
        for mesh in meshes {
            self.paint_mesh(mesh);
        }
        unsafe {
            self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
            self.gl.bind_buffer(gl::ARRAY_BUFFER, None);
            for (loc, _, _, _, _) in &self.attrs {
                self.gl.disable_vertex_attrib_array(*loc);
            }
            self.gl.bind_vertex_array(None);
            self.gl.disable(gl::SCISSOR_TEST);
        }
    }

    pub fn paint_mesh(&self, mesh: &Mesh2D) {
        unsafe {
            self.gl.bind_buffer(gl::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                gl::ARRAY_BUFFER,
                bytemuck::cast_slice(&mesh.vertices),
                gl::STREAM_DRAW,
            );

            self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(self.element_array_buffer));
            self.gl.buffer_data_u8_slice(
                gl::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&mesh.indices),
                gl::STREAM_DRAW,
            );
        }
        unsafe {
            self.gl.draw_elements(
                gl::TRIANGLES,
                mesh.indices.len() as i32,
                gl::UNSIGNED_INT,
                0_i32,
            );
        }
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
                let idx = out.vertices.len() as u32;
                out.add_triangle(idx + 0, idx + 1, idx + 2);
                out.add_triangle(idx + 2, idx + 1, idx + 3);

                let color = color.to_linear_srgb().into();
                out.vertices.push(Vertex2D {
                    pos: pos.into(),
                    color,
                });
                out.vertices.push(Vertex2D {
                    pos: [pos.x + size.x, pos.y],
                    color,
                });
                out.vertices.push(Vertex2D {
                    pos: [pos.x, pos.y + size.y],
                    color,
                });
                out.vertices.push(Vertex2D {
                    pos: [pos.x + size.x, pos.y + size.y],
                    color,
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct Mesh2D {
    indices: Vec<u32>,
    vertices: Vec<Vertex2D>,
}

impl Mesh2D {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            vertices: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }
}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2D {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}
