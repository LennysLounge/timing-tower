use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::TreeUi;
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use self::{timing_tower::TimingTower, variables::Variables};

pub mod cell;
pub mod properties;
pub mod timing_tower;
pub mod variables;

pub trait TreeNode {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode>;
    #[allow(unused)]
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {}
    fn tree_view(&mut self, ui: &mut TreeUi);
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub vars: Variables,
    pub timing_tower: TimingTower,
}

impl TreeNode for StyleDefinition {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self)
        } else {
            self.vars
                .find_mut(id)
                .or_else(|| self.timing_tower.find_mut(id))
        }
    }
    fn tree_view(&mut self, ui: &mut TreeUi) {
        self.vars.tree_view(ui);
        self.timing_tower.tree_view(ui);
    }
}
