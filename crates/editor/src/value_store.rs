use std::collections::HashMap;

use bevy::prelude::{Color, Handle, Image, Resource};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    game_sources,
    style::properties::{
        BooleanProperty, ColorProperty, ImageProperty, NumberProperty, TextProperty,
    },
};

use self::types::{Boolean, Number, Text, Texture, Tint};

pub mod types {
    use bevy::{
        asset::Handle,
        render::{color::Color, texture::Image},
    };

    pub struct Number(pub f32);
    pub struct Text(pub String);
    pub struct Tint(pub Color);
    pub struct Boolean(pub bool);
    pub struct Texture(pub Handle<Image>);
}

pub trait ValueProducer {
    #[allow(unused)]
    fn get_number(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        None
    }
    #[allow(unused)]
    fn get_text(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        None
    }
    #[allow(unused)]
    fn get_tint(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        None
    }
    #[allow(unused)]
    fn get_boolean(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        None
    }
    #[allow(unused)]
    fn get_texture(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        None
    }
}

pub trait NumberSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<Color>;
}

pub trait BooleanSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool>;
}

pub trait ImageSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<Handle<Image>>;
}

pub trait IntoValueProducer {
    fn get_value_producer(&self) -> Box<dyn ValueProducer + Send + Sync>;
    fn asset_id(&self) -> &AssetId;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default, Copy)]
pub enum AssetType {
    #[default]
    Number,
    Text,
    Color,
    Boolean,
    Image,
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
    pub source: Box<dyn ValueProducer + Send + Sync>,
}

#[derive(Resource)]
pub struct ValueStore {
    pub assets: HashMap<Uuid, Asset>,
}

impl ValueStore {
    pub fn reload_repo(
        &mut self,
        vars: Vec<&impl IntoValueProducer>,
        assets: Vec<&impl IntoValueProducer>,
    ) {
        self.assets.clear();
        self.convert(vars);
        self.convert(assets);
        self.convert(game_sources::get_game_sources());
    }

    fn convert(&mut self, asset_defs: Vec<&impl IntoValueProducer>) {
        for var_def in asset_defs {
            self.assets.insert(
                var_def.asset_id().id.clone(),
                Asset {
                    id: var_def.asset_id().clone(),
                    source: var_def.get_value_producer(),
                },
            );
        }
    }

    pub fn get_number(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<f32> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.get_number(self, entry))
            .map(|n| n.0)
    }

    pub fn get_text(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<String> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.get_text(self, entry))
            .map(|t| t.0)
    }

    pub fn get_color(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<Color> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.get_tint(self, entry))
            .map(|t| t.0)
    }
    pub fn get_bool(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.get_boolean(self, entry))
            .map(|t| t.0)
    }
    pub fn get_image(
        &self,
        reference: &AssetReference,
        entry: Option<&Entry>,
    ) -> Option<Handle<Image>> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.get_texture(self, entry))
            .map(|t| t.0)
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

    pub fn get_image_property(
        &self,
        property: &ImageProperty,
        entry: Option<&Entry>,
    ) -> Option<Handle<Image>> {
        match property {
            ImageProperty::None => None,
            ImageProperty::Ref(reference) => self.get_image(reference, entry),
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
            AssetType::Image => "Image",
        }
    }
}
