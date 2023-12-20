use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, TypedValueProducer},
    value_types::{Texture, ValueType},
};

use super::{
    variables::StaticValueProducer,
    visitor::{Node, NodeMut, NodeVisitor, NodeVisitorMut, Visitable},
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
impl Visitable for AssetDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::Asset(self))
    }

    fn leave(&self, _visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(NodeMut::Asset(self))
    }

    fn leave_mut(&mut self, _visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
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
impl Visitable for AssetFolder {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.content.iter().try_for_each(|f| match f {
            AssetOrFolder::Asset(o) => o.walk(visitor),
            AssetOrFolder::Folder(o) => o.walk(visitor),
        })?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::AssetFolder(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::AssetFolder(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.content.iter_mut().try_for_each(|f| match f {
            AssetOrFolder::Asset(o) => o.walk_mut(visitor),
            AssetOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(NodeMut::AssetFolder(self))
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave(NodeMut::AssetFolder(self))
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
