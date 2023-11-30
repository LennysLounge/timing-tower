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

    #[derive(Serialize, Deserialize, Clone, Default)]
    pub struct Number(pub f32);

    #[derive(Serialize, Deserialize, Clone, Default)]
    pub struct Text(pub String);

    #[derive(Serialize, Deserialize, Clone, Default)]
    pub struct Tint(pub Color);

    #[derive(Serialize, Deserialize, Clone, Default)]
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
pub enum ValueType {
    #[default]
    Number,
    Text,
    Tint,
    Boolean,
    Texture,
}
impl ValueType {
    pub fn can_cast_to(&self, other: &ValueType) -> bool {
        match (self, other) {
            (ref a, ref b) if a == b => true,
            (ValueType::Number, ValueType::Text) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            ValueType::Number => "Number",
            ValueType::Text => "Text",
            ValueType::Tint => "Color",
            ValueType::Boolean => "Boolean",
            ValueType::Texture => "Image",
        }
    }
}

pub trait ValueTypeOf<T> {
    fn get() -> Self;
}

impl ValueTypeOf<Number> for ValueType {
    fn get() -> Self {
        ValueType::Number
    }
}
impl ValueTypeOf<Text> for ValueType {
    fn get() -> Self {
        ValueType::Text
    }
}
impl ValueTypeOf<Tint> for ValueType {
    fn get() -> Self {
        ValueType::Tint
    }
}
impl ValueTypeOf<Boolean> for ValueType {
    fn get() -> Self {
        ValueType::Boolean
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct ValueRef<T> {
    pub id: Uuid,
    #[serde(skip)]
    pub phantom: PhantomData<T>,
}

pub trait ToUntypedValueRef<T> {
    fn to_untyped(&self) -> UntypedValueRef;
}

impl ToUntypedValueRef<Number> for ValueRef<Number> {
    fn to_untyped(&self) -> UntypedValueRef {
        UntypedValueRef {
            id: self.id,
            value_type: ValueType::Number,
        }
    }
}

impl ToUntypedValueRef<Text> for ValueRef<Text> {
    fn to_untyped(&self) -> UntypedValueRef {
        UntypedValueRef {
            id: self.id,
            value_type: ValueType::Text,
        }
    }
}
impl ToUntypedValueRef<Tint> for ValueRef<Tint> {
    fn to_untyped(&self) -> UntypedValueRef {
        UntypedValueRef {
            id: self.id,
            value_type: ValueType::Tint,
        }
    }
}
impl ToUntypedValueRef<Boolean> for ValueRef<Boolean> {
    fn to_untyped(&self) -> UntypedValueRef {
        UntypedValueRef {
            id: self.id,
            value_type: ValueType::Boolean,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UntypedValueRef {
    pub id: Uuid,
    pub value_type: ValueType,
}

pub trait ToTypedValueRef<T> {
    fn to_typed(&self) -> Option<ValueRef<T>>;
}
impl ToTypedValueRef<Number> for UntypedValueRef {
    fn to_typed(&self) -> Option<ValueRef<Number>> {
        match self.value_type {
            ValueType::Number => Some(ValueRef {
                id: self.id,
                phantom: PhantomData,
            }),
            _ => None,
        }
    }
}
impl ToTypedValueRef<Text> for UntypedValueRef {
    fn to_typed(&self) -> Option<ValueRef<Text>> {
        match self.value_type {
            ValueType::Text => Some(ValueRef {
                id: self.id,
                phantom: PhantomData,
            }),
            ValueType::Number => Some(ValueRef {
                id: self.id,
                phantom: PhantomData,
            }),
            _ => None,
        }
    }
}
impl ToTypedValueRef<Tint> for UntypedValueRef {
    fn to_typed(&self) -> Option<ValueRef<Tint>> {
        match self.value_type {
            ValueType::Tint => Some(ValueRef {
                id: self.id,
                phantom: PhantomData,
            }),
            _ => None,
        }
    }
}
impl ToTypedValueRef<Boolean> for UntypedValueRef {
    fn to_typed(&self) -> Option<ValueRef<Boolean>> {
        match self.value_type {
            ValueType::Boolean => Some(ValueRef {
                id: self.id,
                phantom: PhantomData,
            }),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetId {
    pub id: Uuid,
    pub name: String,
    pub asset_type: ValueType,
}

impl Default for AssetId {
    fn default() -> Self {
        Self {
            name: "Variable".to_string(),
            id: Uuid::new_v4(),
            asset_type: ValueType::default(),
        }
    }
}
impl AssetId {
    pub fn get_ref(&self) -> UntypedValueRef {
        UntypedValueRef {
            value_type: self.asset_type.clone(),
            id: self.id.clone(),
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

    pub fn get_number(&self, reference: &UntypedValueRef, entry: Option<&Entry>) -> Option<f32> {
        self.assets
            .get(&reference.id)
            .and_then(|v| v.resolve_number(self, entry))
    }

    pub fn get_text(&self, reference: &UntypedValueRef, entry: Option<&Entry>) -> Option<String> {
        self.assets
            .get(&reference.id)
            .and_then(|v| v.resolve_text(self, entry))
    }

    pub fn get_bool(&self, reference: &UntypedValueRef, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.id)
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
