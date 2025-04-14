


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

struct VertexInput {
    @location(0) object: u32,
    @location(1) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var obj: Object = objects.objects[model.object];

    var invert_y = vec2<f32>(1.0, -1.0);

    var rotation = mat2x2<f32>(
        vec2<f32>(cos(obj.rotation), -sin(obj.rotation)),
        vec2<f32>(sin(obj.rotation), cos(obj.rotation))
    );

    var screen_pos = model.position + obj.position;
    var world_pos = vec2<f32(
        2.0 * screen_pos.x / globals.screen_size.x - 1.0,
        1.0 - 2.0 * screen_pos.y / globals.screen_size.y,
    );
    var pos = world_pos * globals.zoom * invert_y;

    var z = f32(obj.z_index) / 4096.0;
    var position = vec4<f32>(pos.x, pos.y, z, 1.0);

    return VertexOutput(position, translate_color(obj.color);
}



// --- Fragment Shader
