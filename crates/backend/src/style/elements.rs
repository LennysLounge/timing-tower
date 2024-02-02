use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use super::cell::{ClipArea, FreeCell};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GraphicItems {
    pub items: Vec<GraphicItem>,
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

/// A visual element that implements some functionality
/// or graphic.
#[derive(Serialize, Deserialize, Clone)]
pub enum GraphicItem {
    Cell(FreeCell),
    ClipArea(FreeClipArea),
}
impl TreeItem for GraphicItem {
    fn id(&self) -> Uuid {
        match self {
            GraphicItem::Cell(cell) => cell.id,
            GraphicItem::ClipArea(clip_area) => clip_area.id,
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
                clip_area.elements.iter().try_for_each(|e| e.walk(f))?;
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
                clip_area
                    .elements
                    .iter_mut()
                    .try_for_each(|e| e.walk_mut(f))?;
            }
        }
        f(self, Method::Leave)
    }
}

/// An element that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FreeClipArea {
    pub id: Uuid,
    pub name: String,
    pub clip_area: ClipArea,
    pub elements: Vec<GraphicItem>,
}
impl FreeClipArea {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Clip area"),
            clip_area: ClipArea::default(),
            elements: Vec::new(),
        }
    }
}
