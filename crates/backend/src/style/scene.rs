use std::ops::ControlFlow;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    definitions::TimingTower,
    visitor::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SceneDefinition {
    pub id: Uuid,
    pub prefered_size: Vec2,
    pub timing_tower: TimingTower,
}
impl StyleNode for SceneDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Scene(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Scene(self)
    }
}
impl NodeIterator for SceneDefinition {
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)?;
        self.timing_tower.walk(f)?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for SceneDefinition {
    fn walk_mut<F>(&mut self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<()>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.timing_tower.walk_mut(f)?;
        f(self.as_node_mut(), Method::Leave)
    }
}
