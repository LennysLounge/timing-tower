use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{cell::Rounding, EnumSet, GraphicItem};

/// An item that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClipArea {
    pub id: Uuid,
    pub name: String,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub rounding: Rounding,
    pub render_layer: u8,
    pub items: Vec<GraphicItem>,
    pub attributes: HashMap<Uuid, EnumSet<ClipAreaAttributes>>,
}
impl ClipArea {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Clip area"),
            pos: Vec3Property::default(),
            size: Vec2Property {
                x: Property::Fixed(Number(100.0)),
                y: Property::Fixed(Number(100.0)),
            },
            skew: Property::default(),
            rounding: Rounding::default(),
            render_layer: 1,
            items: Vec::new(),
            attributes: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClipAreaAttributes {
    Position(Vec3Property),
    Size(Vec2Property),
    Skew(Property<Number>),
    Rounding(Rounding),
}
impl ToString for ClipAreaAttributes {
    fn to_string(&self) -> String {
        String::from(match self {
            ClipAreaAttributes::Position(_) => "Position",
            ClipAreaAttributes::Size(_) => "Size",
            ClipAreaAttributes::Skew(_) => "Skew",
            ClipAreaAttributes::Rounding(_) => "Rounding",
        })
    }
}
