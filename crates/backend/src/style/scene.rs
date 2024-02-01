use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    component::Component,
    definitions::TimingTower,
    iterator::{Node, NodeMut},
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
