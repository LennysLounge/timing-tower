use serde::{Deserialize, Serialize};

use crate::{
    style::graphic::GraphicStateId,
    value_types::{Number, Property, Vec2Property},
};

use super::{Attribute, ComputedGraphicItem, GraphicItem, GraphicItemId};

// An item that displays a table of all drivers in the session.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DriverTable {
    pub id: GraphicItemId,
    pub name: String,
    #[serde(default)]
    pub position: Attribute<Vec2Property>,
    pub row_offset: Attribute<Vec2Property>,
    pub columns: Vec<GraphicItem>,
}
impl DriverTable {
    pub fn new() -> Self {
        Self {
            id: GraphicItemId::new(),
            name: String::from("Driver table"),
            row_offset: Vec2Property {
                x: Property::Fixed(Number(30.0)),
                y: Property::Fixed(Number(30.0)),
            }
            .into(),
            columns: Vec::new(),
            position: Vec2Property {
                x: Property::Fixed(Number(0.0)),
                y: Property::Fixed(Number(0.0)),
            }
            .into(),
        }
    }
    pub fn compute_for_state(&self, state: Option<&GraphicStateId>) -> ComputedDriverTable {
        ComputedDriverTable {
            id: self.id,
            position: self.position.get_state_or_template(state),
            row_offset: self.row_offset.get_state_or_template(state),
            columns: self
                .columns
                .iter()
                .map(|item| item.compute_for_state(state))
                .collect(),
        }
    }
}

pub struct ComputedDriverTable {
    pub id: GraphicItemId,
    pub position: Vec2Property,
    pub row_offset: Vec2Property,
    pub columns: Vec<ComputedGraphicItem>,
}
