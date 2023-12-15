use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{
    definitions::*,
    scene::SceneDefinition,
    visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable},
};

pub mod assets;
pub mod cell;
pub mod folder;
pub mod scene;
pub mod timing_tower;
pub mod variables;
pub mod visitor;

pub mod definitions {
    pub use self::super::{
        assets::AssetDefinition,
        folder::{Folder, FolderInfo},
        scene::SceneDefinition,
        timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow},
        variables::VariableDefinition,
        StyleDefinition,
    };
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: Folder<AssetDefinition>,
    pub vars: Folder<VariableDefinition>,
    pub scene: SceneDefinition,
}
impl StyleNode for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
impl Visitable for StyleDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.assets.walk(visitor)?;
        self.vars.walk(visitor)?;
        self.scene.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_style(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_style(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.assets.walk_mut(visitor)?;
        self.vars.walk_mut(visitor)?;
        self.scene.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_style(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_style(self)
    }
}
