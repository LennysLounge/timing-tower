use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    asset_repo::{AssetReference, AssetType},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum NumberProperty {
    Ref(AssetReference),
    #[serde(untagged)]
    Fixed(f32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TextProperty {
    Ref(AssetReference),
    #[serde(untagged)]
    Fixed(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ColorProperty {
    Ref(AssetReference),
    #[serde(untagged)]
    Fixed(Color),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BooleanProperty {
    Ref(AssetReference),
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
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        match self {
            TextProperty::Fixed(t) => {
                ui.add(TextEdit::singleline(t).desired_width(100.0));
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Text))
                {
                    *self = TextProperty::Ref(reference);
                }
            }
            TextProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, asset_ref, |v| {
                    v.asset_type.can_cast_to(&AssetType::Text)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                }
                if ui.button("x").clicked() {
                    *self = TextProperty::Fixed("".to_string());
                }
            }
        }
    }
}

impl NumberProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        match self {
            NumberProperty::Fixed(c) => {
                ui.add(DragValue::new(c));
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Number))
                {
                    *self = NumberProperty::Ref(reference);
                }
            }
            NumberProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, asset_ref, |v| {
                    v.asset_type.can_cast_to(&AssetType::Number)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                }
                if ui.button("x").clicked() {
                    *self = NumberProperty::Fixed(0.0);
                }
            }
        }
    }
}

impl ColorProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        match self {
            ColorProperty::Fixed(c) => {
                let mut color = c.as_rgba_f32();
                ui.color_edit_button_rgba_unmultiplied(&mut color);
                *c = color.into();

                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Color))
                {
                    *self = ColorProperty::Ref(reference);
                }
            }
            ColorProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, asset_ref, |v| {
                    v.asset_type.can_cast_to(&AssetType::Color)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                }
                if ui.button("x").clicked() {
                    *self = ColorProperty::Fixed(Color::PURPLE);
                }
            }
        }
    }
}

impl BooleanProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
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
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Boolean))
                {
                    *self = BooleanProperty::Ref(reference);
                }
            }
            BooleanProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, asset_ref, |v| {
                    v.asset_type.can_cast_to(&AssetType::Color)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                }
                if ui.button("x").clicked() {
                    *self = BooleanProperty::Fixed(true);
                }
            }
        }
    }
}
