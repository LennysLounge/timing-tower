use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{Number, Property, Vec2Property, Vec3Property};

use super::{
    cell::Rounding,
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    timing_tower::TimingTowerRow,
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
    pub render_layer: u8,
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
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.inner.walk_mut(f)?;
        f(self.as_node_mut(), Method::Leave)
    }
}
