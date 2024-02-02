use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{component::Component, timing_tower::TimingTower, Node, NodeMut, OwnedNode, StyleNode};

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
    fn to_node(self) -> OwnedNode {
        OwnedNode::Scene(self)
    }
}
