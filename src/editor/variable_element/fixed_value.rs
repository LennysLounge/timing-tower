use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};

use crate::variable_repo::{
    StaticBoolean, StaticColor, StaticNumber, StaticText, ValueType, VariableId, VariableSource,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FixedValue {
    #[serde(flatten)]
    id: VariableId,
    value: FixedValueType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FixedValueType {
    Number(f32),
    Text(String),
    Color(Color),
    Boolean(bool),
}

impl Default for FixedValueType {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

impl FixedValue {
    pub fn from_id(id: VariableId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> &VariableId {
        &self.id
    }
    pub fn get_id_mut(&mut self) -> &mut VariableId {
        &mut self.id
    }

    pub fn property_editor(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.value {
                    FixedValueType::Number(_) => "Number",
                    FixedValueType::Text(_) => "Text",
                    FixedValueType::Color(_) => "Color",
                    FixedValueType::Boolean(_) => "Boolean",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(self.value, FixedValueType::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.value = FixedValueType::Number(0.0);
                        self.id.value_type = ValueType::Number;
                    }
                    let is_text = matches!(self.value, FixedValueType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.value = FixedValueType::Text(String::new());
                        self.id.value_type = ValueType::Text;
                    }
                    let is_color = matches!(self.value, FixedValueType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.value = FixedValueType::Color(Color::WHITE);
                        self.id.value_type = ValueType::Color;
                    }
                    let is_boolean = matches!(self.value, FixedValueType::Boolean(_));
                    if ui.selectable_label(is_boolean, "Boolean").clicked() && !is_boolean {
                        self.value = FixedValueType::Boolean(true);
                        self.id.value_type = ValueType::Boolean;
                    }
                });
        });

        match &mut self.value {
            FixedValueType::Number(number) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    ui.add(DragValue::new(number));
                });
            }
            FixedValueType::Text(text) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    ui.text_edit_singleline(text);
                });
            }
            FixedValueType::Color(color) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color_local = color.as_rgba_f32();
                    ui.color_edit_button_rgba_unmultiplied(&mut color_local);
                    *color = color_local.into();
                });
            }
            FixedValueType::Boolean(b) => {
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.checkbox(b, "");
                });
            }
        }
    }

    pub fn as_variable_source(&self) -> VariableSource {
        match &self.value {
            FixedValueType::Number(n) => VariableSource::Number(Box::new(StaticNumber(*n))),
            FixedValueType::Text(t) => VariableSource::Text(Box::new(StaticText(t.clone()))),
            FixedValueType::Color(c) => VariableSource::Color(Box::new(StaticColor(*c))),
            FixedValueType::Boolean(b) => VariableSource::Bool(Box::new(StaticBoolean(*b))),
        }
    }
}
