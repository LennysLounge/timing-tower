use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{
    cell::Rounding,
    timing_tower::TimingTowerRow,
    visitor::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
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
impl NodeIterator for ClipArea<TimingTowerRow> {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.inner.walk(f)?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for ClipArea<TimingTowerRow> {
    fn walk_mut<F>(&mut self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<()>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.inner.walk_mut(f)?;
        f(self.as_node_mut(), Method::Leave)
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
