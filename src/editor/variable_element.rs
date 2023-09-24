use bevy::prelude::Color;
use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, DragValue, Sense, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::style_elements::StyleElement;

#[derive(Serialize, Deserialize, Clone)]
pub struct VariablesElement {
    pub vars: Vec<Variable>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Variable {
    pub id: Uuid,
    pub name: String,
    pub var_type: VariableType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VariableType {
    Number(f32),
    Text(String),
    Color(Color),
}

impl StyleElement for VariablesElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let id = ui.next_auto_id();
        let (toggle, header_res, _) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| ui.label("Variables"))
            .body(|ui| {
                if ui.button("+ Add variable").clicked() {
                    let uuid = Uuid::new_v4();
                    self.vars.push(Variable {
                        id: uuid.clone(),
                        name: "Variable".to_string(),
                        var_type: VariableType::Number(12.0),
                    });
                    *selected_element = Some(uuid);
                }
                for var in self.vars.iter_mut() {
                    var.element_tree(ui, selected_element);
                }
            });
        if ui
            .interact(header_res.response.rect, id, Sense::click())
            .clicked()
            && !toggle.clicked()
        {
            if let Some(mut state) = CollapsingState::load(ui.ctx(), id) {
                state.toggle(ui);
                state.store(ui.ctx());
            }
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.vars.iter_mut().find_map(|v| v.find_mut(id))
    }

    fn property_editor(&mut self, _ui: &mut Ui, _vars: &VariableRepo) {}
}

impl StyleElement for Variable {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid == self.id);
        if ui.selectable_label(is_selected, &self.name).clicked() {
            *selected_element = Some(self.id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        (&self.id == id).then_some(self as &mut dyn StyleElement)
    }

    fn property_editor(&mut self, ui: &mut Ui, _vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.var_type {
                    VariableType::Number(_) => "Number",
                    VariableType::Text(_) => "Text",
                    VariableType::Color(_) => "Color",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(self.var_type, VariableType::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() {
                        self.var_type = VariableType::Number(0.0);
                    }

                    let is_text = matches!(self.var_type, VariableType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() {
                        self.var_type = VariableType::Text("".to_string());
                    }

                    let is_color = matches!(self.var_type, VariableType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() {
                        self.var_type = VariableType::Color(Color::RED);
                    }
                });
        });
        match &mut self.var_type {
            VariableType::Number(n) => {
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add(DragValue::new(n));
                });
            }
            VariableType::Text(t) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    ui.text_edit_singleline(t);
                });
            }
            VariableType::Color(c) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color = c.as_rgba_f32();
                    ui.color_edit_button_rgba_unmultiplied(&mut color);
                    *c = color.into();
                });
            }
        }
    }
}
