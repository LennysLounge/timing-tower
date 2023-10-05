use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::tree_view_2::TreeUi;
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
}

impl StyleDefinition {
    pub fn tree_view(&self, ui: &mut TreeUi) {
        self.vars.tree_view(ui);
        self.timing_tower.tree_view(ui);
    }

    // pub fn property_editor(&mut self, ui: &mut Ui, variable_repo: &VariableRepo, id: &Uuid) {
    //     self.find_mut(id).map(|n| n.as_any_mut()).map(|node| {
    //         if let Some(n) = node.downcast_mut::<VariableBehavior>() {
    //             n.property_editor(ui, &variable_repo);
    //         }
    //         if let Some(n) = node.downcast_mut::<TimingTower>() {
    //             n.property_editor(ui, &variable_repo);
    //         }
    //         if let Some(n) = node.downcast_mut::<TimingTowerTable>() {
    //             n.property_editor(ui, &variable_repo);
    //         }
    //         if let Some(n) = node.downcast_mut::<TimingTowerRow>() {
    //             n.property_editor(ui, &variable_repo);
    //         }
    //         if let Some(n) = node.downcast_mut::<TimingTowerColumn>() {
    //             n.property_editor(ui, &variable_repo);
    //         }
    //     });
    // }
}
