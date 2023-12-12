use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    value_store::{IntoValueProducer, TypedValueProducer},
    value_types::{Texture, ValueType},
};

use super::{
    variables::StaticValueProducer,
    visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable},
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
            ValueType::Texture => TypedValueProducer::Texture(Box::new(StaticValueProducer(
                Texture::Handle(self.path.clone()),
            ))),
            _ => unreachable!(),
        };
        (self.id, typed_value_producer)
    }
}
impl StyleNode for AssetDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
impl Visitable for AssetDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_asset(self)
    }

    fn leave(&self, _visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_asset(self)
    }

    fn leave_mut(&mut self, _visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}
