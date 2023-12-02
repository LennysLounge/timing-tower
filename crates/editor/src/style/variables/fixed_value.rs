use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    value_store::TypedValueProducer,
    value_types::{Boolean, Number, Text, Tint, ValueType},
};

use super::StaticValueProducer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
pub enum FixedValue {
    Number(Number),
    Text(Text),
    Tint(Tint),
    Boolean(Boolean),
}
impl Default for FixedValue {
    fn default() -> Self {
        Self::Number(Number::default())
    }
}

impl FixedValue {
    pub fn property_editor(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self {
                    FixedValue::Number(_) => "Number",
                    FixedValue::Text(_) => "Text",
                    FixedValue::Tint(_) => "Color",
                    FixedValue::Boolean(_) => "Yes/No",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(self, FixedValue::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        *self = FixedValue::Number(Number::default());
                        changed |= true;
                    }
                    let is_text = matches!(self, FixedValue::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        *self = FixedValue::Text(Text::default());
                        changed |= true;
                    }
                    let is_color = matches!(self, FixedValue::Tint(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        *self = FixedValue::Tint(Tint::default());
                        changed |= true;
                    }
                    let is_boolean = matches!(self, FixedValue::Boolean(_));
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        *self = FixedValue::Boolean(Boolean::default());
                        changed |= true;
                    }
                });
        });

        match self {
            FixedValue::Number(Number(number)) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    changed |= ui.add(DragValue::new(number)).changed();
                });
            }
            FixedValue::Text(Text(text)) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    changed |= ui.text_edit_singleline(text).changed();
                });
            }
            FixedValue::Tint(Tint(tint)) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color_local = tint.as_rgba_f32();
                    changed |= ui
                        .color_edit_button_rgba_unmultiplied(&mut color_local)
                        .changed();
                    *tint = color_local.into();
                });
            }
            FixedValue::Boolean(Boolean(boolean)) => {
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .selected_text(match boolean {
                            true => "Yes",
                            false => "No",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(boolean, true, "Yes").changed();
                            changed |= ui.selectable_value(boolean, false, "No").changed();
                        });
                });
            }
        }
        changed
    }
}

impl FixedValue {
    pub fn output_type(&self) -> ValueType {
        match self {
            FixedValue::Number(_) => ValueType::Number,
            FixedValue::Text(_) => ValueType::Text,
            FixedValue::Tint(_) => ValueType::Tint,
            FixedValue::Boolean(_) => ValueType::Boolean,
        }
    }

    pub fn as_typed_producer(&self) -> TypedValueProducer {
        match self.clone() {
            FixedValue::Number(n) => TypedValueProducer::Number(Box::new(StaticValueProducer(n))),
            FixedValue::Text(t) => TypedValueProducer::Text(Box::new(StaticValueProducer(t))),
            FixedValue::Tint(c) => TypedValueProducer::Tint(Box::new(StaticValueProducer(c))),
            FixedValue::Boolean(b) => TypedValueProducer::Boolean(Box::new(StaticValueProducer(b))),
        }
    }
}
