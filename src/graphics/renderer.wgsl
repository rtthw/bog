


// --- Uniforms

struct Globals {
    screen_size: vec2<f32>,
    zoom: f32,
    pad_1: u32,
};

struct Object {
    position: vec2<f32>,
    rotation: f32,
    z_index: i32,
    scale: f32,
    color: u32,
    pad_1: u32,
    pad_2: u32,
};

struct Objects {
    objects: array<Object, 256>,
};

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var<uniform> objects: Objects;



// --- Vertex Shader



// --- Fragment Shader
