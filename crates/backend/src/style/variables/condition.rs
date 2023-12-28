use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    value_store::{TypedValueProducer, TypedValueResolver, ValueProducer, ValueStore},
    value_types::{
        Boolean, Number, Property, Text, Texture, Tint, UntypedValueRef, ValueRef, ValueType,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    pub comparison: Comparison,
    #[serde(flatten)]
    pub output: UntypedOutput,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            comparison: Comparison::Number {
                left: ValueRef::default(),
                comparator: NumberComparator::Equal,
                right: Property::default(),
            },
            output: UntypedOutput::Number(Output::default()),
        }
    }
}

impl Condition {
    pub fn as_typed_producer(&self) -> TypedValueProducer {
        match self.output.clone() {
            UntypedOutput::Number(output) => TypedValueProducer::Number(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            })),
            UntypedOutput::Text(output) => TypedValueProducer::Text(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            })),
            UntypedOutput::Color(output) => TypedValueProducer::Tint(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            })),
            UntypedOutput::Boolean(output) => TypedValueProducer::Boolean(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            })),
            UntypedOutput::Image(output) => TypedValueProducer::Texture(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            })),
        }
    }

    pub fn output_type(&self) -> ValueType {
        match self.output {
            UntypedOutput::Number { .. } => ValueType::Number,
            UntypedOutput::Text { .. } => ValueType::Text,
            UntypedOutput::Color { .. } => ValueType::Tint,
            UntypedOutput::Boolean { .. } => ValueType::Boolean,
            UntypedOutput::Image { .. } => ValueType::Texture,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "comparison_type")]
pub enum Comparison {
    Number {
        left: ValueRef<Number>,
        comparator: NumberComparator,
        right: Property<Number>,
    },
    Text {
        left: ValueRef<Text>,
        comparator: TextComparator,
        right: Property<Text>,
    },
    Boolean {
        left: ValueRef<Boolean>,
        comparator: BooleanComparator,
        right: Property<Boolean>,
    },
}

impl Comparison {
    pub fn left_side_id(&self) -> &Uuid {
        match self {
            Comparison::Number { left, .. } => &left.id,
            Comparison::Text { left, .. } => &left.id,
            Comparison::Boolean { left, .. } => &left.id,
        }
    }
    pub fn value_type(&self) -> ValueType {
        match self {
            Comparison::Number { .. } => ValueType::Number,
            Comparison::Text { .. } => ValueType::Text,
            Comparison::Boolean { .. } => ValueType::Boolean,
        }
    }

    pub fn set_left_side(&mut self, new_untyped_ref: UntypedValueRef) {
        if self.value_type() == new_untyped_ref.value_type {
            // Update the left side if the types are the same.
            match self {
                Comparison::Number { left, .. } => *left = new_untyped_ref.typed(),
                Comparison::Boolean { left, .. } => *left = new_untyped_ref.typed(),
                Comparison::Text { left, .. } => *left = new_untyped_ref.typed(),
            }
        } else {
            // Otherwise change the type of the entire comparison.
            *self = match new_untyped_ref.value_type {
                ValueType::Number => Comparison::Number {
                    left: new_untyped_ref.typed(),
                    comparator: NumberComparator::Equal,
                    right: Property::default(),
                },
                ValueType::Text => Comparison::Text {
                    left: new_untyped_ref.typed(),
                    comparator: TextComparator::Like,
                    right: Property::default(),
                },
                ValueType::Boolean => Comparison::Boolean {
                    left: new_untyped_ref.typed(),
                    comparator: BooleanComparator::Is,
                    right: Property::default(),
                },
                value_type @ _ => {
                    unreachable!("Type {} is not allowd for if condition", value_type.name())
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NumberComparator {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TextComparator {
    Like,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum BooleanComparator {
    Is,
    IsNot,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
pub enum UntypedOutput {
    Number(Output<Number>),
    Text(Output<Text>),
    Color(Output<Tint>),
    Boolean(Output<Boolean>),
    Image(Output<Texture>),
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Output<T> {
    pub truee: Property<T>,
    pub falsee: Property<T>,
}

struct ConditionProducer<T> {
    comparison: Comparison,
    output: Output<T>,
}

impl<T> ConditionProducer<T> {
    fn evaluate_condition(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        match &self.comparison {
            Comparison::Number {
                left,
                comparator,
                right,
            } => {
                let left = vars.get(&left, entry)?.0;
                let right = vars.get_property(&right, entry)?.0;
                Some(match comparator {
                    NumberComparator::Equal => left == right,
                    NumberComparator::Greater => left > right,
                    NumberComparator::GreaterEqual => left >= right,
                    NumberComparator::Less => left < right,
                    NumberComparator::LessEqual => left <= right,
                })
            }
            Comparison::Text {
                left,
                comparator,
                right,
            } => {
                let left = vars.get(&left, entry)?.0;
                let right = vars.get_property(&right, entry)?.0;
                Some(match comparator {
                    TextComparator::Like => left == right,
                })
            }
            Comparison::Boolean {
                left,
                comparator,
                right,
            } => {
                let left = vars.get(&left, entry)?.0;
                let right = vars.get_property(&right, entry)?.0;
                Some(match comparator {
                    BooleanComparator::Is => left == right,
                    BooleanComparator::IsNot => left != right,
                })
            }
        }
    }
}

impl<T> ValueProducer<T> for ConditionProducer<T>
where
    ValueStore: TypedValueResolver<T>,
    T: Clone,
{
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T> {
        let condition = self.evaluate_condition(value_store, entry)?;

        if condition {
            value_store.get_property(&self.output.truee, entry)
        } else {
            value_store.get_property(&self.output.falsee, entry)
        }
    }
}
