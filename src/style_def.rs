use std::collections::HashMap;

use bevy::prelude::{Color, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerStyleDef {
    pub cell: CellStyleDef,
    pub table: TableStyleDef,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TableStyleDef {
    pub cell: CellStyleDef,
    pub row_offset: Vec2,
    pub row_style: RowStyleDef,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RowStyleDef {
    pub cell: CellStyleDef,
    pub columns: HashMap<String, CellStyleDef>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ValueSource {
    FixedValue(String),
    DriverName,
    Position,
    CarNumber,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellStyleDef {
    pub value_source: ValueSource,
    pub color: Color,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
    pub visible: bool,
    pub rounding: Rounding,
    pub text_alginment: TextAlignment,
    pub text_position: Vec2,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Rounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bot_left: f32,
    pub bot_right: f32,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}
