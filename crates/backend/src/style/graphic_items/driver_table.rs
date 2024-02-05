use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property};

use super::GraphicItem;

// An item that displays a table of all drivers in the session.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DriverTable {
    pub id: Uuid,
    pub name: String,
    pub row_offset: Vec2Property,
    pub columns: Vec<GraphicItem>,
}
impl DriverTable {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Driver table"),
            row_offset: Vec2Property {
                x: Property::Fixed(Number(30.0)),
                y: Property::Fixed(Number(30.0)),
            },
            columns: Vec::new(),
        }
    }
}
