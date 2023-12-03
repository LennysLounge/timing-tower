use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::value_store::{IntoValueProducer, TypedValueProducer, ValueProducer, ValueStore};

use self::{condition::Condition, fixed_value::FixedValue, map::Map};

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

impl VariableDefinition {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Variables".to_string(),
            behavior: VariableBehavior::FixedValue(FixedValue::default()),
        }
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
