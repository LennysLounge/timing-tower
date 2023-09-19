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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellStyleDef {
    pub value_source: ValueSource,
    pub color: Color,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
}
