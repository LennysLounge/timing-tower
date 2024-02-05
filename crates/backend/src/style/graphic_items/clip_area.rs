use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{cell::Rounding, GraphicItem};

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
        }
    }
}
