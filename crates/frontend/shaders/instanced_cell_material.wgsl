#import bevy_sprite::{
    mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_world_to_clip},
    mesh2d_view_vindings::view,
}
#import bevy_render::instance_index::get_instance_index


struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    
    @location(3) model_pos: vec3<f32>,
    @location(4) size: vec2<f32>,
    @location(5) corner_offset_x: vec4<f32>,
    @location(6) corner_offset_y: vec4<f32>,
    @location(7) rounding: vec4<f32>,
    @location(8) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) rounding: vec4<f32>,
    @location(3) size: vec2<f32>,
};

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(vertex: Vertex, @builtin(vertex_index) index: u32) -> VertexOutput {
    let vertex_offset = vec2<f32>(
        vertex.corner_offset_x[index % 4u],
        vertex.corner_offset_y[index % 4u],
    );
    let vertex_pos = vertex.size * vertex.uv * vec2(1.0, -1.0) + vertex_offset;
    let final_vertex_pos = vec4<f32>(vertex.model_pos.xy + vertex_pos, vertex.model_pos.z, 1.0);

    var out: VertexOutput;
    out.clip_position = mesh2d_position_world_to_clip(final_vertex_pos);
    out.color = vertex.i_color;
    out.uv = vertex.uv;
    out.rounding = vertex.rounding;
    out.size = vertex.size;
    // Workaround for: https://github.com/bevyengine/bevy/issues/10509
    out.clip_position.x += f32(get_instance_index(0u)) * 0.00001;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let xy = (in.uv * vec2(in.size.x, in.size.y));
    let mask = get_rounding_mask(xy, in.size, in.rounding);

    return mask * in.color * textureSample(texture, texture_sampler, in.uv);
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

fn map_clamp(value: f32, high: f32, low: f32, to_high: f32, to_low: f32) -> f32{
    let t = (value - low )/ (high - low);
    let x = clamp(to_low + (to_high - to_low) * t, to_low, to_high); 
    return x;
}