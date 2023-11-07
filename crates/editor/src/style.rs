use std::any::Any;

use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{DropAction, DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::asset_reference_repo::AssetReferenceRepo;

use self::{
    assets::AssetDefinition, folder::Folder, timing_tower::TimingTower, variables::VariableBehavior,
};

pub mod assets;
pub mod cell;
pub mod folder;
pub mod properties;
pub mod timing_tower;
pub mod variables;

pub enum TreeViewAction {
    Insert {
        target: Uuid,
        node: Box<dyn Any>,
        position: DropPosition,
    },
    Remove {
        node: Uuid,
    },
    Select {
        node: Uuid,
    },
}

pub trait StyleTreeUi {
    #[allow(unused)]
    /// Display the property editor for this tree node and return true if any fields where changed
    /// Returns falls if no changes were made to the node.
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        false
    }
    /// Display the tree view node of this node.
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>);
}

pub trait StyleTreeNodeConversions: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_mut(&mut self) -> &mut dyn StyleTreeNode;
}
impl<T: StyleTreeNode + Any> StyleTreeNodeConversions for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_mut(&mut self) -> &mut dyn StyleTreeNode {
        self
    }
}

pub trait StyleTreeNode: StyleTreeNodeConversions + StyleTreeUi {
    fn id(&self) -> &Uuid;
    fn chidren(&self) -> Vec<&dyn StyleTreeNode>;
    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode>;

    fn can_insert(&self, node: &dyn Any) -> bool;
    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>>;
    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition);

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleTreeNode> {
        self.chidren_mut().into_iter().find_map(|c| {
            if c.id() == id {
                Some(c)
            } else {
                c.find_mut(id)
            }
        })
    }
    fn find(&self, id: &Uuid) -> Option<&dyn StyleTreeNode> {
        self.chidren()
            .into_iter()
            .find_map(|c| if c.id() == id { Some(c) } else { c.find(id) })
    }
    fn find_parent_of(&mut self, id: &Uuid) -> Option<&mut dyn StyleTreeNode> {
        if self.chidren().into_iter().any(|c| c.id() == id) {
            Some(self.as_dyn_mut())
        } else {
            self.chidren_mut()
                .into_iter()
                .find_map(|c| c.find_parent_of(id))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: Folder<AssetDefinition>,
    pub vars: Folder<VariableBehavior>,
    pub timing_tower: TimingTower,
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

impl StyleDefinition {
    pub fn tree_view_elements(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.timing_tower.tree_view(ui, actions);
        });
    }
    pub fn tree_view_variables(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.vars.tree_view(ui, actions);
        });
    }
    pub fn tree_view_assets(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            self.assets.tree_view(ui, actions);
        });
    }

    pub fn can_drop(&self, drop_action: &DropAction) -> bool {
        let dragged = self.find(&drop_action.dragged_node);
        let target = self.find(&drop_action.target_node);
        if let (Some(dragged), Some(target)) = (dragged, target) {
            target.can_insert(dragged.as_any())
        } else {
            false
        }
    }

    pub fn perform_drop(&mut self, drop_action: &DropAction) {
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

    pub fn insert(&mut self, target: &Uuid, node: Box<dyn Any>, position: DropPosition) {
        if let Some(target) = self.find_mut(&target) {
            target.insert(node, &position);
        } else {
            println!("parent not found id:{}", target);
        }
    }

    pub fn remove(&mut self, node: &Uuid) {
        self.find_parent_of(&node)
            .map(|parent| parent.remove(&node));
    }
}
