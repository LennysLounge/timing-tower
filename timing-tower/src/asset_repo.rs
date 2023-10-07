use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    game_sources,
    style::properties::{BooleanProperty, ColorProperty, NumberProperty, TextProperty},
};

pub trait NumberSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<Color>;
}

pub trait BooleanSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool>;
}

pub trait AssetDefinition {
    fn as_asset_source(&self) -> AssetSource;
    fn asset_id(&self) -> &AssetId;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default)]
pub enum AssetType {
    #[default]
    Number,
    Text,
    Color,
    Boolean,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AssetReference {
    pub asset_type: AssetType,
    pub key: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetId {
    pub id: Uuid,
    pub name: String,
    pub asset_type: AssetType,
}

impl Default for AssetId {
    fn default() -> Self {
        Self {
            name: "Variable".to_string(),
            id: Uuid::new_v4(),
            asset_type: AssetType::default(),
        }
    }
}
impl AssetId {
    pub fn get_ref(&self) -> AssetReference {
        AssetReference {
            asset_type: self.asset_type.clone(),
            key: self.id.clone(),
        }
    }
}

pub struct Asset {
    pub id: AssetId,
    pub source: AssetSource,
}

pub enum AssetSource {
    Number(Box<dyn NumberSource + Send + Sync>),
    Text(Box<dyn TextSource + Send + Sync>),
    Color(Box<dyn ColorSource + Send + Sync>),
    Bool(Box<dyn BooleanSource + Send + Sync>),
}

#[derive(Resource)]
pub struct AssetRepo {
    pub assets: HashMap<Uuid, Asset>,
}

impl AssetRepo {
    pub fn reload_repo(&mut self, asset_defs: Vec<&impl AssetDefinition>) {
        self.assets.clear();
        self.convert(asset_defs);
        self.convert(game_sources::get_game_sources());
    }

    fn convert(&mut self, asset_defs: Vec<&impl AssetDefinition>) {
        for var_def in asset_defs {
            self.assets.insert(
                var_def.asset_id().id.clone(),
                Asset {
                    id: var_def.asset_id().clone(),
                    source: var_def.as_asset_source(),
                },
            );
        }
    }

    pub fn get_number(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<f32> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_number(self, entry))
    }

    pub fn get_text(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<String> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_text(self, entry))
    }

    pub fn get_color(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<Color> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_color(self, entry))
    }
    pub fn get_bool(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_bool(self, entry))
    }

    pub fn get_number_property(
        &self,
        property: &NumberProperty,
        entry: Option<&Entry>,
    ) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(reference) => self.get_number(reference, entry),
        }
    }

    pub fn get_text_property(
        &self,
        property: &TextProperty,
        entry: Option<&Entry>,
    ) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(reference) => self.get_text(reference, entry),
        }
    }

    pub fn get_color_property(
        &self,
        property: &ColorProperty,
        entry: Option<&Entry>,
    ) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(reference) => self.get_color(reference, entry),
        }
    }

    pub fn get_bool_property(
        &self,
        property: &BooleanProperty,
        entry: Option<&Entry>,
    ) -> Option<bool> {
        match property {
            BooleanProperty::Fixed(b) => Some(*b),
            BooleanProperty::Ref(reference) => self.get_bool(reference, entry),
        }
    }
}

impl AssetSource {
    pub fn resolve_number(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32> {
        match self {
            AssetSource::Number(s) => s.resolve(vars, entry),
            _ => None,
        }
    }

    pub fn resolve_text(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<String> {
        match self {
            AssetSource::Text(s) => s.resolve(vars, entry),
            AssetSource::Number(s) => s.resolve(vars, entry).map(|n| format!("{n}")),
            _ => None,
        }
    }

    pub fn resolve_color(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<Color> {
        match self {
            AssetSource::Color(s) => s.resolve(vars, entry),
            _ => None,
        }
    }
    pub fn resolve_bool(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        match self {
            AssetSource::Bool(s) => s.resolve(vars, entry),
            _ => None,
        }
    }
}

impl AssetType {
    pub fn can_cast_to(&self, other: &AssetType) -> bool {
        match (self, other) {
            (ref a, ref b) if a == b => true,
            (AssetType::Number, AssetType::Text) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            AssetType::Number => "Number",
            AssetType::Text => "Text",
            AssetType::Color => "Color",
            AssetType::Boolean => "Boolean",
        }
    }
}

/*
*-------------------------------------------------------------------------------------------------------

*/

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
