use std::any::Any;

use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::{cell::Cell, properties::Vec2Property, StyleTreeNode, StyleTreeUi, TreeViewAction};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}

impl StyleTreeUi for TimingTower {
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).show(
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

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerTable {
    pub id: Uuid,
    pub cell: Cell,
    pub row_offset: Vec2Property,
    pub row: TimingTowerRow,
}

impl StyleTreeUi for TimingTowerTable {
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            self.row_offset.x.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            self.row_offset.y.editor(ui, vars);
        });
        ui.separator();
        self.cell.property_editor(ui, vars);
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).show(
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

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub columns: Vec<TimingTowerColumn>,
}

impl StyleTreeUi for TimingTowerRow {
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }

    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let (header, _) = TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label("Timing Tower");
            },
            |ui| {
                for c in self.columns.iter_mut() {
                    c.tree_view(ui, actions);
                }
            },
        );
        header.response.context_menu(|ui| {
            if ui.button("add column").clicked() {
                actions.push(TreeViewAction::Insert {
                    target: self.id,
                    node: Box::new(TimingTowerColumn::new()),
                    position: DropPosition::Last,
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
        self.columns
            .iter()
            .map(|c| c as &dyn StyleTreeNode)
            .collect()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        self.columns
            .iter_mut()
            .map(|c| c as &mut dyn StyleTreeNode)
            .collect()
    }

    fn can_insert(&self, node: &dyn Any) -> bool {
        node.downcast_ref::<TimingTowerColumn>().is_some()
    }

    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>> {
        if let Some(pos) = self.columns.iter().position(|c| &c.id == id) {
            let n = self.columns.remove(pos);
            Some(Box::new(n))
        } else {
            None
        }
    }

    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition) {
        if let Ok(column) = node.downcast::<TimingTowerColumn>() {
            match position {
                DropPosition::First => self.columns.insert(0, *column),
                DropPosition::Last => self.columns.push(*column),
                DropPosition::After(id) => {
                    let pos = self
                        .columns
                        .iter()
                        .position(|c| &c.id == id)
                        .unwrap_or(self.columns.len());
                    self.columns.insert(pos + 1, *column);
                }
                DropPosition::Before(id) => {
                    let pos = self
                        .columns
                        .iter()
                        .position(|c| &c.id == id)
                        .unwrap_or(self.columns.len());
                    self.columns.insert(pos, *column);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerColumn {
    pub id: Uuid,
    pub cell: Cell,
    pub name: String,
}

impl StyleTreeUi for TimingTowerColumn {
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        self.cell.property_editor(ui, vars);
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

impl TimingTowerColumn {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            cell: Cell::default(),
            name: "new column".to_string(),
        }
    }
}
