pub mod cell;
pub mod clip_area;
pub mod driver_table;
pub mod root;

use std::{collections::HashMap, ops::ControlFlow};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use self::{cell::Cell, clip_area::ClipArea, driver_table::DriverTable, root::Root};

/// A item inside a graphic that implements some functionality
/// or visual.
#[derive(Serialize, Deserialize, Clone)]
pub enum GraphicItem {
    Root(Root),
    Cell(Cell),
    ClipArea(ClipArea),
    DriverTable(DriverTable),
}
impl TreeItem for GraphicItem {
    fn id(&self) -> Uuid {
        match self {
            GraphicItem::Root(root) => root.id,
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
            GraphicItem::Root(root) => {
                root.items.iter().try_for_each(|e| e.walk(f))?;
            }
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
            GraphicItem::Root(root) => {
                root.items.iter_mut().try_for_each(|e| e.walk_mut(f))?;
            }
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

#[derive(Serialize, Deserialize, Clone)]
pub struct EnumSet<T: Serialize + ToString> {
    data: HashMap<String, T>,
}
impl<T: Serialize + ToString> EnumSet<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn insert(&mut self, value: T) {
        self.data.insert(value.to_string(), value);
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }
    pub fn remove(&mut self, key: &String) -> Option<T> {
        self.data.remove(key)
    }
}
impl<T: Serialize + ToString> Default for EnumSet<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
