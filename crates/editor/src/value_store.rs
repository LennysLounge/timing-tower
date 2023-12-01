use std::{collections::HashMap, marker::PhantomData};

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    game_sources,
    style::properties::Property,
    value_types::{Boolean, Number, Text, Texture, Tint, ValueType},
};

pub trait ValueProducer<T> {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T>;
}

pub trait IntoValueProducer {
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer);
    fn asset_id(&self) -> &AssetId;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct ValueRef<T> {
    pub id: Uuid,
    #[serde(skip)]
    pub phantom: PhantomData<T>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UntypedValueRef {
    pub id: Uuid,
    pub value_type: ValueType,
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
            let (id, value_producer) = var_def.get_value_producer();
            self.assets.insert(id, value_producer);
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
            TypedValueProducer::Boolean(p) => p.get(self, entry).map(|b| {
                if b.0 {
                    Text(String::from("Yes"))
                } else {
                    Text(String::from("No"))
                }
            }),
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
