use bevy::prelude::Color;
use bevy_egui::egui::{DragValue, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::variable_repo::{Reference, ValueType, VariableRepo};

use super::style_elements::{reference_editor, reference_editor_small};

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
