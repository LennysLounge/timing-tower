use enumcapsulate::{macros::AsVariantRef, AsVariantRef};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    value_store::{ValueId, ValueProducer, ValueResolver, ValueStore},
    value_types::{
        Boolean, Number, Property, Text, Texture, Tint, UntypedValueRef, ValueRef, ValueType,
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
                left: ValueRef::default(),
                comparator: NumberComparator::Equal,
                right: Property::default(),
            },
            output: UntypedOutput::Number(Output::default()),
        }
    }
}

impl Condition {
    pub fn as_typed_producer(&self) -> Box<dyn ValueProducer + Sync + Send> {
        match self.output.clone() {
            output @ UntypedOutput::Number(_) => Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            }),
            output @ UntypedOutput::Text(_) => Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            }),
            output @ UntypedOutput::Color(_) => Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            }),
            output @ UntypedOutput::Boolean(_) => Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            }),
            output @ UntypedOutput::Image(_) => Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    output,
                }
            }),
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
    pub fn left_side_id(&self) -> &ValueId {
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
        match (self, new_untyped_ref.value_type) {
            // If the new value is of the same type as the comparison then we only
            // need to update the reference.
            (Comparison::Number { left, .. }, ValueType::Number) => *left = new_untyped_ref.typed(),
            (Comparison::Text { left, .. }, ValueType::Text) => *left = new_untyped_ref.typed(),
            (Comparison::Boolean { left, .. }, ValueType::Boolean) => {
                *left = new_untyped_ref.typed()
            }
            // Otherwise we have to change the type of the comparision
            (me @ _, ValueType::Number) => {
                *me = Comparison::Number {
                    left: new_untyped_ref.typed(),
                    comparator: NumberComparator::Equal,
                    right: Property::default(),
                }
            }
            (me @ _, ValueType::Text) => {
                *me = Comparison::Text {
                    left: new_untyped_ref.typed(),
                    comparator: TextComparator::Like,
                    right: Property::default(),
                }
            }
            (me @ _, ValueType::Boolean) => {
                *me = Comparison::Boolean {
                    left: new_untyped_ref.typed(),
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

struct ConditionProducer {
    comparison: Comparison,
    output: UntypedOutput,
}

impl ConditionProducer {
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
    fn resolve<T>(
        &self,
        output: &Output<T>,
        value_store: &ValueStore,
        entry: Option<&Entry>,
    ) -> Option<T>
    where
        ValueStore: ValueResolver<T>,
        T: Clone,
    {
        let condition = self.evaluate_condition(value_store, entry)?;

        if condition {
            value_store.get_property(&output.truee, entry)
        } else {
            value_store.get_property(&output.falsee, entry)
        }
    }
}

impl ValueProducer for ConditionProducer {
    fn get_number(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        let output = self.output.as_variant_ref()?;
        self.resolve(output, value_store, entry)
    }
    fn get_text(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        let output = self.output.as_variant_ref()?;
        self.resolve(output, value_store, entry)
    }
    fn get_boolean(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        let output = self.output.as_variant_ref()?;
        self.resolve(output, value_store, entry)
    }
    fn get_texture(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        let output = self.output.as_variant_ref()?;
        self.resolve(output, value_store, entry)
    }
    fn get_tint(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        let output = self.output.as_variant_ref()?;
        self.resolve(output, value_store, entry)
    }
}
