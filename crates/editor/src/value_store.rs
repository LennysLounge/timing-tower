use std::{collections::HashMap, marker::PhantomData};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{game_sources, style::properties::Property};

use self::types::{Boolean, Number, Text, Texture, Tint};

pub mod types {
    use bevy::{
        asset::Handle,
        render::{color::Color, texture::Image},
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Number(pub f32);

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Text(pub String);

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Tint(pub Color);

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Boolean(pub bool);

    #[derive(Serialize, Deserialize, Clone, Default)]
    pub enum Texture {
        #[default]
        None,
        #[serde(skip)]
        Handle(Handle<Image>),
    }
}

pub trait ValueProducer<T> {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T>;
}

pub trait IntoValueProducer {
    fn get_value_producer(&self) -> TypedValueProducer;
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct ValueRef<T> {
    pub id: Uuid,
    #[serde(skip)]
    pub phantom: PhantomData<T>,
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

pub enum TypedValueProducer {
    Number(Box<dyn ValueProducer<Number> + Send + Sync>),
    Text(Box<dyn ValueProducer<Text> + Send + Sync>),
    Tint(Box<dyn ValueProducer<Tint> + Send + Sync>),
    Boolean(Box<dyn ValueProducer<Boolean> + Send + Sync>),
    Texture(Box<dyn ValueProducer<Texture> + Send + Sync>),
}

#[derive(Resource)]
pub struct ValueStore {
    pub assets: HashMap<Uuid, TypedValueProducer>,
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
            self.assets
                .insert(var_def.asset_id().id.clone(), var_def.get_value_producer());
        }
    }

    pub fn get<T>(&self, value_ref: &ValueRef<T>, entry: Option<&Entry>) -> Option<T>
    where
        Self: TypedValueResolver<T>,
    {
        self.assets
            .get(&value_ref.id)
            .and_then(|p| self.get_typed(p, entry))
    }

    pub fn get_property<T>(&self, property: &Property<T>, entry: Option<&Entry>) -> Option<T>
    where
        Self: TypedValueResolver<T>,
        T: Clone,
    {
        match property {
            Property::Fixed(v) => Some(v.clone()),
            Property::ValueRef(value_ref) => self.get(value_ref, entry),
        }
    }

    pub fn get_number(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<f32> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_number(self, entry))
    }

    pub fn get_text(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<String> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_text(self, entry))
    }

    pub fn get_bool(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_bool(self, entry))
    }
}

impl TypedValueProducer {
    pub fn resolve_number(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<f32> {
        match self {
            TypedValueProducer::Number(s) => s.get(vars, entry).map(|n| n.0),
            _ => None,
        }
    }

    pub fn resolve_text(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<String> {
        match self {
            TypedValueProducer::Text(s) => s.get(vars, entry).map(|n| n.0),
            TypedValueProducer::Number(s) => s.get(vars, entry).map(|n| format!("{}", n.0)),
            _ => None,
        }
    }

    pub fn resolve_bool(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        match self {
            TypedValueProducer::Boolean(s) => s.get(vars, entry).map(|n| n.0),
            _ => None,
        }
    }
}

pub trait TypedValueResolver<T> {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<T>;
}
impl TypedValueResolver<Number> for ValueStore {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<Number> {
        match producer {
            TypedValueProducer::Number(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl TypedValueResolver<Text> for ValueStore {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<Text> {
        match producer {
            TypedValueProducer::Number(p) => p.get(self, entry).map(|n| Text(format!("{}", n.0))),
            TypedValueProducer::Text(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl TypedValueResolver<Tint> for ValueStore {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<Tint> {
        match producer {
            TypedValueProducer::Tint(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl TypedValueResolver<Boolean> for ValueStore {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<Boolean> {
        match producer {
            TypedValueProducer::Boolean(p) => p.get(self, entry),
            _ => None,
        }
    }
}
impl TypedValueResolver<Texture> for ValueStore {
    fn get_typed(&self, producer: &TypedValueProducer, entry: Option<&Entry>) -> Option<Texture> {
        match producer {
            TypedValueProducer::Texture(p) => p.get(self, entry),
            _ => None,
        }
    }
}
