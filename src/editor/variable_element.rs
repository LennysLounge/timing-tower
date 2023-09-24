use std::sync::{Arc, Mutex};

use bevy::prelude::Color;
use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, DragValue, Sense, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::style_elements::{ColorProperty, NumberProperty, StyleElement, TextProperty};

#[derive(Serialize, Deserialize, Clone)]
pub struct VariablesElement {
    pub vars: Vec<Variable>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Variable(pub Arc<Mutex<SharedVariable>>);

#[derive(Serialize, Deserialize, Clone)]
pub struct SharedVariable {
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

impl VariablesElement {
    pub fn get_var(&self, id: &Uuid) -> Option<&Variable> {
        self.vars.iter().find_map(|v| {
            let shared = v.0.lock().expect("You done messed up now AAron");
            shared.id.eq(id).then_some(v)
        })
    }

    pub fn get_number(&self, property: &NumberProperty) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(id) => self.get_var(&id.id).and_then(|s| {
                let shared = s.0.lock().expect("Still fucked jo");
                match &shared.var_type {
                    VariableType::Number(n) => Some(*n),
                    VariableType::Text(_) => None,
                    VariableType::Color(_) => None,
                }
            }),
        }
    }

    pub fn get_text(&self, property: &TextProperty) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(id) => self.get_var(&id.id).and_then(|s| {
                let shared = s.0.lock().expect("Still fucked jo");
                match &shared.var_type {
                    VariableType::Number(n) => Some(format!("{n}")),
                    VariableType::Text(s) => Some(s.clone()),
                    VariableType::Color(_) => None,
                }
            }),
        }
    }

    pub fn get_color(&self, property: &ColorProperty) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(id) => self.get_var(&id.id).and_then(|s| {
                let shared = s.0.lock().expect("Still fucked jo");
                match &shared.var_type {
                    VariableType::Number(_) => None,
                    VariableType::Text(_) => None,
                    VariableType::Color(c) => Some(c.clone()),
                }
            }),
        }
    }
}

impl StyleElement for VariablesElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let id = ui.next_auto_id();
        let (toggle, header_res, _) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| ui.label("Variables"))
            .body(|ui| {
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

    fn property_editor(&mut self, _ui: &mut Ui, _vars: &VariablesElement) {}
}

impl StyleElement for Variable {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let shared = self.0.lock().expect("Still fucked jo");
        let is_selected = selected_element.is_some_and(|uuid| uuid.eq(&shared.id));

        if ui.selectable_label(is_selected, &shared.name).clicked() {
            *selected_element = Some(shared.id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        let is_selected = {
            let shared = self.0.lock().expect("Still fucked jo");
            shared.id.eq(id)
        };

        if is_selected {
            Some(self as &mut dyn StyleElement)
        } else {
            None
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, _vars: &VariablesElement) {
        let mut shared = self.0.lock().expect("Still fucked jo");
        ui.label("Name:");
        ui.text_edit_singleline(&mut shared.name);
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match shared.var_type {
                    VariableType::Number(_) => "Number",
                    VariableType::Text(_) => "Text",
                    VariableType::Color(_) => "Color",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(shared.var_type, VariableType::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() {
                        shared.var_type = VariableType::Number(0.0);
                    }

                    let is_text = matches!(shared.var_type, VariableType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() {
                        shared.var_type = VariableType::Text("".to_string());
                    }

                    let is_color = matches!(shared.var_type, VariableType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() {
                        shared.var_type = VariableType::Color(Color::RED);
                    }
                });
        });
        match &mut shared.var_type {
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
