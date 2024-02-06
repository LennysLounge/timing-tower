use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{EnumSet, GraphicItem};

#[derive(Serialize, Deserialize, Clone)]
pub struct Root {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<GraphicItem>,
    pub position: Vec2Property,
    pub attributes: HashMap<Uuid, EnumSet<RootAttributes>>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Graphic"),
            items: Vec::new(),
            position: Vec2Property::default(),
            attributes: HashMap::new(),
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
