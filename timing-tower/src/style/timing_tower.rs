use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::tree_view_2::{TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::{cell::Cell, properties::Vec2Property, TreeNode};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}

impl TreeNode for TimingTower {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self)
        } else {
            self.table.find_mut(id)
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }
}

impl TimingTower {
    pub fn tree_view(&self, ui: &mut TreeUi) {
        TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label("Timing Tower");
            },
            |ui| {
                self.table.tree_view(ui);
            },
        );
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerTable {
    pub id: Uuid,
    pub cell: Cell,
    pub row_offset: Vec2Property,
    pub row: TimingTowerRow,
}

impl TreeNode for TimingTowerTable {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self)
        } else {
            self.row.find_mut(id)
        }
    }
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
}

impl TimingTowerTable {
    pub fn tree_view(&self, ui: &mut TreeUi) {
        TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label("Table");
            },
            |ui| {
                self.row.tree_view(ui);
            },
        );
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub columns: Vec<TimingTowerColumn>,
}
impl TreeNode for TimingTowerRow {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self)
        } else {
            self.columns.iter_mut().find_map(|c| c.find_mut(id))
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }
}

impl TimingTowerRow {
    pub fn tree_view(&self, ui: &mut TreeUi) {
        TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label("Timing Tower");
            },
            |ui| {
                for c in self.columns.iter() {
                    c.tree_view(ui);
                }
            },
        );
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerColumn {
    pub id: Uuid,
    pub cell: Cell,
    pub name: String,
}

impl TreeNode for TimingTowerColumn {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        (&self.id == id).then_some(self as &mut dyn TreeNode)
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        self.cell.property_editor(ui, vars);
    }
}

impl TimingTowerColumn {
    pub fn tree_view(&self, ui: &mut TreeUi) {
        TreeViewBuilder::leaf(self.id).show(ui, |ui| {
            ui.label(&self.name);
        });
    }
}
