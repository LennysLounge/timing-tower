use bevy::{
    math::{Vec2, Vec3},
    render::color::Color,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Messages that are send by the controller to renderers.
#[derive(Serialize, Deserialize)]
pub enum ToRendererMessage {
    Assets { images: Vec<(Uuid, String)> },
    Style(Vec<StyleCommand>),
}

/// Messages that are send by renderes to the controller.
#[derive(Serialize, Deserialize)]
pub enum ToControllerMessage {
    Opened,
    AssetsLoaded,
    Debug(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleCommand {
    id: Uuid,
    style: CellStyle,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellStyle {
    pub text: String,
    pub text_color: Color,
    pub text_size: f32,
    pub text_alignment: TextAlignment,
    pub text_position: Vec2,
    pub color: Color,
    pub texture: Option<Uuid>,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
    pub visible: bool,
    pub rounding: [f32; 4],
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}
