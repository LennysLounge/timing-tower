use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::value_store::{IntoValueProducer, TypedValueProducer, ValueProducer, ValueStore};

use self::{condition::Condition, fixed_value::FixedValue, map::Map};

use super::{
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
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
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Variable(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Variable(self)
    }
}
impl NodeIterator for VariableDefinition {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)
    }
}
impl NodeIteratorMut for VariableDefinition {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)
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

    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::VariableFolder(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::VariableFolder(self)
    }
}
impl NodeIterator for VariableFolder {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.content.iter().try_for_each(|v| match v {
            VariableOrFolder::Variable(o) => o.walk(f),
            VariableOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for VariableFolder {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.content.iter_mut().try_for_each(|c| match c {
            VariableOrFolder::Variable(o) => o.walk_mut(f),
            VariableOrFolder::Folder(o) => o.walk_mut(f),
        })?;
        f(self.as_node_mut(), Method::Leave)
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum NumberComparator {
    #[default]
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
impl NumberComparator {
    fn compare(&self, n1: f32, n2: f32) -> bool {
        match self {
            NumberComparator::Equal => n1 == n2,
            NumberComparator::Greater => n1 > n2,
            NumberComparator::GreaterEqual => n1 >= n2,
            NumberComparator::Less => n1 < n2,
            NumberComparator::LessEqual => n1 <= n2,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum TextComparator {
    #[default]
    Like,
}
impl TextComparator {
    fn compare(&self, t1: &String, t2: &String) -> bool {
        match self {
            TextComparator::Like => t1 == t2,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum BooleanComparator {
    #[default]
    Is,
    IsNot,
}
impl BooleanComparator {
    fn compare(&self, b1: bool, b2: bool) -> bool {
        match self {
            BooleanComparator::Is => b1 == b2,
            BooleanComparator::IsNot => b1 != b2,
        }
    }
}
