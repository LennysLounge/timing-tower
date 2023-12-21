use unified_sim_model::model::Entry;

use crate::reference_store::{IntoProducerData, ProducerData};
use backend::{
    style::variables::{VariableBehavior, VariableDefinition},
    value_store::{ValueProducer, ValueStore},
};

impl IntoProducerData for VariableDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id,
            name: self.name.clone(),
            value_type: match &self.behavior {
                VariableBehavior::FixedValue(o) => o.output_type(),
                VariableBehavior::Condition(o) => o.output_type(),
                VariableBehavior::Map(o) => o.output_type(),
            },
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
