use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::asset_repo::{
    IntoAssetSource, AssetId, AssetRepo, AssetSource, AssetType, BooleanSource, ColorSource,
    NumberSource, TextSource,
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

    pub fn property_editor(&mut self, ui: &mut Ui) {
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
                    }
                    let is_text = matches!(self.value, FixedValueType::Text(_));
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.value = FixedValueType::Text(String::new());
                        self.id.asset_type = AssetType::Text;
                    }
                    let is_color = matches!(self.value, FixedValueType::Color(_));
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.value = FixedValueType::Color(Color::WHITE);
                        self.id.asset_type = AssetType::Color;
                    }
                    let is_boolean = matches!(self.value, FixedValueType::Boolean(_));
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.value = FixedValueType::Boolean(true);
                        self.id.asset_type = AssetType::Boolean;
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
                });
            }
        }
    }
}

impl IntoAssetSource for FixedValue {
    fn asset_id(&self) -> &AssetId {
        &self.id
    }
    fn get_asset_source(&self) -> AssetSource {
        match &self.value {
            FixedValueType::Number(n) => AssetSource::Number(Box::new(StaticNumber(*n))),
            FixedValueType::Text(t) => AssetSource::Text(Box::new(StaticText(t.clone()))),
            FixedValueType::Color(c) => AssetSource::Color(Box::new(StaticColor(*c))),
            FixedValueType::Boolean(b) => AssetSource::Bool(Box::new(StaticBoolean(*b))),
        }
    }
}

pub struct StaticNumber(pub f32);
impl NumberSource for StaticNumber {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<f32> {
        Some(self.0)
    }
}

pub struct StaticText(pub String);
impl TextSource for StaticText {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<String> {
        Some(self.0.clone())
    }
}

pub struct StaticColor(pub Color);
impl ColorSource for StaticColor {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<Color> {
        Some(self.0)
    }
}
pub struct StaticBoolean(pub bool);
impl BooleanSource for StaticBoolean {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<bool> {
        Some(self.0)
    }
}
