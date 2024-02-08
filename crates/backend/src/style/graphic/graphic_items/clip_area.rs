use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{cell::Rounding, Attribute, ComputedGraphicItem, GraphicItem};

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
    pub fn compute_for_state(&self, state: Option<&Uuid>) -> ComputedClipArea {
        ComputedClipArea {
            id: self.id,
            pos: self.pos.get_state_or_template(state),
            size: self.size.get_state_or_template(state),
            skew: self.skew.get_state_or_template(state),
            rounding: self.rounding.get_state_or_template(state),
            render_layer: self.render_layer,
            items: self
                .items
                .iter()
                .map(|item| item.compute_for_state(state))
                .collect(),
        }
    }
}

pub struct ComputedClipArea {
    pub id: Uuid,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub rounding: Rounding,
    pub render_layer: u8,
    pub items: Vec<ComputedGraphicItem>,
}
