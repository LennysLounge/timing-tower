use bevy::{
    prelude::{Asset, Color, Handle, Image, Plugin, Vec2, Vec4},
    reflect::{TypePath, TypeUuid},
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderType},
    },
    sprite::{Material2d, Material2dPlugin},
};

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<GradientMaterial>::default());
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub enum Gradient {
    #[default]
    None,
    Linear(LinearGradient),
    Radial(RadialGradient),
    Conical(ConicalGradient),
}

#[derive(Debug, Clone, Default)]
pub struct LinearGradient {
    pub color: Color,
    pub position: Vec2,
    pub spread: f32,
    pub angle: f32,
}

#[derive(Debug, Clone, Default)]
pub struct RadialGradient {
    pub color: Color,
    pub position: Vec2,
    pub spread: f32,
    pub distance: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ConicalGradient {
    pub color: Color,
    pub position: Vec2,
    pub angle: f32,
}

#[derive(AsBindGroup, Asset, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "a459baf1-6fbd-4c97-bbee-4c8a3fae6a3b"]
#[uniform(0, MaterialUniform)]
pub struct GradientMaterial {
    pub color: Color,
    pub gradient: Gradient,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for GradientMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[derive(Clone, Default, ShaderType)]
struct MaterialUniform {
    kind: i32,
    color: Vec4,
    color_2: Vec4,
    pos: Vec2,
    spread: f32,
    param_1: f32,
}

impl AsBindGroupShaderType<MaterialUniform> for GradientMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> MaterialUniform {
        let (kind, color_2, position, spread, param_1) = match &self.gradient {
            Gradient::None => (0, Color::default(), Vec2::default(), 0.0, 0.0),
            Gradient::Linear(g) => (1, g.color, g.position, g.spread, g.angle),
            Gradient::Radial(g) => (2, g.color, g.position, g.spread, g.distance),
            Gradient::Conical(g) => (3, g.color, g.position, 0.0, g.angle),
        };
        MaterialUniform {
            kind,
            color: self.color.as_linear_rgba_f32().into(),
            color_2: color_2.as_linear_rgba_f32().into(),
            pos: position.clone(),
            spread,
            param_1,
        }
    }
}
