use serde::{Deserialize, Serialize};

use crate::{
    value_store::AnyValueProducer,
    value_types::{Boolean, Number, Text, Tint, ValueType},
};

use super::StaticValueProducer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
pub enum FixedValue {
    Number(Number),
    Text(Text),
    Tint(Tint),
    Boolean(Boolean),
}
impl Default for FixedValue {
    fn default() -> Self {
        Self::Number(Number::default())
    }
}

impl FixedValue {
    pub fn output_type(&self) -> ValueType {
        match self {
            FixedValue::Number(_) => ValueType::Number,
            FixedValue::Text(_) => ValueType::Text,
            FixedValue::Tint(_) => ValueType::Tint,
            FixedValue::Boolean(_) => ValueType::Boolean,
        }
    }

    pub fn as_typed_producer(&self) -> AnyValueProducer {
        match self.clone() {
            FixedValue::Number(n) => StaticValueProducer(n).into(),
            FixedValue::Text(t) => StaticValueProducer(t).into(),
            FixedValue::Tint(c) => StaticValueProducer(c).into(),
            FixedValue::Boolean(b) => StaticValueProducer(b).into(),
        }
    }
}
