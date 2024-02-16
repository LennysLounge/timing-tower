use enumcapsulate::macros::AsVariantRef;
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    value_store::{AnyValueProducer, ProducerId, ValueProducer, ValueResolver, ValueStore},
    value_types::{
        AnyProducerRef, Boolean, Number, ProducerRef, Property, Text, Texture, Tint, ValueType,
    },
};

use super::{BooleanComparator, NumberComparator, TextComparator};

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
                left: ProducerRef::default(),
                comparator: NumberComparator::Equal,
                right: Property::default(),
            },
            output: UntypedOutput::Number(Output::default()),
        }
    }
}

impl Condition {
    pub fn as_typed_producer(&self) -> AnyValueProducer {
        match self.output.clone() {
            UntypedOutput::Number(output) => ConditionProducer {
                comparison: self.comparison.clone(),
                output,
            }
            .into(),
            UntypedOutput::Text(output) => ConditionProducer {
                comparison: self.comparison.clone(),
                output,
            }
            .into(),
            UntypedOutput::Color(output) => ConditionProducer {
                comparison: self.comparison.clone(),
                output,
            }
            .into(),
            UntypedOutput::Boolean(output) => ConditionProducer {
                comparison: self.comparison.clone(),
                output,
            }
            .into(),
            UntypedOutput::Image(output) => ConditionProducer {
                comparison: self.comparison.clone(),
                output,
            }
            .into(),
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
        left: ProducerRef<Number>,
        comparator: NumberComparator,
        right: Property<Number>,
    },
    Text {
        left: ProducerRef<Text>,
        comparator: TextComparator,
        right: Property<Text>,
    },
    Boolean {
        left: ProducerRef<Boolean>,
        comparator: BooleanComparator,
        right: Property<Boolean>,
    },
}

impl Comparison {
    pub fn left_side_id(&self) -> ProducerId {
        match self {
            Comparison::Number { left, .. } => left.id(),
            Comparison::Text { left, .. } => left.id(),
            Comparison::Boolean { left, .. } => left.id(),
        }
    }
    pub fn value_type(&self) -> ValueType {
        match self {
            Comparison::Number { .. } => ValueType::Number,
            Comparison::Text { .. } => ValueType::Text,
            Comparison::Boolean { .. } => ValueType::Boolean,
        }
    }
    pub fn left_side_ref(&self) -> AnyProducerRef {
        match self {
            Comparison::Number { left, .. } => left.clone().to_any_producer_ref(),
            Comparison::Text { left, .. } => left.clone().to_any_producer_ref(),
            Comparison::Boolean { left, .. } => left.clone().to_any_producer_ref(),
        }
    }

    pub fn set_left_side(&mut self, new_untyped_ref: AnyProducerRef) {
        match (self, new_untyped_ref.ty()) {
            // If the new value is of the same type as the comparison then we only
            // need to update the reference.
            (Comparison::Number { left, .. }, ValueType::Number) => {
                *left = new_untyped_ref
                    .to_typed()
                    .expect("Value types should match")
            }
            (Comparison::Text { left, .. }, ValueType::Text) => {
                *left = new_untyped_ref
                    .to_typed()
                    .expect("Value types should match")
            }
            (Comparison::Boolean { left, .. }, ValueType::Boolean) => {
                *left = new_untyped_ref
                    .to_typed()
                    .expect("Value types should match")
            }
            // Otherwise we have to change the type of the comparision
            (me @ _, ValueType::Number) => {
                *me = Comparison::Number {
                    left: new_untyped_ref
                        .to_typed()
                        .expect("Value types should match"),
                    comparator: NumberComparator::Equal,
                    right: Property::default(),
                }
            }
            (me @ _, ValueType::Text) => {
                *me = Comparison::Text {
                    left: new_untyped_ref
                        .to_typed()
                        .expect("Value types should match"),
                    comparator: TextComparator::Like,
                    right: Property::default(),
                }
            }
            (me @ _, ValueType::Boolean) => {
                *me = Comparison::Boolean {
                    left: new_untyped_ref
                        .to_typed()
                        .expect("Value types should match"),
                    comparator: BooleanComparator::Is,
                    right: Property::default(),
                }
            }
            // Any other value types are not allowed for this comparison
            (_, value_type @ _) => {
                // TODO: This isnt unreachable based on this method alone and should
                // probably be an error. Otherwise this method could be moved somewhere
                // where this condition is actually unreachable.
                unreachable!("Type {} is not allowd for if condition", value_type.name())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, AsVariantRef)]
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
            } => Some(comparator.compare(
                vars.get(&left, entry)?.0,
                vars.get_property(&right, entry)?.0,
            )),
            Comparison::Text {
                left,
                comparator,
                right,
            } => Some(comparator.compare(
                &vars.get(&left, entry)?.0,
                &vars.get_property(&right, entry)?.0,
            )),
            Comparison::Boolean {
                left,
                comparator,
                right,
            } => Some(comparator.compare(
                vars.get(&left, entry)?.0,
                vars.get_property(&right, entry)?.0,
            )),
        }
    }
    fn resolve(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T>
    where
        ValueStore: ValueResolver<T>,
        T: Clone,
    {
        let condition = self.evaluate_condition(value_store, entry)?;

        if condition {
            value_store.get_property(&self.output.truee, entry)
        } else {
            value_store.get_property(&self.output.falsee, entry)
        }
    }
}
impl ValueProducer for ConditionProducer<Number> {
    type Output = Number;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        self.resolve(value_store, entry)
    }
}
impl ValueProducer for ConditionProducer<Text> {
    type Output = Text;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        self.resolve(value_store, entry)
    }
}
impl ValueProducer for ConditionProducer<Boolean> {
    type Output = Boolean;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        self.resolve(value_store, entry)
    }
}
impl ValueProducer for ConditionProducer<Texture> {
    type Output = Texture;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        self.resolve(value_store, entry)
    }
}
impl ValueProducer for ConditionProducer<Tint> {
    type Output = Tint;
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        self.resolve(value_store, entry)
    }
}
