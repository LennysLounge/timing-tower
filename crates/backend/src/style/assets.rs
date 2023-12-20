use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, TypedValueProducer},
    value_types::{Texture, ValueType},
};

use super::{
    variables::StaticValueProducer,
    visitor::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
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
            _ => unreachable!(),
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
impl NodeIterator for AssetDefinition {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)
    }
}
impl NodeIteratorMut for AssetDefinition {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)
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
impl NodeIterator for AssetFolder {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.content.iter().try_for_each(|v| match v {
            AssetOrFolder::Asset(o) => o.walk(f),
            AssetOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for AssetFolder {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.content.iter_mut().try_for_each(|v| match v {
            AssetOrFolder::Asset(o) => o.walk_mut(f),
            AssetOrFolder::Folder(o) => o.walk_mut(f),
        })?;
        f(self.as_node_mut(), Method::Leave)
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
