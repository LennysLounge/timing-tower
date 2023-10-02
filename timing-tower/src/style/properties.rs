use bevy::prelude::Color;
use bevy_egui::egui::{self, ComboBox, DragValue, Response, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::variable_repo::{Reference, ValueType, Variable, VariableId, VariableRepo};

#[derive(Serialize, Deserialize, Clone)]
pub enum NumberProperty {
    Ref(Reference),
    #[serde(untagged)]
    Fixed(f32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TextProperty {
    Ref(Reference),
    #[serde(untagged)]
    Fixed(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ColorProperty {
    Ref(Reference),
    #[serde(untagged)]
    Fixed(Color),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BooleanProperty {
    Ref(Reference),
    #[serde(untagged)]
    Fixed(bool),
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
                let new_reference = reference_editor_small(ui, vars, |v| {
                    v.value_type.can_cast_to(&ValueType::Text)
                });
                if let Some(reference) = new_reference {
                    *self = TextProperty::Ref(reference);
                }
            }
            TextProperty::Ref(var_ref) => {
                let new_ref = reference_editor(ui, vars, var_ref, |v| {
                    v.value_type.can_cast_to(&ValueType::Text)
                });
                if let Some(reference) = new_ref {
                    *var_ref = reference;
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
                let new_reference = reference_editor_small(ui, vars, |v| {
                    v.value_type.can_cast_to(&ValueType::Number)
                });
                if let Some(reference) = new_reference {
                    *self = NumberProperty::Ref(reference);
                }
            }
            NumberProperty::Ref(var_ref) => {
                let new_ref = reference_editor(ui, vars, var_ref, |v| {
                    v.value_type.can_cast_to(&ValueType::Number)
                });
                if let Some(reference) = new_ref {
                    *var_ref = reference;
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

                let new_reference = reference_editor_small(ui, vars, |v| {
                    v.value_type.can_cast_to(&ValueType::Color)
                });
                if let Some(reference) = new_reference {
                    *self = ColorProperty::Ref(reference);
                }
            }
            ColorProperty::Ref(var_ref) => {
                let new_ref = reference_editor(ui, vars, var_ref, |v| {
                    v.value_type.can_cast_to(&ValueType::Color)
                });
                if let Some(reference) = new_ref {
                    *var_ref = reference;
                }
                if ui.button("x").clicked() {
                    *self = ColorProperty::Fixed(Color::PURPLE);
                }
            }
        }
    }
}

impl BooleanProperty {
    pub fn editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        match self {
            BooleanProperty::Fixed(b) => {
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .selected_text(match b {
                        true => "Yes",
                        false => "No",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(b, true, "Yes");
                        ui.selectable_value(b, false, "No");
                    });
                let new_reference = reference_editor_small(ui, vars, |v| {
                    v.value_type.can_cast_to(&ValueType::Boolean)
                });
                if let Some(reference) = new_reference {
                    *self = BooleanProperty::Ref(reference);
                }
            }
            BooleanProperty::Ref(var_ref) => {
                let new_ref = reference_editor(ui, vars, var_ref, |v| {
                    v.value_type.can_cast_to(&ValueType::Color)
                });
                if let Some(reference) = new_ref {
                    *var_ref = reference;
                }
                if ui.button("x").clicked() {
                    *self = BooleanProperty::Fixed(true);
                }
            }
        }
    }
}

pub fn reference_editor(
    ui: &mut Ui,
    vars: &VariableRepo,
    reference: &mut Reference,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let var_name = match vars.get_var_def(reference) {
        Some(def) => def.name.as_str(),
        None => "Ref",
    };
    let popup_button = ui.button(var_name);
    reference_popup(ui, &popup_button, vars, use_type)
}

pub fn reference_editor_small(
    ui: &mut Ui,
    vars: &VariableRepo,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let popup_button = ui.button("R");
    reference_popup(ui, &popup_button, vars, use_type)
}

fn reference_popup(
    ui: &mut Ui,
    button_response: &Response,
    vars: &VariableRepo,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let popup_id = ui.next_auto_id();
    if button_response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    egui::popup::popup_below_widget(ui, popup_id, &button_response, |ui| {
        ui.set_min_width(200.0);
        let mut color_vars: Vec<&Variable> =
            vars.vars.values().filter(|var| use_type(&var.id)).collect();
        color_vars.sort_by(|v1, v2| v1.id.name.cmp(&v2.id.name));

        let mut result = None;
        for var in color_vars {
            if ui.selectable_label(false, &var.id.name).clicked() && result.is_none() {
                result = Some(var.get_ref());
                ui.memory_mut(|mem| mem.close_popup());
            }
        }
        result
    })
    .flatten()
}
