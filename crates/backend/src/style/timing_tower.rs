use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::{ClipArea, FreeCell, FreeCellFolder},
    StyleItemRef, StyleItemMut, OwnedStyleItem, StyleItem,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub position: Vec2Property,
    pub row: TimingTowerRow,
    #[serde(default)]
    pub cells: FreeCellFolder,
}
impl StyleItem for TimingTower {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::TimingTower(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::TimingTower(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::TimingTower(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub row_offset: Vec2Property,
    pub clip_area: ClipArea,
    pub columns: FreeCellFolder,
}
impl TimingTowerRow {
    pub fn contained_cells(&self) -> Vec<&FreeCell> {
        self.columns.contained_cells()
    }
}
impl StyleItem for TimingTowerRow {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::TimingTowerRow(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::TimingTowerRow(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::TimingTowerRow(self)
    }
}
