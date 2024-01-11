#import bevy_sprite::{
    mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_world_to_clip},
    mesh2d_view_vindings::view,
}
#import bevy_render::instance_index::get_instance_index

struct InstanceData{
    @location(0) model_pos: vec3<f32>,
    @location(1) size: vec2<f32>,
    @location(2) corner_offset_x: vec4<f32>,
    @location(3) corner_offset_y: vec4<f32>,
    @location(4) rounding: vec4<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) rounding: vec4<f32>,
    @location(3) size: vec2<f32>,
    @location(4) edge_dist: vec4<f32>,
};

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32, instance_data: InstanceData) -> VertexOutput {
    let c0 = corner(instance_data, 0u).xy;
    let c1 = corner(instance_data, 1u).xy;
    let c2 = corner(instance_data, 2u).xy;
    let c3 = corner(instance_data, 3u).xy;
    let position = corner(instance_data, vertex_index);

    var out: VertexOutput;
    out.clip_position = mesh2d_position_world_to_clip(vec4(position, 1.0));
    out.color = instance_data.color;
    out.uv = vec2(
        width_coef[vertex_index % 4u],
        height_coef[vertex_index % 4u],
    );
    out.rounding = instance_data.rounding;
    out.size = instance_data.size;
    out.edge_dist = vec4(
        max(0.0, dist_to_edge(c0, c1, position.xy)),
        max(0.0, dist_to_edge(c1, c3, position.xy)),
        max(0.0, dist_to_edge(c3, c2, position.xy)),
        max(0.0, dist_to_edge(c2, c0, position.xy)),
    );

    // Workaround for: https://github.com/bevyengine/bevy/issues/10509
    out.clip_position.x += f32(get_instance_index(0u)) * 0.00001;
    return out;
}

// Get the coordinates of the corner by its index.
const width_coef = vec4<f32>(0.0, 1.0, 0.0, 1.0);
const height_coef = vec4<f32>(0.0, 0.0, 1.0, 1.0);
fn corner(data: InstanceData, index: u32) -> vec3<f32>{
    return vec3(
        data.corner_offset_x[index % 4u] + data.model_pos.x + data.size.x * width_coef[index % 4u],
        data.corner_offset_y[index % 4u] + data.model_pos.y - data.size.y * height_coef[index % 4u],
        data.model_pos.z
    );
}

// Get the distance of a point to an edge.
fn dist_to_edge(c1: vec2<f32>,c2: vec2<f32>, point: vec2<f32>) -> f32{
    let edge = c2-c1;
    return cross2d((point-c1), edge) / length(edge);
}


fn cross2d(u: vec2<f32>, v: vec2<f32>) -> f32{
    return u.x * v.y - u.y * v.x;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let mask = get_rounding_mask(vec2(in.edge_dist[3u], in.edge_dist[0u]), in.rounding[0])
             * get_rounding_mask(vec2(in.edge_dist[0u], in.edge_dist[1u]), in.rounding[1])
             * get_rounding_mask(vec2(in.edge_dist[3u], in.edge_dist[2u]), in.rounding[2])
             * get_rounding_mask(vec2(in.edge_dist[1u], in.edge_dist[2u]), in.rounding[3]);

    return vec4(1.0, 1.0, 1.0, mask) * in.color * textureSample(texture, texture_sampler, in.uv);
}

fn get_rounding_mask(edge_pos: vec2<f32>, rounding: f32) -> f32{
    let mask = vec2(
        max(edge_pos.x, rounding),
        max(edge_pos.y, rounding),
    );
    let dist = distance(edge_pos, mask);
    return map_clamp(dist, rounding, rounding + 1.0, 1.0, 0.0);
}

fn map_clamp(value: f32, high: f32, low: f32, to_high: f32, to_low: f32) -> f32{
    let t = (value - low )/ (high - low);
    let x = clamp(to_low + (to_high - to_low) * t, to_low, to_high); 
    return x;
}