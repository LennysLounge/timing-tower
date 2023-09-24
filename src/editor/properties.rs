use bevy::prelude::Color;
use bevy_egui::egui::{self, DragValue, TextEdit, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::variable_element::VariableType;

#[derive(Serialize, Deserialize, Clone)]
pub enum NumberProperty {
    Ref(Uuid),
    #[serde(untagged)]
    Fixed(f32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TextProperty {
    Ref(Uuid),
    #[serde(untagged)]
    Fixed(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ColorProperty {
    Ref(Uuid),
    #[serde(untagged)]
    Fixed(Color),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec2Property {
    pub x: NumberProperty,
    pub y: NumberProperty,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec3Property {
    pub x: NumberProperty,
    pub y: NumberProperty,
    pub z: NumberProperty,
}

impl TextProperty {
    pub fn editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        match self {
            TextProperty::Fixed(t) => {
                ui.add(TextEdit::singleline(t).desired_width(100.0));
                let popup_button = ui.button("R");
                let popup_id = ui.next_auto_id();
                if popup_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }
                egui::popup::popup_below_widget(ui, popup_id, &popup_button, |ui| {
                    ui.set_min_width(200.0);
                    let color_vars = vars.vars.values().map(|v| &v.def).filter(|var| {
                        matches!(
                            var.var_type,
                            VariableType::StaticNumber(_) | VariableType::StaticText(_)
                        )
                    });
                    for var in color_vars {
                        if ui.selectable_label(false, &var.name).clicked() {
                            *self = TextProperty::Ref(var.id.clone());
                            ui.memory_mut(|mem| mem.close_popup());
                        }
                    }
                });
            }
            TextProperty::Ref(var_ref) => {
                if let Some(var) = vars.get_var_def(&var_ref) {
                    ui.label(format!("[ {} ]", var.name));
                } else {
                    ui.label(format!("Invalid variable reference"));
                }
                if ui.button("x").clicked() {
                    *self = TextProperty::Fixed("".to_string());
                }
            }
        }
    }
}

impl NumberProperty {
    pub fn editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        match self {
            NumberProperty::Fixed(c) => {
                ui.add(DragValue::new(c));
                let popup_button = ui.button("R");
                let popup_id = ui.next_auto_id();
                if popup_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }
                egui::popup::popup_below_widget(ui, popup_id, &popup_button, |ui| {
                    ui.set_min_width(200.0);
                    let color_vars = vars
                        .vars
                        .values()
                        .map(|v| &v.def)
                        .filter(|var| matches!(var.var_type, VariableType::StaticNumber(_)));
                    for var in color_vars {
                        if ui.selectable_label(false, &var.name).clicked() {
                            *self = NumberProperty::Ref(var.id.clone());
                            ui.memory_mut(|mem| mem.close_popup());
                        }
                    }
                });
            }
            NumberProperty::Ref(var_ref) => {
                if let Some(var) = vars.get_var_def(&var_ref) {
                    ui.label(format!("[ {} ]", var.name));
                } else {
                    ui.label(format!("Invalid variable reference"));
                }
                if ui.button("x").clicked() {
                    *self = NumberProperty::Fixed(0.0);
                }
            }
        }
    }
}

impl ColorProperty {
    pub fn editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        match self {
            ColorProperty::Fixed(c) => {
                let mut color = c.as_rgba_f32();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                *c = color.into();

                let popup_button = ui.button("R");
                //let popup_id = ui.make_persistent_id("color_property_popup");
                let popup_id = ui.next_auto_id();
                if popup_button.clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                }
                egui::popup::popup_below_widget(ui, popup_id, &popup_button, |ui| {
                    ui.set_min_width(200.0);
                    let color_vars = vars
                        .vars
                        .values()
                        .map(|v| &v.def)
                        .filter(|var| matches!(var.var_type, VariableType::StaticColor(_)));
                    for var in color_vars {
                        if ui.selectable_label(false, &var.name).clicked() {
                            *self = ColorProperty::Ref(var.id.clone());
                            ui.memory_mut(|mem| mem.close_popup());
                        }
                    }
                });
            }
            ColorProperty::Ref(var_ref) => {
                if let Some(var) = vars.get_var_def(&var_ref) {
                    ui.label(format!("[ {} ]", var.name));
                } else {
                    ui.label(format!("Invalid variable reference"));
                }
                if ui.button("x").clicked() {
                    *self = ColorProperty::Fixed(Color::PURPLE);
                }
            }
        }
    }
}
