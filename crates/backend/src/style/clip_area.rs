use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{
    cell::Rounding,
    timing_tower::TimingTowerRow,
    visitor::{Node, NodeVisitor, NodeVisitorMut, Visitable, NodeMut},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClipArea<T> {
    pub data: ClipAreaData,
    pub inner: T,
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ClipAreaData {
    pub id: Uuid,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub rounding: Rounding,
}

impl<T> StyleNode for ClipArea<T>
where
    T: StyleNode + Clone + 'static,
    Self: Visitable,
{
    fn id(&self) -> &Uuid {
        &self.data.id
    }
}
impl Visitable for ClipArea<TimingTowerRow> {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.inner.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::ClipArea(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::ClipArea(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.inner.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(NodeMut::ClipArea(self))
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave(NodeMut::ClipArea(self))
    }
}

pub trait DynClipArea: StyleNode {
    fn as_style_node(&self) -> &dyn StyleNode;
    fn as_style_node_mut(&mut self) -> &mut dyn StyleNode;
    fn data(&self) -> &ClipAreaData;
    fn data_mut(&mut self) -> &mut ClipAreaData;
}
impl<T> DynClipArea for ClipArea<T>
where
    T: StyleNode + Clone + 'static,
    Self: Visitable,
{
    fn as_style_node(&self) -> &dyn StyleNode {
        self
    }

    fn as_style_node_mut(&mut self) -> &mut dyn StyleNode {
        self
    }

    fn data(&self) -> &ClipAreaData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut ClipAreaData {
        &mut self.data
    }
}
