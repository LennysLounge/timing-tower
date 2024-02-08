use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    value_store::{UntypedValueProducer, ValueResolver, ValueProducer, ValueStore},
    value_types::{Boolean, Number, Property, Text, Texture, Tint, ValueRef, ValueType},
};

use super::{NumberComparator, TextComparator};

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    #[serde(flatten)]
    pub input: Input,
    #[serde(flatten)]
    pub output: UntypedOutput,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            input: Input::Number {
                input_ref: ValueRef::default(),
                input_cases: Vec::new(),
            },
            output: UntypedOutput::Number(Output::default()),
        }
    }
}

impl Map {
    pub fn output_type(&self) -> ValueType {
        match self.output {
            UntypedOutput::Number(_) => ValueType::Number,
            UntypedOutput::Text(_) => ValueType::Text,
            UntypedOutput::Tint(_) => ValueType::Tint,
            UntypedOutput::Boolean(_) => ValueType::Boolean,
            UntypedOutput::Texture(_) => ValueType::Texture,
        }
    }

    pub fn as_typed_producer(&self) -> UntypedValueProducer {
        let cases = self.generate_cases();
        match &self.output {
            UntypedOutput::Number(output) => UntypedValueProducer::Number(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Text(output) => UntypedValueProducer::Text(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Tint(output) => UntypedValueProducer::Tint(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Boolean(output) => UntypedValueProducer::Boolean(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Texture(output) => UntypedValueProducer::Texture(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
        }
    }

    fn generate_cases(&self) -> Vec<CaseComparison> {
        match &self.input {
            Input::Number {
                input_ref: input,
                input_cases: cases,
            } => cases
                .iter()
                .map(|c| {
                    CaseComparison::Number((input.clone(), c.comparator.clone(), c.right.clone()))
                })
                .collect(),
            Input::Text {
                input_ref: input,
                input_cases: cases,
            } => cases
                .iter()
                .map(|c| {
                    CaseComparison::Text((input.clone(), c.comparator.clone(), c.right.clone()))
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "input_type")]
pub enum Input {
    Number {
        input_ref: ValueRef<Number>,
        input_cases: Vec<NumberCase>,
    },
    Text {
        input_ref: ValueRef<Text>,
        input_cases: Vec<TextCase>,
    },
}

impl Input {
    pub fn value_type(&self) -> ValueType {
        match self {
            Input::Number { .. } => ValueType::Number,
            Input::Text { .. } => ValueType::Text,
        }
    }
    pub fn input_id(&self) -> Uuid {
        match self {
            Input::Number {
                input_ref: input, ..
            } => input.id,
            Input::Text {
                input_ref: input, ..
            } => input.id,
        }
    }
    pub fn case_count(&self) -> usize {
        match self {
            Input::Number {
                input_cases: cases, ..
            } => cases.len(),
            Input::Text {
                input_cases: cases, ..
            } => cases.len(),
        }
    }

    pub fn remove(&mut self, index: usize) {
        match self {
            Input::Number {
                input_cases: cases, ..
            } => _ = cases.remove(index),
            Input::Text {
                input_cases: cases, ..
            } => _ = cases.remove(index),
        }
    }

    pub fn push(&mut self) {
        match self {
            Input::Number {
                input_cases: cases, ..
            } => cases.push(NumberCase::default()),
            Input::Text {
                input_cases: cases, ..
            } => cases.push(TextCase::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NumberCase {
    pub right: Property<Number>,
    pub comparator: NumberComparator,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TextCase {
    pub right: Property<Text>,
    pub comparator: TextComparator,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
pub enum UntypedOutput {
    Number(Output<Number>),
    Text(Output<Text>),
    Tint(Output<Tint>),
    Boolean(Output<Boolean>),
    Texture(Output<Texture>),
}

impl UntypedOutput {
    pub fn case_count(&self) -> usize {
        match self {
            UntypedOutput::Number(o) => o.cases.len(),
            UntypedOutput::Text(o) => o.cases.len(),
            UntypedOutput::Tint(o) => o.cases.len(),
            UntypedOutput::Boolean(o) => o.cases.len(),
            UntypedOutput::Texture(o) => o.cases.len(),
        }
    }
    pub fn remove(&mut self, index: usize) {
        match self {
            UntypedOutput::Number(output) => _ = output.cases.remove(index),
            UntypedOutput::Text(output) => _ = output.cases.remove(index),
            UntypedOutput::Tint(output) => _ = output.cases.remove(index),
            UntypedOutput::Boolean(output) => _ = output.cases.remove(index),
            UntypedOutput::Texture(output) => _ = output.cases.remove(index),
        }
    }
    pub fn push(&mut self) {
        match self {
            UntypedOutput::Number(o) => o.cases.push(Property::default()),
            UntypedOutput::Text(o) => o.cases.push(Property::default()),
            UntypedOutput::Tint(o) => o.cases.push(Property::default()),
            UntypedOutput::Boolean(o) => o.cases.push(Property::default()),
            UntypedOutput::Texture(o) => o.cases.push(Property::default()),
        }
    }
    pub fn clear(&mut self) {
        match self {
            UntypedOutput::Number(o) => o.cases.clear(),
            UntypedOutput::Text(o) => o.cases.clear(),
            UntypedOutput::Tint(o) => o.cases.clear(),
            UntypedOutput::Boolean(o) => o.cases.clear(),
            UntypedOutput::Texture(o) => o.cases.clear(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Output<T> {
    #[serde(rename = "output_cases")]
    pub cases: Vec<Property<T>>,
    pub default: Property<T>,
}
impl<T> Output<T>
where
    Property<T>: Default,
{
    pub fn with_count(count: usize) -> Self {
        Self {
            cases: {
                let mut v = Vec::with_capacity(count);
                for _ in 0..count {
                    v.push(Property::default());
                }
                v
            },
            default: Property::default(),
        }
    }
}

struct MapProducer<T> {
    cases: Vec<CaseComparison>,
    output: Output<T>,
}

impl<T> ValueProducer<T> for MapProducer<T>
where
    ValueStore: ValueResolver<T>,
    T: Clone,
{
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<T> {
        let case_index = self
            .cases
            .iter()
            .enumerate()
            .find_map(|(index, case)| case.test(value_store, entry).then_some(index));

        if case_index.is_none() {
            return value_store.get_property(&self.output.default, entry);
        }

        let output_property = case_index
            .and_then(|index| self.output.cases.get(index))
            .expect("Index should be valid since cases and ouputs have the same length");

        value_store.get_property(output_property, entry)
    }
}

enum CaseComparison {
    Number((ValueRef<Number>, NumberComparator, Property<Number>)),
    Text((ValueRef<Text>, TextComparator, Property<Text>)),
}
impl CaseComparison {
    fn test(&self, asset_repo: &ValueStore, entry: Option<&Entry>) -> bool {
        match self {
            CaseComparison::Number((reference, comp, prop)) => {
                let value = asset_repo.get(reference, entry);
                let pivot = asset_repo.get_property(prop, entry);
                if let (Some(value), Some(pivot)) = (value, pivot) {
                    comp.compare(value.0, pivot.0)
                } else {
                    false
                }
            }
            CaseComparison::Text((reference, comp, prop)) => {
                let value = asset_repo.get(reference, entry);
                let pivot = asset_repo.get_property(prop, entry);
                if let (Some(value), Some(pivot)) = (value, pivot) {
                    comp.compare(&value.0, &pivot.0)
                } else {
                    false
                }
            }
        }
    }
}
