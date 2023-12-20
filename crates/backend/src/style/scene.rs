use std::ops::ControlFlow;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    definitions::TimingTower,
    visitor::{Node, NodeMut, NodeVisitor, NodeVisitorMut, Visitable, Method, VisitableMut},
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
impl Visitable for SceneDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(self.as_node(), Method::Visit)?;
        self.timing_tower.walk(visitor)?;
        visitor.visit(self.as_node(), Method::Leave)
    }
}
impl VisitableMut for SceneDefinition {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.timing_tower.walk_mut(visitor)?;
        visitor.leave(self.as_node_mut())
    }
}
