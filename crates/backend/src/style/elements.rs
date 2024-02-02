use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use super::cell::{ClipArea, FreeCell};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Elements {
    pub elements: Vec<Element>,
}

impl TreeIterator for Elements {
    type Item<'item> = Element;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        self.elements.iter().try_for_each(|e| e.walk(f))
    }
}
impl TreeIteratorMut for Elements {
    type Item<'item> = Element;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        self.elements.iter_mut().try_for_each(|e| e.walk_mut(f))
    }
}

/// A visual element that implements some functionality
/// or graphic.
#[derive(Serialize, Deserialize, Clone)]
pub enum Element {
    Cell(FreeCell),
    ClipArea(FreeClipArea),
}
impl TreeItem for Element {
    fn id(&self) -> Uuid {
        match self {
            Element::Cell(cell) => cell.id,
            Element::ClipArea(clip_area) => clip_area.id,
        }
    }
}

impl TreeIterator for Element {
    type Item<'item> = Element;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            Element::Cell(_) => (),
            Element::ClipArea(clip_area) => {
                clip_area.elements.iter().try_for_each(|e| e.walk(f))?;
            }
        }
        f(self, Method::Leave)
    }
}

impl TreeIteratorMut for Element {
    type Item<'item> = Element;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            Element::Cell(_) => (),
            Element::ClipArea(clip_area) => {
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
    pub elements: Vec<Element>,
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
