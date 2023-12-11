use std::any::Any;

use backend::style::{
    folder::Folder,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerRow, TimingTowerTable},
};
use bevy_egui::egui::Ui;
use tree_view::DropPosition;
use uuid::Uuid;

use super::{folder::FolderActions, StyleTreeNode, StyleTreeUi, TreeViewAction};

impl StyleTreeUi for TimingTower {}

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

impl StyleTreeUi for TimingTowerTable {}

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

impl StyleTreeUi for TimingTowerRow {}

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

impl StyleTreeUi for TimingTowerColumn {}

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
