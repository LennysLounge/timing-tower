use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, DragValue, TextEdit, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    value_store::{types::Text, AssetReference, AssetType, Property, ValueRef},
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
pub enum ImageProperty {
    Ref(AssetReference),
    #[serde(untagged)]
    None,
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
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;
        match self {
            TextProperty::Fixed(t) => {
                changed |= ui
                    .add(TextEdit::singleline(t).desired_width(100.0))
                    .changed();
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Text))
                {
                    *self = TextProperty::Ref(reference);
                    changed |= true;
                }
            }
            TextProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, &asset_ref.key, |v| {
                    v.asset_type.can_cast_to(&AssetType::Text)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = TextProperty::Fixed("".to_string());
                    changed |= true;
                }
            }
        }
        changed
    }
}

pub fn text_property_editor(
    ui: &mut Ui,
    property: &mut Property<Text>,
    asset_repo: &AssetReferenceRepo,
) -> bool {
    let mut changed = false;
    match property {
        Property::Fixed(text) => {
            changed |= ui
                .add(TextEdit::singleline(&mut text.0).desired_width(100.0))
                .changed();
            if let Some(reference) =
                asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Text))
            {
                *property = Property::ValueRef(ValueRef::<Text> {
                    id: reference.key,
                    phantom: std::marker::PhantomData,
                });
                changed |= true;
            }
        }
        Property::ValueRef(value_ref) => {
            let new_ref = asset_repo.editor(ui, &value_ref.id, |v| {
                v.asset_type.can_cast_to(&AssetType::Text)
            });
            if let Some(new_ref) = new_ref {
                value_ref.id = new_ref.key;
                changed |= true;
            }
            if ui.button("x").clicked() {
                *property = Property::Fixed(Text("".to_string()));
                changed |= true;
            }
        }
    }
    changed
}

impl NumberProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;
        match self {
            NumberProperty::Fixed(c) => {
                changed |= ui.add(DragValue::new(c)).changed();
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Number))
                {
                    *self = NumberProperty::Ref(reference);
                    changed = true;
                }
            }
            NumberProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, &asset_ref.key, |v| {
                    v.asset_type.can_cast_to(&AssetType::Number)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                    changed = true;
                }
                if ui.button("x").clicked() {
                    *self = NumberProperty::Fixed(0.0);
                    changed = true;
                }
            }
        }
        changed
    }
}

impl ColorProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;
        match self {
            ColorProperty::Fixed(c) => {
                let mut color = c.as_rgba_f32();
                changed |= ui.color_edit_button_rgba_unmultiplied(&mut color).changed();
                *c = color.into();

                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Color))
                {
                    *self = ColorProperty::Ref(reference);
                    changed |= true;
                }
            }
            ColorProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, &asset_ref.key, |v| {
                    v.asset_type.can_cast_to(&AssetType::Color)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = ColorProperty::Fixed(Color::PURPLE);
                    changed |= true;
                }
            }
        }
        changed
    }
}

impl BooleanProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;
        match self {
            BooleanProperty::Fixed(b) => {
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
                if let Some(reference) =
                    asset_repo.editor_small(ui, |v| v.asset_type.can_cast_to(&AssetType::Boolean))
                {
                    *self = BooleanProperty::Ref(reference);
                    changed |= true;
                }
            }
            BooleanProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, &asset_ref.key, |v| {
                    v.asset_type.can_cast_to(&AssetType::Color)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = BooleanProperty::Fixed(true);
                    changed |= true;
                }
            }
        }
        changed
    }
}

impl Default for ImageProperty {
    fn default() -> Self {
        Self::None
    }
}

impl ImageProperty {
    pub fn editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;
        match self {
            ImageProperty::None => {
                if let Some(reference) =
                    asset_repo.editor_none(ui, |v| v.asset_type.can_cast_to(&AssetType::Image))
                {
                    *self = ImageProperty::Ref(reference);
                    changed |= true;
                }
            }
            ImageProperty::Ref(asset_ref) => {
                let new_ref = asset_repo.editor(ui, &asset_ref.key, |v| {
                    v.asset_type.can_cast_to(&AssetType::Image)
                });
                if let Some(new_ref) = new_ref {
                    *asset_ref = new_ref;
                    changed |= true;
                }
                if ui.button("x").clicked() {
                    *self = ImageProperty::None;
                    changed |= true;
                }
            }
        }
        changed
    }
}
