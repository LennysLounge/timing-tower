use std::{collections::HashMap, marker::PhantomData};

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

pub struct ValueRef<T> {
    id: Uuid,
    phantom: PhantomData<T>,
}

pub enum Property<T> {
    Fixed(T),
    ValueRef(ValueRef<T>),
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

    pub fn get_color(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<Color> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_tint(self, entry))
    }
    pub fn get_bool(&self, reference: &AssetReference, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_bool(self, entry))
    }
    pub fn get_image(
        &self,
        reference: &AssetReference,
        entry: Option<&Entry>,
    ) -> Option<Handle<Image>> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.resolve_texture(self, entry))
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

    pub fn resolve_tint(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<Color> {
        match self {
            TypedValueProducer::Tint(s) => s.get(vars, entry).map(|n| n.0),
            _ => None,
        }
    }
    pub fn resolve_bool(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        match self {
            TypedValueProducer::Boolean(s) => s.get(vars, entry).map(|n| n.0),
            _ => None,
        }
    }
    pub fn resolve_texture(
        &self,
        vars: &ValueStore,
        entry: Option<&Entry>,
    ) -> Option<Handle<Image>> {
        match self {
            TypedValueProducer::Texture(s) => s.get(vars, entry).map(|n| n.0),
            _ => None,
        }
    }
}

trait TypedValueResolver<T> {
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
