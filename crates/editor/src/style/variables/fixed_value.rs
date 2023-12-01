use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    reference_store::AssetId,
    value_store::{IntoValueProducer, TypedValueProducer, ValueProducer, ValueStore},
    value_types::{Boolean, Number, Text, Tint, ValueType},
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FixedValue {
    #[serde(flatten)]
    id: AssetId,
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
    pub fn from_id(id: AssetId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

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
                        self.value = FixedValueType::Number(0.0);
                        self.id.asset_type = ValueType::Number;
                        changed |= true;
                    }
                    let is_text = matches!(self.value, FixedValueType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.value = FixedValueType::Text(String::new());
                        self.id.asset_type = ValueType::Text;
                        changed |= true;
                    }
                    let is_color = matches!(self.value, FixedValueType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.value = FixedValueType::Color(Color::WHITE);
                        self.id.asset_type = ValueType::Tint;
                        changed |= true;
                    }
                    let is_boolean = matches!(self.value, FixedValueType::Boolean(_));
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.value = FixedValueType::Boolean(true);
                        self.id.asset_type = ValueType::Boolean;
                        changed |= true;
                    }
                });
        });

        match &mut self.value {
            FixedValueType::Number(number) => {
                ui.horizontal(|ui| {
                    ui.label("Value");
                    changed |= ui.add(DragValue::new(number)).changed();
                });
            }
            FixedValueType::Text(text) => {
                ui.horizontal(|ui| {
                    ui.label("Text");
                    changed |= ui.text_edit_singleline(text).changed();
                });
            }
            FixedValueType::Color(color) => {
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    let mut color_local = color.as_rgba_f32();
                    changed |= ui
                        .color_edit_button_rgba_unmultiplied(&mut color_local)
                        .changed();
                    *color = color_local.into();
                });
            }
            FixedValueType::Boolean(b) => {
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .selected_text(match b {
                            true => "Yes",
                            false => "No",
                        })
                        .show_ui(ui, |ui| {
                            changed |= ui.selectable_value(b, true, "Yes").changed();
                            changed |= ui.selectable_value(b, false, "No").changed();
                        });
                });
            }
        }
        changed
    }
}

impl IntoValueProducer for FixedValue {
    fn asset_id(&self) -> &AssetId {
        &self.id
    }
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer) {
        let producer = match &self.value {
            FixedValueType::Number(n) => TypedValueProducer::Number(Box::new(StaticNumber(*n))),
            FixedValueType::Text(t) => TypedValueProducer::Text(Box::new(StaticText(t.clone()))),
            FixedValueType::Color(c) => TypedValueProducer::Tint(Box::new(StaticColor(*c))),
            FixedValueType::Boolean(b) => TypedValueProducer::Boolean(Box::new(StaticBoolean(*b))),
        };
        (self.id.id, producer)
    }
}

pub struct StaticNumber(pub f32);
impl ValueProducer<Number> for StaticNumber {
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Number> {
        Some(Number(self.0))
    }
}

pub struct StaticText(pub String);
impl ValueProducer<Text> for StaticText {
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Text> {
        Some(Text(self.0.clone()))
    }
}

pub struct StaticColor(pub Color);
impl ValueProducer<Tint> for StaticColor {
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Tint> {
        Some(Tint(self.0))
    }
}

pub struct StaticBoolean(pub bool);
impl ValueProducer<Boolean> for StaticBoolean {
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Boolean> {
        Some(Boolean(self.0))
    }
}
