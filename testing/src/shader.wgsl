


// --- Utilities

fn convert_color(color: u32) -> vec4<f32> {
    return vec4<f32>(
        translate_color_channel(f32((color >> 8u) & 255u)),
        translate_color_channel(f32((color >> 16u) & 255u)),
        translate_color_channel(f32((color >> 24u) & 255u)),
        f32(color & 255u) / 255.0,
    );
}

fn translate_color_channel(color: f32) -> f32 {
    return pow((color / 255.0 + 0.055) / 1.055, 2.4);
}



// --- Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = convert_color(model.color);
    out.clip_position = vec4<f32>(model.position, 1.0, 1.0);
    return out;
}



// --- Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
