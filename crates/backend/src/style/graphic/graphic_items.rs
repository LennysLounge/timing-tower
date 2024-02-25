pub mod cell;
pub mod clip_area;
pub mod driver_table;
pub mod entry_context;
pub mod root;

use std::{
    collections::HashMap,
    ops::{ControlFlow, Deref, DerefMut},
};

use enumcapsulate::macros::Encapsulate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use self::{
    cell::{Cell, ComputedCell},
    clip_area::{ClipArea, ComputedClipArea},
    driver_table::{ComputedDriverTable, DriverTable},
    entry_context::{ComputedEntryContext, EntryContext},
    root::{ComputedRoot, Root},
};

use super::GraphicStateId;

/// Id that identifies a graphic item.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GraphicItemId(pub Uuid);
impl GraphicItemId {
    pub fn new() -> Self {
        GraphicItemId(Uuid::new_v4())
    }
}

/// A item inside a graphic that implements some functionality
/// or visual.
#[derive(Serialize, Deserialize, Clone, Encapsulate)]
#[serde(tag = "graphic_item_type")]
pub enum GraphicItem {
    Root(Root),
    Cell(Cell),
    ClipArea(ClipArea),
    DriverTable(DriverTable),
    EntryContext(EntryContext),
}
impl GraphicItem {
    pub fn compute_for_state(&self, state: Option<&GraphicStateId>) -> ComputedGraphicItem {
        match self {
            GraphicItem::Root(o) => ComputedGraphicItem::Root(o.compute_for_state(state)),
            GraphicItem::Cell(o) => ComputedGraphicItem::Cell(o.compute_for_state(state)),
            GraphicItem::ClipArea(o) => ComputedGraphicItem::ClipArea(o.compute_for_state(state)),
            GraphicItem::DriverTable(o) => {
                ComputedGraphicItem::DriverTable(o.compute_for_state(state))
            }
            GraphicItem::EntryContext(o) => {
                ComputedGraphicItem::EntryContext(o.compute_for_state(state))
            }
        }
    }
}

impl TreeItem for GraphicItem {
    type Id = GraphicItemId;

    fn id(&self) -> Self::Id {
        match self {
            GraphicItem::Root(root) => root.id,
            GraphicItem::Cell(cell) => cell.id,
            GraphicItem::ClipArea(clip_area) => clip_area.id,
            GraphicItem::DriverTable(driver_table) => driver_table.id,
            GraphicItem::EntryContext(entry_context) => entry_context.id,
        }
    }
}

impl TreeIterator for GraphicItem {
    type Item = GraphicItem;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item, Method) -> ControlFlow<R>,
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
            GraphicItem::EntryContext(entry_context) => {
                entry_context.items.iter().try_for_each(|e| e.walk(f))?;
            }
        }
        f(self, Method::Leave)
    }
}

impl TreeIteratorMut for GraphicItem {
    type Item = GraphicItem;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item, Method) -> ControlFlow<R>,
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
            GraphicItem::EntryContext(entry_context) => {
                entry_context
                    .items
                    .iter_mut()
                    .try_for_each(|e| e.walk_mut(f))?;
            }
        }
        f(self, Method::Leave)
    }
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Attribute<T> {
    template: T,
    states: HashMap<GraphicStateId, T>,
}
impl<T> Attribute<T> {
    pub fn template(&self) -> &T {
        &self.template
    }
    pub fn template_mut(&mut self) -> &mut T {
        &mut self.template
    }

    pub fn get_state(&mut self, state_id: &GraphicStateId) -> Option<&mut T> {
        self.states.get_mut(state_id)
    }
    pub fn add_state(&mut self, state_id: GraphicStateId)
    where
        T: Clone,
    {
        self.states.insert(state_id, self.template.clone());
    }
    pub fn remove_state(&mut self, state_id: &GraphicStateId) {
        self.states.remove(state_id);
    }
    pub fn has_state(&self, state_id: &GraphicStateId) -> bool {
        self.states.contains_key(&state_id)
    }
    pub fn get_state_or_template(&self, state_id: Option<&GraphicStateId>) -> T
    where
        T: Clone,
    {
        state_id
            .and_then(|id| self.states.get(id))
            .map(|maybe_state| maybe_state.clone())
            .unwrap_or_else(|| self.template.clone())
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

pub enum ComputedGraphicItem {
    Root(ComputedRoot),
    Cell(ComputedCell),
    ClipArea(ComputedClipArea),
    DriverTable(ComputedDriverTable),
    EntryContext(ComputedEntryContext),
}
