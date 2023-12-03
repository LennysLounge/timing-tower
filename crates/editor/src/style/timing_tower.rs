use std::any::Any;

use backend::style::{
    folder::Folder,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
};
use bevy_egui::egui::Ui;
use tree_view::{DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::{properties::PropertyEditor, reference_store::ReferenceStore};

use super::{
    folder::{FolderActions, FolderActionsExtended},
    AttributeEditor, StyleTreeNode, StyleTreeUi, TreeViewAction,
};

impl StyleTreeUi for TimingTower {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        self.cell.property_editor(ui, asset_repo)
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).default_open(true).show(
            ui,
            |ui| {
                ui.label("Timing Tower");
            },
            |ui| {
                self.table.tree_view(ui, actions);
            },
        );
    }
}

impl StyleTreeNode for TimingTower {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.table]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![&mut self.table]
    }

    fn can_insert(&self, _node: &dyn Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn Any>, _position: &DropPosition) {}
}

impl StyleTreeUi for TimingTowerTable {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.row_offset.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.row_offset.y, asset_repo))
                .changed();
        });
        ui.separator();
        changed |= self.cell.property_editor(ui, asset_repo);
        changed
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).default_open(true).show(
            ui,
            |ui| {
                ui.label("Table");
            },
            |ui| {
                self.row.tree_view(ui, actions);
            },
        );
    }
}

impl StyleTreeNode for TimingTowerTable {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.row]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![&mut self.row]
    }

    fn can_insert(&self, _node: &dyn Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn Any>, _position: &DropPosition) {}
}

impl StyleTreeUi for TimingTowerRow {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        self.cell.property_editor(ui, asset_repo)
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let (header, _) = TreeViewBuilder::dir(self.id).default_open(true).show(
            ui,
            |ui| {
                ui.label("Row");
            },
            |ui| {
                self.columns.tree_view_flat(ui, actions);
            },
        );
        header.response.context_menu(|ui| {
            if ui.button("add column").clicked() {
                let column = TimingTowerColumn::new();
                actions.push(TreeViewAction::Select { node: column.id });
                actions.push(TreeViewAction::Insert {
                    target: self.id,
                    node: Box::new(column),
                    position: DropPosition::Last,
                });

                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                let folder = Folder::<TimingTowerColumn>::new();
                actions.push(TreeViewAction::Select { node: folder.id });
                actions.push(TreeViewAction::Insert {
                    target: self.id,
                    node: Box::new(folder),
                    position: DropPosition::First,
                });
                ui.close_menu();
            }
        });
    }
}

impl StyleTreeNode for TimingTowerRow {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.columns]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![&mut self.columns]
    }

    fn can_insert(&self, node: &dyn Any) -> bool {
        node.downcast_ref::<TimingTowerColumn>().is_some()
    }

    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>> {
        self.columns.remove(id)
    }

    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition) {
        self.columns.insert(node, position)
    }
}

impl StyleTreeUi for TimingTowerColumn {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        ui.label("Name:");
        changed |= ui.text_edit_singleline(&mut self.name).changed();
        ui.separator();
        changed |= self.cell.property_editor(ui, asset_repo);
        changed
    }

    fn tree_view(&mut self, tree_ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let res = TreeViewBuilder::leaf(self.id).show(tree_ui, |ui| {
            ui.label(&self.name);
        });

        res.response.context_menu(|ui| {
            if ui.button("add column").clicked() {
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(TimingTowerColumn::new()),
                    position: DropPosition::After(self.id),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(Folder::<TimingTowerColumn>::new()),
                    position: DropPosition::After(self.id),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                actions.push(TreeViewAction::Remove { node: self.id });
                ui.close_menu();
            }
        });
    }
}

impl StyleTreeNode for TimingTowerColumn {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        Vec::new()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        Vec::new()
    }

    fn can_insert(&self, _node: &dyn Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn Any>, _position: &DropPosition) {}
}

impl FolderActions for TimingTowerColumn {
    type FolderType = Self;

    fn context_menu(
        ui: &mut Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add column").clicked() {
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(TimingTowerColumn::new()),
                position: DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("add group").clicked() {
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(Folder::<TimingTowerColumn>::new()),
                position: DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("delete").clicked() {
            actions.push(TreeViewAction::Remove { node: *folder.id() });
            ui.close_menu();
        }
    }
}
