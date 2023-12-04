use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{cell::Cell, folder::Folder};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerTable {
    pub id: Uuid,
    pub cell: Cell,
    pub row_offset: Vec2Property,
    pub row: TimingTowerRow,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub columns: Folder<TimingTowerColumn>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerColumn {
    pub id: Uuid,
    pub cell: Cell,
    pub name: String,
}

impl TimingTowerColumn {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            cell: Cell::default(),
            name: "new column".to_string(),
        }
    }
}
