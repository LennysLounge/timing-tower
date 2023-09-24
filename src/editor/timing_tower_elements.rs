use bevy::prelude::Vec2;
use bevy_egui::egui::{collapsing_header::CollapsingState, DragValue, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::style_elements::{CellElement, StyleElement};

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerElement {
    pub id: Uuid,
    pub cell: CellElement,
    pub table: TimingTowerTableElement,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerTableElement {
    pub id: Uuid,
    pub cell: CellElement,
    pub row_offset: Vec2,
    pub row: TimingTowerRowElement,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerRowElement {
    pub id: Uuid,
    pub cell: CellElement,
    pub columns: Vec<TimingTowerColumnElement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimingTowerColumnElement {
    pub id: Uuid,
    pub cell: CellElement,
    pub name: String,
}

impl StyleElement for TimingTowerElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Timing Tower").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                self.table.element_tree(ui, selected_element);
            });
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.table.find_mut(id)
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }
}

impl StyleElement for TimingTowerTableElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Table").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                self.row.element_tree(ui, selected_element);
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.row.find_mut(id)
    }
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            ui.add(DragValue::new(&mut self.row_offset.x));
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            ui.add(DragValue::new(&mut self.row_offset.y));
        });
        ui.separator();
        self.cell.property_editor(ui, vars);
    }
}

impl StyleElement for TimingTowerRowElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
            .show_header(ui, |ui| {
                let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
                if ui.selectable_label(is_selected, "Row").clicked() {
                    *selected_element = Some(self.id.clone());
                }
            })
            .body(|ui| {
                if ui.button("+ Add cell").clicked() {
                    let column = TimingTowerColumnElement {
                        id: Uuid::new_v4(),
                        cell: CellElement::default(),
                        name: "Column".to_string(),
                    };
                    self.columns.push(column);
                }
                for column in self.columns.iter_mut() {
                    column.element_tree(ui, selected_element);
                }
            });
    }
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        if self.id.eq(id) {
            return Some(self as &mut dyn StyleElement);
        }
        self.columns
            .iter_mut()
            .find_map(|element| element.find_mut(id))
    }
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        self.cell.property_editor(ui, vars);
    }
}

impl StyleElement for TimingTowerColumnElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&self.id));
        if ui
            .selectable_label(is_selected, self.name.clone())
            .clicked()
        {
            *selected_element = Some(self.id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.id.eq(id).then_some(self as &mut dyn StyleElement)
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        self.cell.property_editor(ui, vars);
    }
}
