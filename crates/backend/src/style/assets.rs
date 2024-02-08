use std::ops::Deref;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    exact_variant::ExactVariant,
    value_store::{ValueId, ValueProducer},
    value_types::{Font, Texture, ValueType},
};

use super::{variables::StaticValueProducer, StyleItem};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: Uuid,
    pub name: String,
    pub value_type: ValueType,
    pub path: String,
}
impl AssetDefinition {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("image"),
            value_type: ValueType::Texture,
            path: String::new(),
        }
    }
    pub fn value_producer(&self) -> Box<dyn ValueProducer + Sync + Send> {
        match self.value_type {
            ValueType::Texture => Box::new(StaticValueProducer(Texture::Handle(self.id))),
            ValueType::Font => Box::new(StaticValueProducer(Font::Handle(self.id))),
            value_type @ _ => {
                unreachable!("An asset cannot have a value type of {}", value_type.name())
            }
        }
    }
    pub fn value_id(&self) -> ValueId {
        ValueId(self.id)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AssetFolder {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<AssetOrFolder>,
}
impl AssetFolder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Group"),
            content: Vec::new(),
        }
    }
    pub fn contained_assets(&self) -> Vec<&AssetDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                AssetOrFolder::Asset(a) => vec![a.deref()],
                AssetOrFolder::Folder(f) => f.contained_assets(),
            })
            .collect()
    }
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum AssetOrFolder {
    Asset(ExactVariant<StyleItem, AssetDefinition>),
    Folder(ExactVariant<StyleItem, AssetFolder>),
}
impl AssetOrFolder {
    pub fn id(&self) -> &Uuid {
        match self {
            AssetOrFolder::Asset(o) => &o.id,
            AssetOrFolder::Folder(o) => &o.id,
        }
    }
}
