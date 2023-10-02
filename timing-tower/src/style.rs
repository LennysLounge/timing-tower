use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{TreeNode, TreeNodeConverstions};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use self::{
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
    variables::{VariableBehavior, Variables},
};

pub mod cell;
pub mod properties;
pub mod timing_tower;
pub mod variables;

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub vars: Variables,
    pub timing_tower: TimingTower,
}

impl TreeNode for StyleDefinition {
    fn is_directory(&self) -> bool {
        true
    }

    fn show_label(&self, ui: &mut bevy_egui::egui::Ui) {
        ui.label("Style");
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        vec![self.vars.as_dyn(), self.timing_tower.as_dyn()]
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        vec![self.vars.as_dyn_mut(), self.timing_tower.as_dyn_mut()]
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}

impl StyleDefinition {
    pub fn property_editor(&mut self, ui: &mut Ui, variable_repo: &VariableRepo, id: &Uuid) {
        self.find_mut(id).map(|n| n.as_any_mut()).map(|node| {
            if let Some(n) = node.downcast_mut::<VariableBehavior>() {
                n.property_editor(ui, &variable_repo);
            }
            if let Some(n) = node.downcast_mut::<TimingTower>() {
                n.property_editor(ui, &variable_repo);
            }
            if let Some(n) = node.downcast_mut::<TimingTowerTable>() {
                n.property_editor(ui, &variable_repo);
            }
            if let Some(n) = node.downcast_mut::<TimingTowerRow>() {
                n.property_editor(ui, &variable_repo);
            }
            if let Some(n) = node.downcast_mut::<TimingTowerColumn>() {
                n.property_editor(ui, &variable_repo);
            }
        });
    }
}
