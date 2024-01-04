#import bevy_sprite::{
    mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip, mesh2d_position_world_to_clip},
    mesh2d_view_vindings::view,
}


struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.

    out.clip_position = mesh2d_position_world_to_clip(vec4<f32>(position, 1.0));

    //out.clip_position = view.view_proj * vec4<f32>(position, 1.0);
    out.color = vertex.i_color;
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color * textureSample(texture, texture_sampler, in.uv);
}