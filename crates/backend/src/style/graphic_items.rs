use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
    value_types::{Number, Property, Vec2Property},
};

use super::cell::{ClipArea, FreeCell};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GraphicItems {
    pub items: Vec<GraphicItem>,
    #[serde(default)]
    pub position: Vec2Property,
}

impl TreeIterator for GraphicItems {
    type Item<'item> = GraphicItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        self.items.iter().try_for_each(|e| e.walk(f))
    }
}
impl TreeIteratorMut for GraphicItems {
    type Item<'item> = GraphicItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        self.items.iter_mut().try_for_each(|e| e.walk_mut(f))
    }
}

/// A item inside a graphic that implements some functionality
/// or visual.
#[derive(Serialize, Deserialize, Clone)]
pub enum GraphicItem {
    Cell(FreeCell),
    ClipArea(FreeClipArea),
    DriverTable(DriverTable),
}
impl TreeItem for GraphicItem {
    fn id(&self) -> Uuid {
        match self {
            GraphicItem::Cell(cell) => cell.id,
            GraphicItem::ClipArea(clip_area) => clip_area.id,
            GraphicItem::DriverTable(driver_table) => driver_table.id,
        }
    }
}

impl TreeIterator for GraphicItem {
    type Item<'item> = GraphicItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            GraphicItem::Cell(_) => (),
            GraphicItem::ClipArea(clip_area) => {
                clip_area.items.iter().try_for_each(|e| e.walk(f))?;
            }
            GraphicItem::DriverTable(driver_table) => {
                driver_table.columns.iter().try_for_each(|c| c.walk(f))?;
            }
        }
        f(self, Method::Leave)
    }
}

impl TreeIteratorMut for GraphicItem {
    type Item<'item> = GraphicItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            GraphicItem::Cell(_) => (),
            GraphicItem::ClipArea(clip_area) => {
                clip_area.items.iter_mut().try_for_each(|e| e.walk_mut(f))?;
            }
            GraphicItem::DriverTable(driver_table) => {
                driver_table
                    .columns
                    .iter_mut()
                    .try_for_each(|c| c.walk_mut(f))?;
            }
        }
        f(self, Method::Leave)
    }
}

/// An item that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FreeClipArea {
    pub id: Uuid,
    pub name: String,
    pub clip_area: ClipArea,
    pub items: Vec<GraphicItem>,
}
impl FreeClipArea {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Clip area"),
            clip_area: ClipArea::default(),
            items: Vec::new(),
        }
    }
}

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
