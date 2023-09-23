use std::collections::HashMap;

use bevy::prelude::{Color, Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SceneStyleDef {
    pub vars: HashMap<String, VariableDef>,
    pub timing_tower: TimingTowerStyleDef,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableDef {
    Number(f32),
    Text(String),
    Color(Color),
}

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
    pub columns: Vec<ColumnStyleDef>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColumnStyleDef {
    pub cell: CellStyleDef,
    pub name: String,
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

impl Default for CellStyleDef {
    fn default() -> Self {
        Self {
            value_source: ValueSource::FixedValue("Column".to_string()),
            color: Color::PURPLE,
            pos: Vec3::new(10.0, 10.0, 0.0),
            size: Vec2::new(30.0, 30.0),
            skew: 0.0,
            visible: true,
            rounding: Rounding::default(),
            text_alginment: TextAlignment::default(),
            text_position: Vec2::new(5.0, 15.0),
        }
    }
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

#[derive(Serialize, Deserialize, Clone)]

pub enum PropertyValue {
    VarRef(String),
    #[serde(untagged)]
    Text(String),
    #[serde(untagged)]
    Color(Color),
    #[serde(untagged)]
    Number(f32),
}
