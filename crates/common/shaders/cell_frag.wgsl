#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_vertex_output::VertexOutput,
}

struct Gradient {
    kind: i32,
    color: vec4<f32>,
    color_2: vec4<f32>,
    pos: vec2<f32>,
    spread: f32,
    param_1: f32,
};

struct Shape {
    size: vec2<f32>,
    rounding: vec4<f32>,
};

@group(1) @binding(0) var<uniform> gradient: Gradient;
@group(1) @binding(1) var<uniform> shape: Shape;
@group(1) @binding(2) var base_color_texture: texture_2d<f32>;
@group(1) @binding(3) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

    let xy = (mesh.uv * vec2(shape.size.x, shape.size.y));
    let rounding_mask = get_rounding_mask(xy, shape.size, shape.rounding);

    return get_color(mesh) * rounding_mask;   
}

fn get_color(mesh: VertexOutput) -> vec4<f32> {
    var t = 1.0;

    // Linear gradient
    if gradient.kind == 1 {
        let n = vec2f(
            sin(gradient.param_1),
            cos(gradient.param_1)
        );
        let to_pixel = mesh.position.xy - gradient.pos;
        t = clamp(
                (dot(n, to_pixel) / gradient.spread + 0.5),
                0.0,
                1.0
            );
    }
    // Radial gradient
    else if gradient.kind == 2 {
        t = clamp(
                (distance(gradient.pos, mesh.position.xy) - gradient.param_1) / gradient.spread,
                0.0,
                1.0
            );
    }
    // Conical gradient
    else if gradient.kind == 3 {
        let to_pixel = mesh.position.xy - gradient.pos;
        var angle = (atan2(to_pixel.y, to_pixel.x) + gradient.param_1) / 6.2831853 + 0.5;
        t = clamp(angle % 1.0, 0.0, 1.0);
    }

    // Turn linear value into a sin value.
    t = (1.0-cos(t*3.1415926)) / 2.0;
    
    return (gradient.color * t + gradient.color_2 * (1.0-t))
        * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}

fn map_clamp(value: f32, high: f32, low: f32, to_high: f32, to_low: f32) -> f32{
    let t = (value - low )/ (high - low);
    let x = clamp(to_low + (to_high - to_low) * t, to_low, to_high); 
    return x;
}

fn get_rounding_mask(param_pos: vec2<f32>, param_size: vec2<f32>, rounding: vec4<f32>) -> vec4<f32>{
    let smoothing_distance = 1.0;

    let pos = param_pos - vec2(smoothing_distance / 2.0, smoothing_distance / 2.0);
    let size = param_size - vec2(smoothing_distance, smoothing_distance);
    
    var rounding_mask = 1.0;
    {
        let r = rounding.x;
        let mask = vec2(
            max(pos.x, r),
            max(pos.y, r),
        );
        let dist = distance(pos, mask);
        rounding_mask *= map_clamp(dist, r, r+smoothing_distance, 1.0, 0.0);
    }
    {
        let r = rounding.y;
        let mask = vec2(
            min(pos.x, size.x - r),
            max(pos.y, r),
        );
        let dist = distance(pos, mask);
        rounding_mask *= map_clamp(dist, r, r+smoothing_distance, 1.0, 0.0);
    }
        {
        let r = rounding.z;
        let mask = vec2(
            min(pos.x, size.x - r),
            min(pos.y, size.y - r),
        );
        let dist = distance(pos, mask);
        rounding_mask *= map_clamp(dist, r, r+smoothing_distance, 1.0, 0.0);
    }
        {
        let r = rounding.w;
        let mask = vec2(
            max(pos.x, r),
            min(pos.y, size.y - r),
        );
        let dist = distance(pos, mask);
        rounding_mask *= map_clamp(dist, r, r+smoothing_distance, 1.0, 0.0);
    }
    return vec4(1.0, 1.0, 1.0, rounding_mask);
}