use bevy::prelude::Color;
use bevy_egui::egui::{DragValue, Ui};
use serde::{Deserialize, Serialize};

use crate::variable_repo::{StaticColor, StaticNumber, StaticText, VariableSource};

use super::VariableOutputType;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FixedValue {
    StaticNumber(f32),
    StaticText(String),
    StaticColor(Color),
}

impl Default for FixedValue {
    fn default() -> Self {
        Self::StaticNumber(0.0)
    }
}

impl FixedValue {
    pub fn property_editor(&mut self, ui: &mut Ui, output_type: &VariableOutputType) {
        match (output_type, &self) {
            (VariableOutputType::Number, Self::StaticNumber(_)) => (),
            (VariableOutputType::Number, _) => *self = Self::StaticNumber(0.0),
            (VariableOutputType::Text, Self::StaticText(_)) => (),
            (VariableOutputType::Text, _) => *self = Self::StaticText(String::new()),
            (VariableOutputType::Color, Self::StaticColor(_)) => (),
            (VariableOutputType::Color, _) => *self = Self::StaticColor(Color::WHITE),
        }

        match self {
            Self::StaticNumber(number) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    ui.add(DragValue::new(number));
                });
            }
            Self::StaticText(text) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    ui.text_edit_singleline(text);
                });
            }
            Self::StaticColor(color) => {
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
            Self::StaticNumber(n) => VariableSource::Number(Box::new(StaticNumber(*n))),
            Self::StaticText(t) => VariableSource::Text(Box::new(StaticText(t.clone()))),
            Self::StaticColor(c) => VariableSource::Color(Box::new(StaticColor(*c))),
        }
    }
}
