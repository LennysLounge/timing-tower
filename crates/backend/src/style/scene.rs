use std::ops::ControlFlow;

use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    definitions::TimingTower,
    visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable},
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
}
impl Visitable for SceneDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.timing_tower.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_scene(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_scene(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.timing_tower.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_scene(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_scene(self)
    }
}
