use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{
    cell::Rounding,
    timing_tower::TimingTowerRow,
    visitor::{Method, Node, NodeMut, NodeVisitor, NodeVisitorMut, Visitable},
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

impl StyleNode for ClipArea<TimingTowerRow> {
    fn id(&self) -> &Uuid {
        &self.data.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::ClipArea(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::ClipArea(self)
    }
}
impl Visitable for ClipArea<TimingTowerRow> {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(self.as_node(), Method::Visit)?;
        self.inner.walk(visitor)?;
        visitor.visit(self.as_node(), Method::Leave)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.inner.walk_mut(visitor)?;
        visitor.leave(self.as_node_mut())
    }
}

pub trait DynClipArea: StyleNode {
    fn as_style_node(&self) -> &dyn StyleNode;
    fn as_style_node_mut(&mut self) -> &mut dyn StyleNode;
    fn data(&self) -> &ClipAreaData;
    fn data_mut(&mut self) -> &mut ClipAreaData;
}
impl DynClipArea for ClipArea<TimingTowerRow> {
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
