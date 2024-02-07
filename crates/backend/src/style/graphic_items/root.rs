use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{Attribute, GraphicItem};

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
