use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, TypedValueProducer},
    value_types::{Font, Texture, ValueType},
};

use super::{
    iterator::{Node, NodeMut},
    variables::StaticValueProducer,
    StyleNode,
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
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer) {
        let typed_value_producer = match self.value_type {
            ValueType::Texture => {
                TypedValueProducer::Texture(Box::new(StaticValueProducer(Texture::Handle(self.id))))
            }
            ValueType::Font => {
                TypedValueProducer::Font(Box::new(StaticValueProducer(Font::Handle(self.id))))
            }
            value_type @ _ => {
                unreachable!("An asset cannot have a value type of {}", value_type.name())
            }
        };
        (self.id, typed_value_producer)
    }
}
impl StyleNode for AssetDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Asset(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Asset(self)
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
impl StyleNode for AssetFolder {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::AssetFolder(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::AssetFolder(self)
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
