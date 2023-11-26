#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_vertex_output::VertexOutput,
}

struct Shape {
    size: vec2<f32>,
    skew: f32,
    rounding: vec4<f32>,
};
@group(1) @binding(1) var<uniform> shape: Shape;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;

    let position = shape.size * vertex.uv * vec2(1.0, -1.0) + vec2(shape.skew, 0.0) * (1.0-vertex.uv.y);
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        mesh_functions::get_model_matrix(vertex.instance_index),
        vec4<f32>(position, 0.0, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);

    return out;
}
