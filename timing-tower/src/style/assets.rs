use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::asset_reference_repo::AssetReferenceRepo;

use super::{
    folder::{Folder, FolderActions},
    StyleTreeNode, StyleTreeUi, TreeViewAction,
};

#[derive(Serialize, Deserialize, Clone)]
pub enum AssetDefinition {
    Image(ImageAsset),
}

impl FolderActions for AssetDefinition {
    type FolderType = AssetDefinition;

    fn context_menu(
        ui: &mut Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add image").clicked() {
            let image = AssetDefinition::Image(ImageAsset::new());
            actions.push(TreeViewAction::Select { node: *image.id() });
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(image),
                position: tree_view::DropPosition::Last,
            });
            ui.close_menu();
        }
    }
}

impl StyleTreeUi for AssetDefinition {
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        match self {
            AssetDefinition::Image(o) => o.tree_view(ui, actions),
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        match self {
            AssetDefinition::Image(o) => o.property_editor(ui, asset_repo),
        }
    }
}

impl StyleTreeNode for AssetDefinition {
    fn id(&self) -> &uuid::Uuid {
        match self {
            AssetDefinition::Image(o) => o.id(),
        }
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        match self {
            AssetDefinition::Image(o) => o.chidren(),
        }
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        match self {
            AssetDefinition::Image(o) => o.chidren_mut(),
        }
    }

    fn can_insert(&self, node: &dyn std::any::Any) -> bool {
        match self {
            AssetDefinition::Image(o) => o.can_insert(node),
        }
    }

    fn remove(&mut self, id: &uuid::Uuid) -> Option<Box<dyn std::any::Any>> {
        match self {
            AssetDefinition::Image(o) => o.remove(id),
        }
    }

    fn insert(&mut self, node: Box<dyn std::any::Any>, position: &tree_view::DropPosition) {
        match self {
            AssetDefinition::Image(o) => o.insert(node, position),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageAsset {
    pub id: Uuid,
    pub name: String,
}

impl StyleTreeUi for ImageAsset {
    fn tree_view(&mut self, ui: &mut TreeUi, _actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::leaf(self.id).show(ui, |ui| {
            ui.label(&self.name);
        });
    }

    fn property_editor(&mut self, ui: &mut Ui, _asset_repo: &AssetReferenceRepo) {
        ui.label("Name");
        ui.text_edit_singleline(&mut self.name);
    }
}

impl StyleTreeNode for ImageAsset {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        Vec::new()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        Vec::new()
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn std::any::Any>, _position: &tree_view::DropPosition) {}
}

impl ImageAsset {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "new image".to_string(),
        }
    }
}
