use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, ValueProducer},
    value_types::{Font, Texture, ValueType},
};

use super::{
    variables::StaticValueProducer, OwnedStyleItem, StyleItem, StyleItemMut, StyleItemRef,
};

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
}
impl IntoValueProducer for AssetDefinition {
    fn get_value_producer(&self) -> (Uuid, Box<dyn ValueProducer + Sync + Send>) {
        let typed_value_producer: Box<dyn ValueProducer + Sync + Send> = match self.value_type {
            ValueType::Texture => Box::new(StaticValueProducer(Texture::Handle(self.id))),
            ValueType::Font => Box::new(StaticValueProducer(Font::Handle(self.id))),
            value_type @ _ => {
                unreachable!("An asset cannot have a value type of {}", value_type.name())
            }
        };
        (self.id, typed_value_producer)
    }
}
impl StyleItem for AssetDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::Asset(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::Asset(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::Asset(self)
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
                AssetOrFolder::Asset(a) => vec![a],
                AssetOrFolder::Folder(f) => f.contained_assets(),
            })
            .collect()
    }
}
impl StyleItem for AssetFolder {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::AssetFolder(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::AssetFolder(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::AssetFolder(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum AssetOrFolder {
    Asset(AssetDefinition),
    Folder(AssetFolder),
}
impl AssetOrFolder {
    pub fn id(&self) -> &Uuid {
        match self {
            AssetOrFolder::Asset(o) => &o.id,
            AssetOrFolder::Folder(o) => &o.id,
        }
    }
}
