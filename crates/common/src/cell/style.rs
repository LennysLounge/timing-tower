use bevy::prelude::{Color, Entity, Event, Handle, Image, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct SetStyle {
    pub entity: Entity,
    pub style: CellStyle,
}

pub struct CellStyle {
    pub text: String,
    pub text_color: Color,
    pub text_size: f32,
    pub text_alignment: TextAlignment,
    pub text_position: Vec2,
    pub color: Color,
    pub texture: Option<Handle<Image>>,
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

#[derive(Serialize, Deserialize)]
pub struct CellStyleMessage {
    pub text: String,
    pub text_color: Color,
    pub text_size: f32,
    pub text_alignment: TextAlignment,
    pub text_position: Vec2,
    pub color: Color,
    //pub texture: Option<Handle<Image>>,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
    pub visible: bool,
    pub rounding: [f32; 4],
}
