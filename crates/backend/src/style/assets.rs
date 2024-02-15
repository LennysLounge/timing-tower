use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{
    exact_variant::ExactVariant,
    value_store::{AnyValueProducer, ValueId},
    value_types::{Font, Texture, ValueType},
};

use super::{variables::StaticValueProducer, StyleId, StyleItem};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: StyleId,
    pub name: String,
    pub value_type: ValueType,
    pub path: String,
}
impl AssetDefinition {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("image"),
            value_type: ValueType::Texture,
            path: String::new(),
        }
    }
    pub fn value_producer(&self) -> AnyValueProducer {
        match self.value_type {
            ValueType::Texture => StaticValueProducer(Texture::Handle(self.value_id().0)).into(),
            ValueType::Font => StaticValueProducer(Font::Handle(self.value_id().0)).into(),
            value_type @ _ => {
                unreachable!("An asset cannot have a value type of {}", value_type.name())
            }
        }
    }
    pub fn value_id(&self) -> ValueId {
        ValueId(self.id.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AssetFolder {
    pub id: StyleId,
    pub name: String,
    pub content: Vec<AssetOrFolder>,
}
impl AssetFolder {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
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
    pub fn id(&self) -> &StyleId {
        match self {
            AssetOrFolder::Asset(o) => &o.id,
            AssetOrFolder::Folder(o) => &o.id,
        }
    }
}
