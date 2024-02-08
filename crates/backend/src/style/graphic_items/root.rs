use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{Attribute, ComputedGraphicItem, GraphicItem};

#[derive(Serialize, Deserialize, Clone)]
pub struct Root {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<GraphicItem>,
    pub position: Attribute<Vec2Property>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Graphic"),
            items: Vec::new(),
            position: Vec2Property::default().into(),
        }
    }

    pub fn compute_for_state(&self, state: Option<&Uuid>) -> ComputedRoot {
        ComputedRoot {
            id: self.id,
            position: self.position.get_state_or_template(state),
            items: self
                .items
                .iter()
                .map(|item| item.compute_for_state(state))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RootAttributes {
    Position(Vec2Property),
}
impl ToString for RootAttributes {
    fn to_string(&self) -> String {
        String::from(match self {
            RootAttributes::Position(_) => "Position",
        })
    }
}

pub struct ComputedRoot {
    pub id: Uuid,
    pub position: Vec2Property,
    pub items: Vec<ComputedGraphicItem>,
}
