//! Graphics module



pub struct GraphicsCard {
    pub(crate) device: wgpu::Device,
}



pub struct ShaderDescriptor<'a> {
    pub label: Option<&'a str>,
}
