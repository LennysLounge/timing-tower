use std::ops::ControlFlow;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    component::Component,
    definitions::TimingTower,
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SceneDefinition {
    pub id: Uuid,
    pub prefered_size: Vec2,
    pub timing_tower: TimingTower,
    pub components: Vec<Component>,
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
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.timing_tower.walk(f)?;
        self.components.iter().try_for_each(|c| c.walk(f))?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for SceneDefinition {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.timing_tower.walk_mut(f)?;
        self.components.iter_mut().try_for_each(|c| c.walk_mut(f))?;
        f(self.as_node_mut(), Method::Leave)
    }
}
