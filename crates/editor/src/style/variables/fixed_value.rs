use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    value_store::{TypedValueProducer, ValueProducer, ValueStore},
    value_types::{Boolean, Number, Text, Tint, ValueType},
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FixedValue {
    value: FixedValueType,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FixedValueType {
    Number(Number),
    Text(Text),
    Color(Tint),
    Boolean(Boolean),
}
impl Default for FixedValueType {
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
                .selected_text(match self.value {
                    FixedValueType::Number(_) => "Number",
                    FixedValueType::Text(_) => "Text",
                    FixedValueType::Color(_) => "Color",
                    FixedValueType::Boolean(_) => "Yes/No",
                })
                .show_ui(ui, |ui| {
                    let is_number = matches!(self.value, FixedValueType::Number(_));
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.value = FixedValueType::Number(Number::default());
                        changed |= true;
                    }
                    let is_text = matches!(self.value, FixedValueType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.value = FixedValueType::Text(Text::default());
                        changed |= true;
                    }
                    let is_color = matches!(self.value, FixedValueType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.value = FixedValueType::Color(Tint::default());
                        changed |= true;
                    }
                    let is_boolean = matches!(self.value, FixedValueType::Boolean(_));
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.value = FixedValueType::Boolean(Boolean::default());
                        changed |= true;
                    }
                });
        });

        match &mut self.value {
            FixedValueType::Number(Number(number)) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    changed |= ui.add(DragValue::new(number)).changed();
                });
            }
            FixedValueType::Text(Text(text)) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    changed |= ui.text_edit_singleline(text).changed();
                });
            }
            FixedValueType::Color(Tint(tint)) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color_local = tint.as_rgba_f32();
                    changed |= ui
                        .color_edit_button_rgba_unmultiplied(&mut color_local)
                        .changed();
                    *tint = color_local.into();
                });
            }
            FixedValueType::Boolean(Boolean(boolean)) => {
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
        match self.value {
            FixedValueType::Number(_) => ValueType::Number,
            FixedValueType::Text(_) => ValueType::Text,
            FixedValueType::Color(_) => ValueType::Tint,
            FixedValueType::Boolean(_) => ValueType::Boolean,
        }
    }

    pub fn as_typed_producer(&self) -> TypedValueProducer {
        match self.value.clone() {
            FixedValueType::Number(n) => {
                TypedValueProducer::Number(Box::new(StaticValueProducer(n)))
            }
            FixedValueType::Text(t) => TypedValueProducer::Text(Box::new(StaticValueProducer(t))),
            FixedValueType::Color(c) => TypedValueProducer::Tint(Box::new(StaticValueProducer(c))),
            FixedValueType::Boolean(b) => {
                TypedValueProducer::Boolean(Box::new(StaticValueProducer(b)))
            }
        }
    }
}

struct StaticValueProducer<T>(T);
impl<T> ValueProducer<T> for StaticValueProducer<T>
where
    T: Clone,
{
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<T> {
        Some(self.0.clone())
    }
}
