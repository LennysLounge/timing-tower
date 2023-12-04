use std::any::Any;

use backend::style::StyleDefinition;
use bevy_egui::egui::Ui;
use tree_view::{DropAction, DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::reference_store::ReferenceStore;

use self::tree::{StyleTreeNode, StyleTreeUi, TreeViewAction};

pub mod assets;
pub mod cell;
pub mod folder;
pub mod timing_tower;
pub mod tree;
pub mod variables;

trait AttributeEditor {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool;
}

impl StyleTreeUi for StyleDefinition {
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.assets.tree_view(ui, actions);
            ui.ui.separator();
            self.vars.tree_view(ui, actions);
            ui.ui.separator();
            self.timing_tower.tree_view(ui, actions);
            ui.ui.separator();
        });
    }
}

impl StyleTreeNode for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.vars, &self.timing_tower, &self.assets]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![&mut self.vars, &mut self.timing_tower, &mut self.assets]
    }

    fn can_insert(&self, _node: &dyn Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn Any>, _position: &DropPosition) {}
}

pub trait StyleDefinitionUiThings {
    fn tree_view_elements(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>);
    fn tree_view_variables(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>);
    fn tree_view_assets(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>);
    fn can_drop(&self, drop_action: &DropAction) -> bool;
    fn perform_drop(&mut self, drop_action: &DropAction);
    fn insert(&mut self, target: &Uuid, node: Box<dyn Any>, position: DropPosition);
    fn remove(&mut self, node: &Uuid);
}
impl StyleDefinitionUiThings for StyleDefinition {
    fn tree_view_elements(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.timing_tower.tree_view(ui, actions);
        });
    }
    fn tree_view_variables(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.vars.tree_view(ui, actions);
        });
    }
    fn tree_view_assets(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.assets.tree_view(ui, actions);
        });
    }

    fn can_drop(&self, drop_action: &DropAction) -> bool {
        let dragged = self.find(&drop_action.dragged_node);
        let target = self.find(&drop_action.target_node);
        if let (Some(dragged), Some(target)) = (dragged, target) {
            target.can_insert(dragged.as_any())
        } else {
            false
        }
    }

    fn perform_drop(&mut self, drop_action: &DropAction) {
        if !self.can_drop(drop_action) {
            return;
        };

        let dragged = self
            .find_parent_of(&drop_action.dragged_node)
            .and_then(|parent| parent.remove(&drop_action.dragged_node));
        let target = self.find_mut(&drop_action.target_node);
        if let (Some(dragged), Some(target)) = (dragged, target) {
            target.insert(dragged, &drop_action.position);
        }
    }

    fn insert(&mut self, target: &Uuid, node: Box<dyn Any>, position: DropPosition) {
        if let Some(target) = self.find_mut(&target) {
            target.insert(node, &position);
        } else {
            println!("parent not found id:{}", target);
        }
    }

    fn remove(&mut self, node: &Uuid) {
        self.find_parent_of(&node)
            .map(|parent| parent.remove(&node));
    }
}
