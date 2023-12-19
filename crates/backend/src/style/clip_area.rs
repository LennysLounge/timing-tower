use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    visitor::{NodeVisitor, NodeVisitorMut, Visitable},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ClipArea<T>
where
    T: StyleNode + 'static,
{
    pub data: ClipAreaData,
    pub inner: T,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ClipAreaData {
    pub id: Uuid,
}

impl<T> StyleNode for ClipArea<T>
where
    T: StyleNode + Clone + 'static,
{
    fn id(&self) -> &Uuid {
        &self.data.id
    }
}
impl<T> Visitable for ClipArea<T>
where
    T: StyleNode + Clone + 'static,
{
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.inner.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_clip_area(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_clip_area(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.inner.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_clip_area(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_clip_area(self)
    }
}

pub trait DynClipArea: StyleNode {
    fn as_style_node(&self) -> &dyn StyleNode;
    fn data(&self) -> &ClipAreaData;
}
impl<T> DynClipArea for ClipArea<T>
where
    T: StyleNode + Clone,
{
    fn as_style_node(&self) -> &dyn StyleNode {
        self
    }

    fn data(&self) -> &ClipAreaData {
        &self.data
    }
}
