use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::value_store::{IntoValueProducer, TypedValueProducer, ValueProducer, ValueStore};

use self::{condition::Condition, fixed_value::FixedValue, map::Map};

use super::{
    visitor::{NodeVisitor, NodeVisitorMut, Visitable, Node},
    StyleNode,
};

pub mod condition;
pub mod fixed_value;
pub mod map;

#[derive(Serialize, Deserialize, Clone)]
pub struct VariableDefinition {
    pub id: Uuid,
    pub name: String,
    #[serde(flatten)]
    pub behavior: VariableBehavior,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "behavior")]
pub enum VariableBehavior {
    FixedValue(FixedValue),
    Condition(Condition),
    Map(Map),
}

impl VariableDefinition {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Variables".to_string(),
            behavior: VariableBehavior::FixedValue(FixedValue::default()),
        }
    }
}
impl IntoValueProducer for VariableDefinition {
    fn get_value_producer(&self) -> (Uuid, TypedValueProducer) {
        let producer = match &self.behavior {
            VariableBehavior::FixedValue(o) => o.as_typed_producer(),
            VariableBehavior::Condition(o) => o.as_typed_producer(),
            VariableBehavior::Map(o) => o.as_typed_producer(),
        };
        (self.id, producer)
    }
}
impl StyleNode for VariableDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
impl Visitable for VariableDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::Variable(self))
    }

    fn leave(&self, _visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_variable(self)
    }

    fn leave_mut(&mut self, _visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

pub struct StaticValueProducer<T>(pub T);
impl<T> ValueProducer<T> for StaticValueProducer<T>
where
    T: Clone,
{
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<T> {
        Some(self.0.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VariableFolder {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<VariableOrFolder>,
}
impl VariableFolder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Group"),
            content: Vec::new(),
        }
    }
    pub fn contained_variables(&self) -> Vec<&VariableDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                VariableOrFolder::Variable(a) => vec![a],
                VariableOrFolder::Folder(f) => f.contained_variables(),
            })
            .collect()
    }
}
impl StyleNode for VariableFolder {
    fn id(&self) -> &Uuid {
        &self.id
    }
}
impl Visitable for VariableFolder {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.content.iter().try_for_each(|f| match f {
            VariableOrFolder::Variable(o) => o.walk(visitor),
            VariableOrFolder::Folder(o) => o.walk(visitor),
        })?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::VariableFolder(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::VariableFolder(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.content.iter_mut().try_for_each(|f| match f {
            VariableOrFolder::Variable(o) => o.walk_mut(visitor),
            VariableOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_variable_folder(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_variable_folder(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum VariableOrFolder {
    Variable(VariableDefinition),
    Folder(VariableFolder),
}
impl VariableOrFolder {
    pub fn id(&self) -> &Uuid {
        match self {
            VariableOrFolder::Variable(o) => &o.id,
            VariableOrFolder::Folder(o) => &o.id,
        }
    }
}
