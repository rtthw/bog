


use std::sync::Arc;

use bog_core::{Mat4, Rect};

use crate::{buffer::Buffer, Image, ImageFilterMethod, ImageHandle};



pub struct ImageManager {
    pub layers: Vec<ImageLayer>,
    pub prepare_layer: usize,
}

impl ImageManager {
    pub fn new() -> Self {
        Self {
            layers: Vec::with_capacity(3),
            prepare_layer: 0,
        }
    }

    pub fn prepare(
        &mut self,
        pipeline: &ImagePipeline,
        device: &wgpu::Device,
        belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        cache: &mut ImageCache,
        images: &[Image],
        transform: Mat4,
        scale: f32,
    ) {
        let nearest_instances: &mut Vec<Instance> = &mut Vec::new();
        let linear_instances: &mut Vec<Instance> = &mut Vec::new();

        for image in images {
            match &image {
                Image::Raster(image, bounds) => {
                    if let Some(atlas_entry) =
                        cache.upload_raster(device, encoder, &image.handle)
                    {
                        add_instances(
                            [bounds.x, bounds.y],
                            [bounds.w, bounds.h],
                            image.rotation,
                            image.opacity,
                            image.snap,
                            atlas_entry,
                            match image.filter_method {
                                ImageFilterMethod::Nearest => {
                                    nearest_instances
                                }
                                ImageFilterMethod::Linear => {
                                    linear_instances
                                }
                            },
                        );
                    }
                }
                // TODO: SVG
            }
        }

        if nearest_instances.is_empty() && linear_instances.is_empty() {
            return;
        }

        if self.layers.len() <= self.prepare_layer {
            self.layers.push(ImageLayer::new(
                device,
                &pipeline.constant_layout,
                &pipeline.nearest_sampler,
                &pipeline.linear_sampler,
            ));
        }

        let layer = &mut self.layers[self.prepare_layer];

        layer.prepare(
            device,
            encoder,
            belt,
            nearest_instances,
            linear_instances,
            transform,
            scale,
        );

        self.prepare_layer += 1;
    }

    pub fn render<'a>(
        &'a self,
        pipeline: &'a ImagePipeline,
        cache: &'a ImageCache,
        layer: usize,
        bounds: Rect<u32>,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        if let Some(layer) = self.layers.get(layer) {
            render_pass.set_pipeline(&pipeline.raw);

            render_pass.set_scissor_rect(
                bounds.x,
                bounds.y,
                bounds.w,
                bounds.h,
            );

            render_pass.set_bind_group(1, cache.bind_group(), &[]);

            layer.render(render_pass);
        }
    }

    pub fn cleanup(&mut self) {
        self.prepare_layer = 0;
    }
}



pub struct ImageLayer {
    uniforms: wgpu::Buffer,
    nearest: Data,
    linear: Data,
}

impl ImageLayer {
    fn new(
        device: &wgpu::Device,
        constant_layout: &wgpu::BindGroupLayout,
        nearest_sampler: &wgpu::Sampler,
        linear_sampler: &wgpu::Sampler,
    ) -> Self {
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bog::uniforms_buffer::image"),
            size: core::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let nearest = Data::new(device, constant_layout, nearest_sampler, &uniforms);
        let linear = Data::new(device, constant_layout, linear_sampler, &uniforms);

        Self {
            uniforms,
            nearest,
            linear,
        }
    }

    fn prepare(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        nearest_instances: &[Instance],
        linear_instances: &[Instance],
        transform: Mat4,
        scale_factor: f32,
    ) {
        let uniforms = Uniforms {
            transform: *transform.as_ref(),
            scale_factor,
            _padding: [0.0; 3],
        };

        let bytes = bytemuck::bytes_of(&uniforms);

        belt.write_buffer(
            encoder,
            &self.uniforms,
            0,
            (bytes.len() as u64).try_into().expect("Sized uniforms"),
            device,
        )
        .copy_from_slice(bytes);

        self.nearest.upload(device, encoder, belt, nearest_instances);
        self.linear.upload(device, encoder, belt, linear_instances);
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.nearest.render(render_pass);
        self.linear.render(render_pass);
    }
}

struct Data {
    constants: wgpu::BindGroup,
    instances: Buffer<Instance>,
    instance_count: usize,
}

impl Data {
    pub fn new(
        device: &wgpu::Device,
        constant_layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        uniforms: &wgpu::Buffer,
    ) -> Self {
        let constants = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bog::bind_group::image"),
            layout: constant_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        wgpu::BufferBinding {
                            buffer: uniforms,
                            offset: 0,
                            size: None,
                        },
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        let instances = Buffer::new(
            device,
            "bog::instance_buffer::image",
            Instance::INITIAL,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        Self {
            constants,
            instances,
            instance_count: 0,
        }
    }

    fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        instances: &[Instance],
    ) {
        self.instance_count = instances.len();

        if self.instance_count == 0 {
            return;
        }

        let _ = self.instances.resize(device, instances.len());
        let _ = self.instances.write(device, encoder, belt, 0, instances);
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count == 0 {
            return;
        }

        render_pass.set_bind_group(0, &self.constants, &[]);
        render_pass.set_vertex_buffer(0, self.instances.slice(..));

        render_pass.draw(0..6, 0..self.instance_count as u32);
    }
}



fn add_instances(
    image_position: [f32; 2],
    image_size: [f32; 2],
    rotation: f32,
    opacity: f32,
    snap: bool,
    entry: &AtlasEntry,
    instances: &mut Vec<Instance>,
) {
    let center = [
        image_position[0] + image_size[0] / 2.0,
        image_position[1] + image_size[1] / 2.0,
    ];

    match entry {
        AtlasEntry::Contiguous(allocation) => {
            add_instance(
                image_position,
                center,
                image_size,
                rotation,
                opacity,
                snap,
                allocation,
                instances,
            );
        }
        AtlasEntry::Fragmented { fragments, size } => {
            let scaling_x = image_size[0] / size.0 as f32;
            let scaling_y = image_size[1] / size.1 as f32;

            for fragment in fragments {
                let allocation = &fragment.allocation;

                let [x, y] = image_position;
                let (fragment_x, fragment_y) = fragment.position;
                let (fragment_width, fragment_height) = allocation.size();

                let position = [
                    x + fragment_x as f32 * scaling_x,
                    y + fragment_y as f32 * scaling_y,
                ];

                let size = [
                    fragment_width as f32 * scaling_x,
                    fragment_height as f32 * scaling_y,
                ];

                add_instance(
                    position, center, size, rotation, opacity, snap,
                    allocation, instances,
                );
            }
        }
    }
}

#[inline]
fn add_instance(
    position: [f32; 2],
    center: [f32; 2],
    size: [f32; 2],
    rotation: f32,
    opacity: f32,
    snap: bool,
    allocation: &Allocation,
    instances: &mut Vec<Instance>,
) {
    let (x, y) = allocation.position();
    let (width, height) = allocation.size();
    let layer = allocation.layer();

    let instance = Instance {
        _position: position,
        _center: center,
        _size: size,
        _rotation: rotation,
        _opacity: opacity,
        _position_in_atlas: [
            (x as f32 + 0.5) / ATLAS_SIZE as f32,
            (y as f32 + 0.5) / ATLAS_SIZE as f32,
        ],
        _size_in_atlas: [
            (width as f32 - 1.0) / ATLAS_SIZE as f32,
            (height as f32 - 1.0) / ATLAS_SIZE as f32,
        ],
        _layer: layer as u32,
        _snap: snap as u32,
    };

    instances.push(instance);
}



// ---



#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Instance {
    _position: [f32; 2],
    _center: [f32; 2],
    _size: [f32; 2],
    _rotation: f32,
    _opacity: f32,
    _position_in_atlas: [f32; 2],
    _size_in_atlas: [f32; 2],
    _layer: u32,
    _snap: u32,
}

impl Instance {
    pub const INITIAL: usize = 20;
}

#[derive(Clone, Copy, Debug)]
#[derive(bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    transform: [f32; 16],
    scale_factor: f32,
    // Uniforms must be aligned to their largest member,
    // this uses a mat4x4<f32> which aligns to 16, so align to that
    _padding: [f32; 3],
}




#[derive(Debug, Clone)]
pub struct ImagePipeline {
    raw: wgpu::RenderPipeline,
    backend: wgpu::Backend,
    nearest_sampler: wgpu::Sampler,
    linear_sampler: wgpu::Sampler,
    texture_layout: Arc<wgpu::BindGroupLayout>,
    constant_layout: wgpu::BindGroupLayout,
}

impl ImagePipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        backend: wgpu::Backend,
    ) -> Self {
        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            min_filter: wgpu::FilterMode::Nearest,
            mag_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let constant_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bog::uniforms_layout::image"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            core::mem::size_of::<Uniforms>() as u64,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering,
                    ),
                    count: None,
                },
            ],
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bog::texture_atlas_layout::image"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
                count: None,
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bog::pipeline_layout::image"),
            push_constant_ranges: &[],
            bind_group_layouts: &[&constant_layout, &texture_layout],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bog::shader::image"),
            source: gpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!("shaders/image.wgsl"),
            )),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bog::pipeline::image"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: core::mem::size_of::<Instance>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array!(
                        // Position
                        0 => Float32x2,
                        // Center
                        1 => Float32x2,
                        // Scale
                        2 => Float32x2,
                        // Rotation
                        3 => Float32,
                        // Opacity
                        4 => Float32,
                        // Atlas position
                        5 => Float32x2,
                        // Atlas scale
                        6 => Float32x2,
                        // Layer
                        7 => Sint32,
                        // Snap
                        8 => Uint32,
                    ),
                }],
                compilation_options:
                    wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options:
                    wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        ImagePipeline {
            raw: pipeline,
            backend,
            nearest_sampler,
            linear_sampler,
            texture_layout: Arc::new(texture_layout),
            constant_layout,
        }
    }

    pub fn create_cache(&self, device: &wgpu::Device) -> ImageCache {
        ImageCache::new(device, self.backend, self.texture_layout.clone())
    }
}



pub struct ImageCache {
    atlas: ImageAtlas,
    raster: RasterCache,
}

impl ImageCache {
    pub fn new(
        device: &wgpu::Device,
        backend: wgpu::Backend,
        layout: Arc<wgpu::BindGroupLayout>,
    ) -> Self {
        Self {
            atlas: ImageAtlas::new(device, backend, layout),
            raster: RasterCache::default(),
        }
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        self.atlas.bind_group()
    }

    pub fn measure_image(&mut self, handle: &ImageHandle) -> (u32, u32) {
        self.raster.load(handle).dimensions()
    }

    fn upload_raster(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        handle: &ImageHandle,
    ) -> Option<&AtlasEntry> {
        self.raster.upload(device, encoder, handle, &mut self.atlas)
    }

    pub fn trim(&mut self) {
        self.raster.trim(&mut self.atlas);
    }
}



// ---



const ATLAS_SIZE: u32 = 2048;

struct ImageAtlas {
    backend: wgpu::Backend,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    texture_bind_group: wgpu::BindGroup,
    texture_layout: Arc<wgpu::BindGroupLayout>,
    layers: Vec<AtlasLayer>,
}

impl ImageAtlas {
    fn new(
        device: &wgpu::Device,
        backend: wgpu::Backend,
        texture_layout: Arc<wgpu::BindGroupLayout>,
    ) -> Self {
        let layers = match backend {
            // NOTE: On the GL backend we start with 2 layers, to help wgpu figure
            //       out that this texture is `GL_TEXTURE_2D_ARRAY` rather than `GL_TEXTURE_2D`
            // https://github.com/gfx-rs/wgpu/blob/004e3efe84a320d9331371ed31fa50baa2414911/wgpu-hal/src/gles/mod.rs#L371
            wgpu::Backend::Gl => vec![AtlasLayer::Empty, AtlasLayer::Empty],
            _ => vec![AtlasLayer::Empty],
        };

        let extent = wgpu::Extent3d {
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            depth_or_array_layers: layers.len() as u32,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("iced_wgpu::image texture atlas"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let texture_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("iced_wgpu::image texture atlas bind group"),
                layout: &texture_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                }],
            });

        ImageAtlas {
            backend,
            texture,
            texture_view,
            texture_bind_group,
            texture_layout,
            layers,
        }
    }

    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.texture_bind_group
    }

    fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Option<AtlasEntry> {
        let entry = {
            let current_size = self.layers.len();
            let entry = self.allocate(width, height)?;

            // We grow the internal texture after allocating if necessary
            let new_layers = self.layers.len() - current_size;
            self.grow(new_layers, device, encoder);

            entry
        };

        // println!("Allocated atlas entry: {entry:?}");

        // NOTE: It is a webgpu requirement that:
        //          BufferCopyView.layout.bytes_per_row % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT == 0
        //       So we calculate padded_width by rounding width up to the next multiple of
        //       wgpu::COPY_BYTES_PER_ROW_ALIGNMENT.
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padding = (align - (4 * width) % align) % align;
        let padded_width = (4 * width + padding) as usize;
        let padded_data_size = padded_width * height as usize;

        let mut padded_data = vec![0; padded_data_size];

        for row in 0..height as usize {
            let offset = row * padded_width;

            padded_data[offset..offset + 4 * width as usize].copy_from_slice(
                &data[row * 4 * width as usize..(row + 1) * 4 * width as usize],
            );
        }

        match &entry {
            AtlasEntry::Contiguous(allocation) => {
                self.upload_allocation(
                    &padded_data,
                    width,
                    height,
                    padding,
                    0,
                    allocation,
                    device,
                    encoder,
                );
            }
            AtlasEntry::Fragmented { fragments, .. } => {
                for fragment in fragments {
                    let (x, y) = fragment.position;
                    let offset = (y * padded_width as u32 + 4 * x) as usize;

                    self.upload_allocation(
                        &padded_data,
                        width,
                        height,
                        padding,
                        offset,
                        &fragment.allocation,
                        device,
                        encoder,
                    );
                }
            }
        }

        Some(entry)
    }

    fn remove(&mut self, entry: &AtlasEntry) {
        // println!("Removing atlas entry: {entry:?}");

        match entry {
            AtlasEntry::Contiguous(allocation) => {
                self.deallocate(allocation);
            }
            AtlasEntry::Fragmented { fragments, .. } => {
                for fragment in fragments {
                    self.deallocate(&fragment.allocation);
                }
            }
        }
    }

    fn allocate(&mut self, width: u32, height: u32) -> Option<AtlasEntry> {
        // Allocate one layer if texture fits perfectly
        if width == ATLAS_SIZE && height == ATLAS_SIZE {
            let mut empty_layers = self
                .layers
                .iter_mut()
                .enumerate()
                .filter(|(_, layer)| layer.is_empty());

            if let Some((i, layer)) = empty_layers.next() {
                *layer = AtlasLayer::Full;

                return Some(AtlasEntry::Contiguous(Allocation::Full { layer: i }));
            }

            self.layers.push(AtlasLayer::Full);

            return Some(AtlasEntry::Contiguous(Allocation::Full {
                layer: self.layers.len() - 1,
            }));
        }

        // Split big textures across multiple layers
        if width > ATLAS_SIZE || height > ATLAS_SIZE {
            let mut fragments = Vec::new();
            let mut y = 0;

            while y < height {
                let height = std::cmp::min(height - y, ATLAS_SIZE);
                let mut x = 0;

                while x < width {
                    let width = std::cmp::min(width - x, ATLAS_SIZE);

                    let allocation = self.allocate(width, height)?;

                    if let AtlasEntry::Contiguous(allocation) = allocation {
                        fragments.push(Fragment {
                            position: (x, y),
                            allocation,
                        });
                    }

                    x += width;
                }

                y += height;
            }

            return Some(AtlasEntry::Fragmented {
                size: (width, height),
                fragments,
            });
        }

        // Try allocating on an existing layer
        for (i, layer) in self.layers.iter_mut().enumerate() {
            match layer {
                AtlasLayer::Empty => {
                    let mut allocator = Allocator::new(ATLAS_SIZE);

                    if let Some(region) = allocator.allocate(width, height) {
                        *layer = AtlasLayer::Busy(allocator);

                        return Some(AtlasEntry::Contiguous(Allocation::Partial {
                            region,
                            layer: i,
                        }));
                    }
                }
                AtlasLayer::Busy(allocator) => {
                    if let Some(region) = allocator.allocate(width, height) {
                        return Some(AtlasEntry::Contiguous(Allocation::Partial {
                            region,
                            layer: i,
                        }));
                    }
                }
                AtlasLayer::Full => {}
            }
        }

        // Create new layer with atlas allocator
        let mut allocator = Allocator::new(ATLAS_SIZE);

        if let Some(region) = allocator.allocate(width, height) {
            self.layers.push(AtlasLayer::Busy(allocator));

            return Some(AtlasEntry::Contiguous(Allocation::Partial {
                region,
                layer: self.layers.len() - 1,
            }));
        }

        // We ran out of memory (?)
        None
    }

    fn deallocate(&mut self, allocation: &Allocation) {
        // println!("Deallocating atlas: {allocation:?}");

        match allocation {
            Allocation::Full { layer } => {
                self.layers[*layer] = AtlasLayer::Empty;
            }
            Allocation::Partial { layer, region } => {
                let layer = &mut self.layers[*layer];

                if let AtlasLayer::Busy(allocator) = layer {
                    allocator.deallocate(region);

                    if allocator.is_empty() {
                        *layer = AtlasLayer::Empty;
                    }
                }
            }
        }
    }

    fn upload_allocation(
        &mut self,
        data: &[u8],
        image_width: u32,
        image_height: u32,
        padding: u32,
        offset: usize,
        allocation: &Allocation,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        use wgpu::util::DeviceExt;

        let (x, y) = allocation.position();
        let (width, height) = allocation.size();
        let layer = allocation.layer();

        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bog::upload_buffer::image"),
            contents: data,
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        encoder.copy_buffer_to_texture(
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: offset as u64,
                    bytes_per_row: Some(4 * image_width + padding),
                    rows_per_image: Some(image_height),
                },
            },
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x,
                    y,
                    z: layer as u32,
                },
                aspect: wgpu::TextureAspect::default(),
            },
            extent,
        );
    }

    fn grow(
        &mut self,
        amount: usize,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if amount == 0 {
            return;
        }

        // On the GL backend if layers.len() == 6 we need to help wgpu figure out that this texture
        // is still a `GL_TEXTURE_2D_ARRAY` rather than `GL_TEXTURE_CUBE_MAP`.
        //
        // This will over-allocate some unused memory on GL, but it's better than not being able to
        // grow the atlas past a depth of 6.
        //
        // See: https://github.com/gfx-rs/wgpu/blob/004e3efe84a320d9331371ed31fa50baa2414911/wgpu-hal/src/gles/mod.rs#L371
        let depth_or_array_layers = match self.backend {
            wgpu::Backend::Gl if self.layers.len() == 6 => 7,
            _ => self.layers.len() as u32,
        };

        let new_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bog::texture_atlas::image"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let amount_to_copy = self.layers.len() - amount;

        for (i, layer) in self.layers.iter_mut().take(amount_to_copy).enumerate() {
            if layer.is_empty() {
                continue;
            }

            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: wgpu::TextureAspect::default(),
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &new_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: wgpu::TextureAspect::default(),
                },
                wgpu::Extent3d {
                    width: ATLAS_SIZE,
                    height: ATLAS_SIZE,
                    depth_or_array_layers: 1,
                },
            );
        }

        self.texture = new_texture;
        self.texture_view = self.texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        self.texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bog_texture_atlas_bind_group::image"),
            layout: &self.texture_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &self.texture_view,
                ),
            }],
        });
    }
}



enum AtlasLayer {
    Empty,
    Busy(Allocator),
    Full,
}

impl AtlasLayer {
    fn is_empty(&self) -> bool {
        matches!(self, AtlasLayer::Empty)
    }

    // fn allocations(&self) -> usize {
    //     match self {
    //         AtlasLayer::Empty => 0,
    //         AtlasLayer::Busy(allocator) => allocator.allocations(),
    //         AtlasLayer::Full => 1,
    //     }
    // }
}

#[derive(Debug)]
enum AtlasEntry {
    Contiguous(Allocation),
    Fragmented {
        size: (u32, u32),
        fragments: Vec<Fragment>,
    },
}

impl AtlasEntry {
    fn size(&self) -> (u32, u32) {
        match self {
            AtlasEntry::Contiguous(allocation) => allocation.size(),
            AtlasEntry::Fragmented { size, .. } => *size,
        }
    }
}

#[derive(Debug)]
struct Fragment {
    position: (u32, u32),
    allocation: Allocation,
}

#[derive(Debug)]
enum Allocation {
    Partial {
        layer: usize,
        region: Region,
    },
    Full {
        layer: usize,
    },
}

impl Allocation {
    fn position(&self) -> (u32, u32) {
        match self {
            Allocation::Partial { region, .. } => region.position(),
            Allocation::Full { .. } => (0, 0),
        }
    }

    fn size(&self) -> (u32, u32) {
        match self {
            Allocation::Partial { region, .. } => region.size(),
            Allocation::Full { .. } => (ATLAS_SIZE, ATLAS_SIZE),
        }
    }

    fn layer(&self) -> usize {
        match self {
            Allocation::Partial { layer, .. } => *layer,
            Allocation::Full { layer } => *layer,
        }
    }
}

struct Allocator {
    raw: guillotiere::AtlasAllocator,
    allocations: usize,
}

impl Allocator {
    fn new(size: u32) -> Allocator {
        let raw = guillotiere::AtlasAllocator::new(
            guillotiere::Size::new(size as i32, size as i32),
        );

        Allocator {
            raw,
            allocations: 0,
        }
    }

    fn allocate(&mut self, width: u32, height: u32) -> Option<Region> {
        let allocation =
            self.raw.allocate(guillotiere::Size::new(width as i32, height as i32))?;

        self.allocations += 1;

        Some(Region { allocation })
    }

    fn deallocate(&mut self, region: &Region) {
        self.raw.deallocate(region.allocation.id);

        self.allocations = self.allocations.saturating_sub(1);
    }

    fn is_empty(&self) -> bool {
        self.allocations == 0
    }

    // fn allocations(&self) -> usize {
    //     self.allocations
    // }
}

#[derive(Debug)]
struct Region {
    allocation: guillotiere::Allocation,
}

impl Region {
    fn position(&self) -> (u32, u32) {
        let rectangle = &self.allocation.rectangle;

        (rectangle.min.x as u32, rectangle.min.y as u32)
    }

    fn size(&self) -> (u32, u32) {
        let size = self.allocation.rectangle.size();

        (size.width as u32, size.height as u32)
    }
}



// ---



#[derive(Default)]
struct RasterCache {
    map: rustc_hash::FxHashMap<u64, RasterImageMemory>,
    hits: rustc_hash::FxHashSet<u64>,
    should_trim: bool,
}

impl RasterCache {
    fn load(&mut self, handle: &ImageHandle) -> &mut RasterImageMemory {
        if self.contains(handle) {
            return self.get(handle).unwrap();
        }

        let memory = match load_image(handle) {
            Ok(image) => RasterImageMemory::Host(image),
            Err(::image::error::ImageError::IoError(_)) => RasterImageMemory::NotFound,
            Err(_) => RasterImageMemory::Invalid,
        };

        self.should_trim = true;

        self.insert(handle, memory);
        self.get(handle).unwrap()
    }

    fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        handle: &ImageHandle,
        atlas: &mut ImageAtlas,
    ) -> Option<&AtlasEntry> {
        let memory = self.load(handle);

        if let RasterImageMemory::Host(image) = memory {
            let (width, height) = image.dimensions();

            let entry = atlas.upload(device, encoder, width, height, image)?;

            *memory = RasterImageMemory::Device(entry);
        }

        if let RasterImageMemory::Device(allocation) = memory {
            Some(allocation)
        } else {
            None
        }
    }

    fn trim(&mut self, atlas: &mut ImageAtlas) {
        // Only trim if new entries have landed in the `Cache`
        if !self.should_trim {
            return;
        }

        let hits = &self.hits;

        self.map.retain(|k, memory| {
            let retain = hits.contains(k);

            if !retain {
                if let RasterImageMemory::Device(entry) = memory {
                    atlas.remove(entry);
                }
            }

            retain
        });

        self.hits.clear();
        self.should_trim = false;
    }

    fn get(&mut self, handle: &ImageHandle) -> Option<&mut RasterImageMemory> {
        let _ = self.hits.insert(handle.id());

        self.map.get_mut(&handle.id())
    }

    fn insert(&mut self, handle: &ImageHandle, memory: RasterImageMemory) {
        let _ = self.map.insert(handle.id(), memory);
    }

    fn contains(&self, handle: &ImageHandle) -> bool {
        self.map.contains_key(&handle.id())
    }
}



#[derive(Debug)]
enum RasterImageMemory {
    Host(::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>),
    Device(AtlasEntry),
    NotFound,
    Invalid,
}

impl RasterImageMemory {
    fn dimensions(&self) -> (u32, u32) {
        match self {
            RasterImageMemory::Host(image) => image.dimensions(),
            RasterImageMemory::Device(entry) => entry.size(),
            RasterImageMemory::NotFound => (1, 1),
            RasterImageMemory::Invalid => (1, 1),
        }
    }
}



// ---



// TODO: Handle EXIF orientation.
fn load_image(
    handle: &ImageHandle,
) -> ::image::ImageResult<::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>>
{
    let (width, height, pixels) = match handle {
        ImageHandle::Path(_, path) => {
            let image = ::image::open(path)?;
            let rgba = image.into_rgba8();

            (
                rgba.width(),
                rgba.height(),
                rgba.into_raw(),
            )
        }
        ImageHandle::Bytes(_, bytes) => {
            let image = ::image::load_from_memory(bytes)?;
            let rgba = image.into_rgba8();

            (
                rgba.width(),
                rgba.height(),
                rgba.into_raw(),
            )
        }
    };

    if let Some(image) = ::image::ImageBuffer::from_raw(width, height, pixels) {
        Ok(image)
    } else {
        Err(::image::error::ImageError::Limits(
            ::image::error::LimitError::from_kind(
                ::image::error::LimitErrorKind::DimensionError,
            ),
        ))
    }
}
