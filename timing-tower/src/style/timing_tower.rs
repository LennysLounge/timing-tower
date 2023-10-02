use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{TreeNode, TreeNodeConverstions};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::{cell::Cell, properties::Vec2Property};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}

impl TreeNode for TimingTower {
    fn is_directory(&self) -> bool {
        true
    }

    fn show_label(&self, ui: &mut bevy_egui::egui::Ui) {
        ui.label("Timing Tower");
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        vec![self.table.as_dyn()]
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        vec![self.table.as_dyn_mut()]
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}

impl TimingTower {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
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
    fn is_directory(&self) -> bool {
        true
    }

    fn show_label(&self, ui: &mut Ui) {
        ui.label("Table");
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        vec![self.row.as_dyn()]
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        vec![self.row.as_dyn_mut()]
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}
impl TimingTowerTable {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub columns: Vec<TimingTowerColumn>,
}

impl TreeNode for TimingTowerRow {
    fn is_directory(&self) -> bool {
        true
    }

    fn show_label(&self, ui: &mut Ui) {
        ui.label("Row");
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        self.columns.iter().map(|c| c.as_dyn()).collect()
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        self.columns.iter_mut().map(|c| c.as_dyn_mut()).collect()
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}

impl TimingTowerRow {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerColumn {
    pub id: Uuid,
    pub cell: Cell,
    pub name: String,
}

impl TreeNode for TimingTowerColumn {
    fn is_directory(&self) -> bool {
        false
    }

    fn show_label(&self, ui: &mut Ui) {
        ui.label(&self.name);
    }

    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        Vec::new()
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        Vec::new()
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}
impl TimingTowerColumn {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        self.cell.property_editor(ui, vars);
    }
}
