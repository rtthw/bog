//! Renderer



pub struct Renderer {}



#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectPrimitive {
    pub transform: [f32; 2],
    pub rotation: f32,
    pub z_index: i32,
    pub color: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectVertex {
    pub primitive: u32,
    pub position: [f32; 2],
}
