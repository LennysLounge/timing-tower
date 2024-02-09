use std::ops::Deref;

use crate::{
    exact_variant::ExactVariant,
    value_store::{ValueId, ValueProducer, ValueStore},
    value_types::{Boolean, Font, Number, Text, Texture, Tint},
};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use self::{condition::Condition, fixed_value::FixedValue, map::Map};

use super::{StyleId, StyleItem};

pub mod condition;
pub mod fixed_value;
pub mod map;

#[derive(Serialize, Deserialize, Clone)]
pub struct VariableDefinition {
    pub id: StyleId,
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
            id: StyleId::new(),
            name: "Variables".to_string(),
            behavior: VariableBehavior::FixedValue(FixedValue::default()),
        }
    }
    pub fn value_producer(&self) -> Box<dyn ValueProducer + Sync + Send> {
        match &self.behavior {
            VariableBehavior::FixedValue(o) => o.as_typed_producer(),
            VariableBehavior::Condition(o) => o.as_typed_producer(),
            VariableBehavior::Map(o) => o.as_typed_producer(),
        }
    }
    pub fn value_id(&self) -> ValueId {
        ValueId(self.id.0)
    }
}

pub struct StaticValueProducer<T>(pub T);
impl ValueProducer for StaticValueProducer<Number> {
    fn get_number(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Number> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticValueProducer<Text> {
    fn get_text(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Text> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticValueProducer<Boolean> {
    fn get_boolean(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Boolean> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticValueProducer<Tint> {
    fn get_tint(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Tint> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticValueProducer<Texture> {
    fn get_texture(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Texture> {
        Some(self.0.clone())
    }
}
impl ValueProducer for StaticValueProducer<Font> {
    fn get_font(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<Font> {
        Some(self.0.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VariableFolder {
    pub id: StyleId,
    pub name: String,
    pub content: Vec<VariableOrFolder>,
}
impl VariableFolder {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("Group"),
            content: Vec::new(),
        }
    }
    pub fn contained_variables(&self) -> Vec<&VariableDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                VariableOrFolder::Variable(a) => vec![a.deref()],
                VariableOrFolder::Folder(f) => f.contained_variables(),
            })
            .collect()
    }
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum VariableOrFolder {
    Variable(ExactVariant<StyleItem, VariableDefinition>),
    Folder(ExactVariant<StyleItem, VariableFolder>),
}
impl VariableOrFolder {
    pub fn id(&self) -> &StyleId {
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
