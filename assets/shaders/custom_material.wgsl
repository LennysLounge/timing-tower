#import bevy_pbr::mesh_vertex_output MeshVertexOutput

struct CustomMaterial {
    kind: i32,
    color: vec4<f32>,
    color_2: vec4<f32>,
    pos: vec2<f32>,
    spread: f32,
    param_1: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {

    var t = 1.0;
    
    // Linear gradient
    if material.kind == 1 {
        let n = vec2f(
            sin(material.param_1),
            cos(material.param_1)
        );
        let to_pixel = mesh.world_position.xy - material.pos;
        t = clamp(
                (dot(n, to_pixel) / material.spread + 0.5),
                0.0,
                1.0
            );
    }
    // Radial gradient
    else if material.kind == 2 {
        t = clamp(
                (distance(material.pos, mesh.world_position.xy) - material.param_1) / material.spread,
                0.0,
                1.0
            );
    }
    // Conical gradient
    else if material.kind == 3 {
        let to_pixel = mesh.world_position.xy - material.pos;
        var angle = (atan2(to_pixel.y, to_pixel.x) + material.param_1) / 6.2831853 + 0.5;
        t = clamp(angle % 1.0, 0.0, 1.0);
    }

    // Turn linear value into a sin value.
    t = (1.0-cos(t*3.1415926)) / 2.0;
    
    return (material.color * t + material.color_2 * (1.0-t))
        * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}
