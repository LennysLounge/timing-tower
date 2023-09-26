use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};

use crate::variable_repo::{StaticColor, StaticNumber, StaticText, VariableSource};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FixedValue {
    Number(f32),
    Text(String),
    Color(Color),
}

impl Default for FixedValue {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

impl FixedValue {
    pub fn property_editor(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self {
                    FixedValue::Number(_) => "Number",
                    FixedValue::Text(_) => "Text",
                    FixedValue::Color(_) => "Color",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(self, Self::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        *self = Self::Number(0.0)
                    }
                    let is_text = matches!(self, Self::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        *self = Self::Text(String::new())
                    }
                    let is_color = matches!(self, Self::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        *self = Self::Color(Color::WHITE)
                    }
                });
        });

        match self {
            Self::Number(number) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    ui.add(DragValue::new(number));
                });
            }
            Self::Text(text) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    ui.text_edit_singleline(text);
                });
            }
            Self::Color(color) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color_local = color.as_rgba_f32();
                    ui.color_edit_button_rgba_unmultiplied(&mut color_local);
                    *color = color_local.into();
                });
            }
        }
    }

    pub fn as_variable_source(&self) -> VariableSource {
        match self {
            Self::Number(n) => VariableSource::Number(Box::new(StaticNumber(*n))),
            Self::Text(t) => VariableSource::Text(Box::new(StaticText(t.clone()))),
            Self::Color(c) => VariableSource::Color(Box::new(StaticColor(*c))),
        }
    }
}
