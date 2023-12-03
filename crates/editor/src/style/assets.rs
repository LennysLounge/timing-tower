use bevy_egui::egui::Ui;
use tree_view::{TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::reference_store::{IntoProducerData, ProducerData, ReferenceStore};
use backend::{
    style::{assets::AssetDefinition, folder::Folder},
    value_types::ValueType,
};

use super::{folder::FolderActions, StyleTreeNode, StyleTreeUi, TreeViewAction};

impl FolderActions for AssetDefinition {
    type FolderType = AssetDefinition;

    fn context_menu(
        ui: &mut Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add image").clicked() {
            let image = AssetDefinition {
                id: Uuid::new_v4(),
                name: String::from("Image"),
                value_type: ValueType::Texture,
                path: String::new(),
            };
            actions.push(TreeViewAction::Select { node: *image.id() });
            actions.push(TreeViewAction::Insert {
                target: folder.id,
                node: Box::new(image),
                position: tree_view::DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("add group").clicked() {
            let new_folder = Folder::<AssetDefinition>::new();
            actions.push(TreeViewAction::Select {
                node: *new_folder.id(),
            });
            actions.push(TreeViewAction::Insert {
                target: folder.id,
                node: Box::new(new_folder),
                position: tree_view::DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("delete").clicked() {
            actions.push(TreeViewAction::Remove { node: folder.id });
            ui.close_menu();
        }
    }
}

impl IntoProducerData for AssetDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id.clone(),
            name: self.name.clone(),
            value_type: self.value_type.clone(),
        }
    }
}

impl StyleTreeUi for AssetDefinition {
    fn tree_view(&mut self, tree_ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let res = TreeViewBuilder::leaf(self.id).show(tree_ui, |ui| {
            ui.label(&self.name);
        });
        res.response.context_menu(|ui| {
            if ui.button("add image").clicked() {
                let image = AssetDefinition {
                    id: Uuid::new_v4(),
                    name: String::from("Image"),
                    value_type: ValueType::Texture,
                    path: String::new(),
                };
                actions.push(TreeViewAction::Select { node: *image.id() });
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(image),
                    position: tree_view::DropPosition::After(self.id),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                let folder = Folder::<AssetDefinition>::new();
                actions.push(TreeViewAction::Select { node: *folder.id() });
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(folder),
                    position: tree_view::DropPosition::After(self.id),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                actions.push(TreeViewAction::Remove { node: self.id });
                ui.close_menu();
            }
        });
    }

    fn property_editor(&mut self, ui: &mut Ui, _asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.label("Name");
        changed |= ui.text_edit_singleline(&mut self.name).changed();
        ui.separator();
        ui.label("Path:");
        changed |= ui.text_edit_singleline(&mut self.path).changed();
        // Feature: Show load state of assets
        // match self.load_state {
        //     LoadState::NotLoaded => ui.label("Asset is not loaded"),
        //     LoadState::Loading => ui.label("Asset is loading"),
        //     LoadState::Loaded => ui.label("Asset loaded correctly"),
        //     LoadState::Failed => ui.label(
        //         "Failed to load the asset. Make sure the path is pointing to a valid image file.",
        //     ),
        // };
        changed
    }
}

impl StyleTreeNode for AssetDefinition {
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
