use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{cell::Rounding, Attribute, GraphicItem};

/// An item that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClipArea {
    pub id: Uuid,
    pub name: String,
    pub pos: Attribute<Vec3Property>,
    pub size: Attribute<Vec2Property>,
    pub skew: Attribute<Property<Number>>,
    pub rounding: Attribute<Rounding>,
    pub render_layer: u8,
    pub items: Vec<GraphicItem>,
}
impl ClipArea {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Clip area"),
            pos: Vec3Property::default().into(),
            size: Vec2Property {
                x: Property::Fixed(Number(100.0)),
                y: Property::Fixed(Number(100.0)),
            }
            .into(),
            skew: Property::default().into(),
            rounding: Rounding::default().into(),
            render_layer: 1,
            items: Vec::new(),
        }
    }
}
