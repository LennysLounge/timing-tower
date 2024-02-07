pub mod cell;
pub mod clip_area;
pub mod driver_table;
pub mod root;

use std::{
    collections::HashMap,
    ops::{ControlFlow, Deref, DerefMut},
};

use enumcapsulate::macros::Encapsulate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use self::{cell::Cell, clip_area::ClipArea, driver_table::DriverTable, root::Root};

/// A item inside a graphic that implements some functionality
/// or visual.
#[derive(Serialize, Deserialize, Clone, Encapsulate)]
#[serde(tag = "graphic_item_type")]
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
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Attribute<T> {
    template: T,
    states: HashMap<Uuid, T>,
}
impl<T> Attribute<T> {
    pub fn template(&self) -> &T {
        &self.template
    }
    pub fn template_mut(&mut self) -> &mut T {
        &mut self.template
    }

    pub fn get_state(&mut self, state_id: &Uuid) -> Option<&mut T> {
        self.states.get_mut(state_id)
    }
    pub fn add_state(&mut self, state_id: Uuid)
    where
        T: Clone,
    {
        self.states.insert(state_id, self.template.clone());
    }
    pub fn remove_state(&mut self, state_id: &Uuid) {
        self.states.remove(state_id);
    }
    pub fn has_state(&self, state_id: &Uuid) -> bool {
        self.states.contains_key(&state_id)
    }
}
impl<T> Deref for Attribute<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.template()
    }
}
impl<T> DerefMut for Attribute<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.template_mut()
    }
}

impl<T> From<T> for Attribute<T> {
    fn from(value: T) -> Self {
        Self {
            template: value,
            states: HashMap::new(),
        }
    }
}
