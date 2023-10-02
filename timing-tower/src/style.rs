use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use tree_view::{TreeNode, TreeNodeConverstions};
use uuid::Uuid;

use self::variables::Variables;

pub mod variables;

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub vars: Variables,
    // pub timing_tower: TimingTowerElement,
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
        vec![self.vars.as_dyn()]
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        vec![self.vars.as_dyn_mut()]
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}
