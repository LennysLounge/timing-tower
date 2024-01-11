#import bevy_sprite::{
    mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_world_to_clip},
    mesh2d_view_vindings::view,
}
#import bevy_render::instance_index::get_instance_index

const corner_index_table: u32 = 0xe24u;

struct InstanceData{
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
    @location(4) edge_dist: vec4<f32>,
};

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32, instance_data: InstanceData) -> VertexOutput {
    //convert the vertex index into a corner index
    let corner_index = corner_index_table >> (vertex_index*2u) & 0x03u;

    let position = corner(instance_data, corner_index);

    let edge_distance = vec4(
        max(0.0, dist_to_edge(instance_data, position.xy, 0u)),
        max(0.0, dist_to_edge(instance_data, position.xy, 1u)),
        max(0.0, dist_to_edge(instance_data, position.xy, 2u)),
        max(0.0, dist_to_edge(instance_data, position.xy, 3u)),
    );

    var out: VertexOutput;
    out.clip_position = mesh2d_position_world_to_clip(vec4(position, 1.0));
    out.color = instance_data.i_color;
    out.uv = vec2(
        size_coef[corner_index % 4u],
        size_coef[(corner_index+3u) % 4u],
    );
    out.rounding = instance_data.rounding;
    out.size = instance_data.size;
    out.edge_dist = edge_distance;
    // Workaround for: https://github.com/bevyengine/bevy/issues/10509
    out.clip_position.x += f32(get_instance_index(0u)) * 0.00001;
    return out;
}

// Get the coordinates of the corner by its index.
const size_coef = vec4<f32>(0.0, 1.0, 1.0, 0.0);
fn corner(data: InstanceData, index: u32) -> vec3<f32>{
    return vec3(
        data.corner_offset_x[index % 4u] + data.model_pos.x + data.size.x * size_coef[index % 4u],
        data.corner_offset_y[index % 4u] + data.model_pos.y - data.size.y * size_coef[(index+3u) % 4u],
        data.model_pos.z
    );
}

// Get the distance of a point to an edge.
fn dist_to_edge(data: InstanceData, point: vec2<f32>, edge_index: u32) -> f32{
    let p1 = corner(data, edge_index).xy;
    let p2 = corner(data, edge_index + 1u).xy;
    let edge = p2-p1;
    return cross2d((point-p1), edge) / length(edge);
}

fn cross2d(u: vec2<f32>, v: vec2<f32>) -> f32{
    return u.x * v.y - u.y * v.x;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let mask = get_rounding_mask(in.edge_dist, 0u, in.rounding[1])
             * get_rounding_mask(in.edge_dist, 1u, in.rounding[2])
             * get_rounding_mask(in.edge_dist, 2u, in.rounding[3])
             * get_rounding_mask(in.edge_dist, 3u, in.rounding[0]);

    return vec4(1.0, 1.0, 1.0, mask) * in.color * textureSample(texture, texture_sampler, in.uv);
}

fn get_rounding_mask(edge_distance: vec4<f32>, index: u32, rounding: f32) -> f32{
    let ab = vec2(
        edge_distance[index % 4u],
        edge_distance[(index+1u) % 4u],
    );
    let mask = vec2(
        max(ab.x, rounding),
        max(ab.y, rounding),
    );
    let dist = distance(ab, mask);
    return map_clamp(dist, rounding, rounding + 1.0, 1.0, 0.0);
}

fn map_clamp(value: f32, high: f32, low: f32, to_high: f32, to_low: f32) -> f32{
    let t = (value - low )/ (high - low);
    let x = clamp(to_low + (to_high - to_low) * t, to_low, to_high); 
    return x;
}