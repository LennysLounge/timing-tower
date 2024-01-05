use bevy::{
    asset::load_internal_asset,
    prelude::{Asset, Color, Handle, Image, Plugin, Shader, Vec2, Vec4},
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderType},
    sprite::{Material2d, Material2dPlugin},
};
use uuid::uuid;

const VERT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("8f2e85d4-c560-410c-9159-c37a95e865e5").as_u128());
const FRAG_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(uuid!("eb34f151-aa39-4148-8e01-7c801b4b8566").as_u128());

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(
            app,
            FRAG_SHADER_HANDLE,
            "../shaders/cell_frag.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            VERT_SHADER_HANDLE,
            "../shaders/cell_vert.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<CellMaterial>::default());
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
#[uuid = "02ff810f-b8de-4d62-8b09-7da5072fae14"]
#[uniform(0, GradientUniform)]
#[uniform(1, ShapeUniform)]
pub struct CellMaterial {
    pub color: Color,
    pub gradient: Gradient,
    pub size: Vec2,
    pub skew: f32,
    pub rounding: Vec4,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for CellMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        FRAG_SHADER_HANDLE.into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        VERT_SHADER_HANDLE.into()
    }
}

#[derive(ShaderType)]
struct GradientUniform {
    kind: i32,
    color: Vec4,
    color_2: Vec4,
    pos: Vec2,
    spread: f32,
    param_1: f32,
}

impl From<&CellMaterial> for GradientUniform {
    fn from(value: &CellMaterial) -> Self {
        let (kind, color_2, position, spread, param_1) = match &value.gradient {
            Gradient::None => (0, Color::default(), Vec2::default(), 0.0, 0.0),
            Gradient::Linear(g) => (1, g.color, g.position, g.spread, g.angle),
            Gradient::Radial(g) => (2, g.color, g.position, g.spread, g.distance),
            Gradient::Conical(g) => (3, g.color, g.position, 0.0, g.angle),
        };
        GradientUniform {
            kind,
            color: value.color.as_linear_rgba_f32().into(),
            color_2: color_2.as_linear_rgba_f32().into(),
            pos: position.clone(),
            spread,
            param_1,
        }
    }
}

#[derive(ShaderType)]
struct ShapeUniform {
    size: Vec2,
    skew: f32,
    rounding: Vec4,
}

impl From<&CellMaterial> for ShapeUniform {
    fn from(value: &CellMaterial) -> Self {
        ShapeUniform {
            size: value.size,
            skew: value.skew,
            rounding: value.rounding,
        }
    }
}
