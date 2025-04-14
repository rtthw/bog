//! Renderer



use super::WindowGraphics;



pub const MAX_OBJECTS: u64 = 256;

pub struct Renderer {
    /// The uniform buffer for globals.
    global_buffer: wgpu::Buffer,
    /// The uniform buffer for object primitives.
    object_buffer: wgpu::Buffer,
}

impl Renderer {
    pub fn new(graphics: &WindowGraphics) -> Self {
        let global_buffer_size = std::mem::size_of::<Globals>() as u64;
        let global_buffer = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("bog::Renderer::global_buffer"),
            size: global_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let object_buffer_size = std::mem::size_of::<Object>() as u64 * MAX_OBJECTS;
        let object_buffer = graphics.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("bog::Renderer::object_buffer"),
            size: object_buffer_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            global_buffer,
            object_buffer,
        }
    }

    pub fn use_globals(&mut self, graphics: &WindowGraphics, globals: Globals) {
        graphics.queue().write_buffer(
            &self.global_buffer,
            0,
            bytemuck::cast_slice(&[globals]),
        );
    }

    pub fn use_objects(&mut self, graphics: &WindowGraphics, objects: &[Object]) {
        graphics.queue().write_buffer(
            &self.object_buffer,
            0,
            bytemuck::cast_slice(objects),
        );
    }
}



pub struct ObjectSet {
    objects: Vec<Object>,
    meshes: Vec<ObjectMesh>,
}

impl ObjectSet {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(MAX_OBJECTS as usize),
            meshes: Vec::with_capacity(MAX_OBJECTS as usize),
        }
    }

    pub fn get(&self, id: u32) -> Option<&Object> {
        self.objects.get(id as usize)
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> &[Object] {
        &self.objects[range]
    }

    pub fn push(&mut self, object: Object, mesh: ObjectMesh) {
        if self.objects.len() > MAX_OBJECTS as usize {
            return;
        }
        self.objects.push(object);
        self.meshes.push(mesh);
    }
}



pub struct ObjectMesh {
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
}



#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Globals {
    pub screen_size: [f32; 2],
    pub zoom: f32,
    pad_1: u32,
}

#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Object {
    pub position: [f32; 2],
    pub rotation: f32,
    pub z_index: i32,
    pub scale: f32,
    pub color: u32,
    pad_1: u32,
    pad_2: u32,
}

#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    /// Identifier for the object this vertex belongs to.
    pub object: u32,
    pub position: [f32; 2],
}

impl Vertex {
    pub const fn desc() -> &'static [wgpu::VertexAttribute] {
        &[
            wgpu::VertexAttribute {
                offset: 0,
                format: wgpu::VertexFormat::Uint32,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<u32>() as u64,
                format: wgpu::VertexFormat::Float32x2,
                shader_location: 1,
            },
        ]
    }
}
