


struct Globals {
    transform: mat4x4<f32>,
    scale: f32,
}

@group(0) @binding(0) var<uniform> globals: Globals;



fn distance_alg(
    frag_coord: vec2<f32>,
    position: vec2<f32>,
    size: vec2<f32>,
    radius: f32
) -> f32 {
    var inner_half_size: vec2<f32> = (size - vec2<f32>(radius, radius) * 2.0) / 2.0;
    var top_left: vec2<f32> = position + vec2<f32>(radius, radius);
    return rounded_box_sdf(frag_coord - top_left - inner_half_size, inner_half_size, 0.0);
}

// Given a vector from a point to the center of a rounded rectangle of the given `size` and
// border `radius`, determines the point's distance from the nearest edge of the rounded rectangle
fn rounded_box_sdf(to_center: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    return length(max(abs(to_center) - size + vec2<f32>(radius, radius), vec2<f32>(0.0, 0.0))) - radius;
}

// Based on the fragment position and the center of the quad, select one of the 4 radii.
// Order matches CSS: pqdb
fn select_border_radius(radii: vec4<f32>, position: vec2<f32>, center: vec2<f32>) -> f32 {
    var rx = radii.x;
    var ry = radii.y;
    rx = select(radii.x, radii.y, position.x > center.x);
    ry = select(radii.w, radii.z, position.x > center.x);
    rx = select(rx, ry, position.y > center.y);
    return rx;
}



// Compute the normalized quad coordinates based on the vertex index.
fn vertex_position(vertex_index: u32) -> vec2<f32> {
    // #: 0 1 2 3 4 5
    // x: 1 1 0 0 0 1
    // y: 1 0 0 0 1 1
    return vec2<f32>((vec2(1u, 2u) + vertex_index) % vec2(6u) < vec2(3u));
}

fn convert_color(color: u32) -> vec4<f32> {
    let r = f32((color >> 24u) & 255u) / 255.0;
    let g = f32((color >> 16u) & 255u) / 255.0;
    let b = f32((color >> 8u) & 255u) / 255.0;
    let a = f32(color & 255u) / 255.0;

    return vec4<f32>(
        color_channel_to_linear(r),
        color_channel_to_linear(g),
        color_channel_to_linear(b),
        a,
    );
}

fn color_channel_to_linear(channel: f32) -> f32 {
    if channel < 0.04045 {
        return channel / 12.92;
    } else {
        return pow((channel + 0.055) / 1.055, 2.4);
    }
}



struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) color: u32,
    @location(1) pos: vec2<f32>,
    @location(2) scale: vec2<f32>,
    @location(3) border_color: u32,
    @location(4) border_radius: vec4<f32>,
    @location(5) border_width: f32,
    @location(6) shadow_color: u32,
    @location(7) shadow_offset: vec2<f32>,
    @location(8) shadow_blur_radius: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) border_color: vec4<f32>,
    @location(2) pos: vec2<f32>,
    @location(3) scale: vec2<f32>,
    @location(4) border_radius: vec4<f32>,
    @location(5) border_width: f32,
    @location(6) shadow_color: vec4<f32>,
    @location(7) shadow_offset: vec2<f32>,
    @location(8) shadow_blur_radius: f32,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var pos: vec2<f32> = (input.pos + min(input.shadow_offset, vec2<f32>(0.0, 0.0)) - input.shadow_blur_radius) * globals.scale;
    var scale: vec2<f32> = (input.scale + vec2<f32>(abs(input.shadow_offset.x), abs(input.shadow_offset.y)) + input.shadow_blur_radius * 2.0) * globals.scale;
    var snap: vec2<f32> = vec2<f32>(0.0, 0.0);

    if input.scale.x == 1.0 {
        snap.x = round(pos.x) - pos.x;
    }

    if input.scale.y == 1.0 {
        snap.y = round(pos.y) - pos.y;
    }

    var min_border_radius = min(input.scale.x, input.scale.y) * 0.5;
    var border_radius: vec4<f32> = vec4<f32>(
        min(input.border_radius.x, min_border_radius),
        min(input.border_radius.y, min_border_radius),
        min(input.border_radius.z, min_border_radius),
        min(input.border_radius.w, min_border_radius)
    );

    var transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale.y + 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(pos - vec2<f32>(0.5, 0.5) + snap, 0.0, 1.0)
    );

    out.position = globals.transform * transform * vec4<f32>(vertex_position(input.vertex_index), 0.0, 1.0);
    out.color = convert_color(input.color);
    out.border_color = convert_color(input.border_color);
    out.pos = input.pos * globals.scale + snap;
    out.scale = input.scale * globals.scale;
    out.border_radius = border_radius * globals.scale;
    out.border_width = input.border_width * globals.scale;
    out.shadow_color = convert_color(input.shadow_color);
    out.shadow_offset = input.shadow_offset * globals.scale;
    out.shadow_blur_radius = input.shadow_blur_radius * globals.scale;

    return out;
}

@fragment
fn fs_main(
    input: VertexOutput
) -> @location(0) vec4<f32> {
    var mixed_color: vec4<f32> = input.color;

    var border_radius = select_border_radius(
        input.border_radius,
        input.position.xy,
        (input.pos + input.scale * 0.5).xy
    );

    if (input.border_width > 0.0) {
        var internal_border: f32 = max(border_radius - input.border_width, 0.0);

        var internal_distance: f32 = distance_alg(
            input.position.xy,
            input.pos + vec2<f32>(input.border_width, input.border_width),
            input.scale - vec2<f32>(input.border_width * 2.0, input.border_width * 2.0),
            internal_border
        );

        var border_mix: f32 = smoothstep(
            max(internal_border - 0.5, 0.0),
            internal_border + 0.5,
            internal_distance
        );

        mixed_color = mix(input.color, input.border_color, vec4<f32>(border_mix, border_mix, border_mix, border_mix));
    }

    var dist: f32 = distance_alg(
        vec2<f32>(input.position.x, input.position.y),
        input.pos,
        input.scale,
        border_radius
    );

    var radius_alpha: f32 = 1.0 - smoothstep(
        max(border_radius - 0.5, 0.0),
        border_radius + 0.5,
        dist
    );

    let quad_color = vec4<f32>(mixed_color.x, mixed_color.y, mixed_color.z, mixed_color.w * radius_alpha);

    if input.shadow_color.a > 0.0 {
        let shadow_radius = select_border_radius(
            input.border_radius,
            input.position.xy - input.shadow_offset,
            (input.pos + input.scale * 0.5).xy
        );
        let shadow_distance = max(rounded_box_sdf(input.position.xy - input.pos - input.shadow_offset - (input.scale / 2.0), input.scale / 2.0, shadow_radius), 0.);

        let shadow_alpha = 1.0 - smoothstep(-input.shadow_blur_radius, input.shadow_blur_radius, shadow_distance);
        let shadow_color = input.shadow_color;
        let base_color = mix(
            vec4<f32>(shadow_color.x, shadow_color.y, shadow_color.z, 0.0),
            quad_color,
            quad_color.a
        );

        return mix(base_color, shadow_color, (1.0 - radius_alpha) * shadow_alpha);
    } else {
        return quad_color;
    }
}
