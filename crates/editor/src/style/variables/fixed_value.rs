use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::value_store::{
    types::{Boolean, Number, Text, Tint},
    AssetId, AssetType, BooleanSource, ColorSource, IntoValueProducer, NumberSource, TextSource,
    ValueProducer, ValueStore,
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
                        self.id.asset_type = AssetType::Number;
                        changed |= true;
                    }
                    let is_text = matches!(self.value, FixedValueType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.value = FixedValueType::Text(String::new());
                        self.id.asset_type = AssetType::Text;
                        changed |= true;
                    }
                    let is_color = matches!(self.value, FixedValueType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.value = FixedValueType::Color(Color::WHITE);
                        self.id.asset_type = AssetType::Color;
                        changed |= true;
                    }
                    let is_boolean = matches!(self.value, FixedValueType::Boolean(_));
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.value = FixedValueType::Boolean(true);
                        self.id.asset_type = AssetType::Boolean;
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
    fn get_value_producer(&self) -> Box<dyn ValueProducer + Send + Sync> {
        match &self.value {
            FixedValueType::Number(n) => Box::new(StaticNumber(*n)),
            FixedValueType::Text(t) => Box::new(StaticText(t.clone())),
            FixedValueType::Color(c) => Box::new(StaticColor(*c)),
            FixedValueType::Boolean(b) => Box::new(StaticBoolean(*b)),
        }
    }
}

pub struct StaticNumber(pub f32);
impl NumberSource for StaticNumber {
    fn resolve(&self, _vars: &ValueStore, _entry: Option<&Entry>) -> Option<f32> {
        Some(self.0)
    }
}
impl ValueProducer for StaticNumber {
    fn get_number(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Number> {
        Some(Number(self.0))
    }
}

pub struct StaticText(pub String);
impl TextSource for StaticText {
    fn resolve(&self, _vars: &ValueStore, _entry: Option<&Entry>) -> Option<String> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticText {
    fn get_text(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Text> {
        Some(Text(self.0.clone()))
    }
}

pub struct StaticColor(pub Color);
impl ColorSource for StaticColor {
    fn resolve(&self, _vars: &ValueStore, _entry: Option<&Entry>) -> Option<Color> {
        Some(self.0)
    }
}
impl ValueProducer for StaticColor {
    fn get_tint(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Tint> {
        Some(Tint(self.0))
    }
}

pub struct StaticBoolean(pub bool);
impl BooleanSource for StaticBoolean {
    fn resolve(&self, _vars: &ValueStore, _entry: Option<&Entry>) -> Option<bool> {
        Some(self.0)
    }
}
impl ValueProducer for StaticBoolean {
    fn get_boolean(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Boolean> {
        Some(Boolean(self.0))
    }
}
