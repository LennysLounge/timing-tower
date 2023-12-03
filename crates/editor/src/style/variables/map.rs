use bevy_egui::egui::{vec2, ComboBox, InnerResponse, Response, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    reference_store::ReferenceStore,
    style::properties::{PropertyEditor, ValueTypeEditor},
    value_store::{TypedValueProducer, TypedValueResolver, ValueProducer, ValueStore},
};
use backend::value_types::{
    Boolean, Number, Property, Text, Texture, Tint, ValueRef, ValueType, ValueTypeOf,
};

use super::EguiComboBoxExtension;

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    #[serde(flatten)]
    input: Input,
    #[serde(flatten)]
    output: UntypedOutput,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            input: Input::Number {
                input: ValueRef::default(),
                cases: Vec::new(),
            },
            output: UntypedOutput::Number(Output::default()),
        }
    }
}

impl Map {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Map input: ");

            let InnerResponse {
                inner: new_untyped_ref,
                response: _,
            } = asset_repo.untyped_editor(ui, &self.input.input_id(), |v| match v.value_type {
                ValueType::Number => true,
                ValueType::Text => true,
                _ => false,
            });

            if let Some(new_untyped_ref) = new_untyped_ref {
                // Only update the actual input reference
                if new_untyped_ref.value_type == self.input.value_type() {
                    match &mut self.input {
                        Input::Number { input, .. } => *input = new_untyped_ref.typed(),
                        Input::Text { input, .. } => *input = new_untyped_ref.typed(),
                    }
                } else {
                    // Change the entire type of the input to match the new reference.
                    self.input = match new_untyped_ref.value_type {
                        ValueType::Number => Input::Number {
                            input: new_untyped_ref.typed(),
                            cases: Vec::new(),
                        },
                        ValueType::Text => Input::Text {
                            input: new_untyped_ref.typed(),
                            cases: Vec::new(),
                        },
                        ValueType::Tint => unreachable!("Type Color not allowed in comparison"),
                        ValueType::Boolean => {
                            unreachable!("Type Boolean not allowed in comparison")
                        }
                        ValueType::Texture => unreachable!("Type Image not allowed in comparison"),
                    };
                    self.output.clear();
                }
                changed |= true;
            };
        });
        ui.horizontal(|ui| {
            ui.label("to type: ");

            let count = self.input.case_count();
            changed |= ComboBox::from_id_source(ui.next_auto_id())
                .choose(
                    ui,
                    &mut self.output,
                    vec![
                        (UntypedOutput::Number(Output::with_count(count)), "Number"),
                        (UntypedOutput::Text(Output::with_count(count)), "Text"),
                        (UntypedOutput::Tint(Output::with_count(count)), "Color"),
                        (UntypedOutput::Boolean(Output::with_count(count)), "Yes/No"),
                        (UntypedOutput::Texture(Output::with_count(count)), "Image"),
                    ],
                )
                .changed();
        });
        ui.separator();

        let mut remove_case = None;
        for index in 0..self.input.case_count() {
            ui.horizontal(|ui| {
                changed |= self.input.edit_case(ui, asset_repo, index);
                ui.allocate_space(vec2(10.0, 0.0));
                ui.label("then");
                ui.allocate_space(vec2(10.0, 0.0));
                ui.horizontal(|ui| {
                    changed |= self.output.edit_case(ui, asset_repo, index).changed();
                });
            });
            if ui.small_button("remove").clicked() {
                remove_case = Some(index);
            }
            ui.separator();
        }

        if let Some(index) = remove_case {
            self.input.remove(index);
            self.output.remove(index);
        }

        ui.horizontal(|ui| {
            ui.label("Default:");
            changed |= self.output.edit_default(ui, asset_repo).changed();
        });
        ui.separator();

        if ui.button("add case").clicked() {
            self.input.push();
            self.output.push();
        }

        if self.input.case_count() != self.output.case_count() {
            panic!(
                "Case counts in map are different. This should never happen. inputs: {}, outputs:{}",
                self.input.case_count(),
                self.output.case_count()
            );
        }

        changed
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

    pub fn as_typed_producer(&self) -> TypedValueProducer {
        let cases = self.generate_cases();
        match &self.output {
            UntypedOutput::Number(output) => TypedValueProducer::Number(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Text(output) => TypedValueProducer::Text(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Tint(output) => TypedValueProducer::Tint(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Boolean(output) => TypedValueProducer::Boolean(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
            UntypedOutput::Texture(output) => TypedValueProducer::Texture(Box::new(MapProducer {
                cases,
                output: output.clone(),
            })),
        }
    }

    fn generate_cases(&self) -> Vec<CaseComparison> {
        match &self.input {
            Input::Number { input, cases } => cases
                .iter()
                .map(|c| {
                    CaseComparison::Number((input.clone(), c.comparator.clone(), c.right.clone()))
                })
                .collect(),
            Input::Text { input, cases } => cases
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
enum Input {
    Number {
        #[serde(rename = "input_ref")]
        input: ValueRef<Number>,
        #[serde(rename = "input_cases")]
        cases: Vec<NumberCase>,
    },
    Text {
        #[serde(rename = "input_ref")]
        input: ValueRef<Text>,
        #[serde(rename = "input_cases")]
        cases: Vec<TextCase>,
    },
}

impl Input {
    fn value_type(&self) -> ValueType {
        match self {
            Input::Number { .. } => ValueType::Number,
            Input::Text { .. } => ValueType::Text,
        }
    }
    fn input_id(&self) -> Uuid {
        match self {
            Input::Number { input, .. } => input.id,
            Input::Text { input, .. } => input.id,
        }
    }
    fn case_count(&self) -> usize {
        match self {
            Input::Number { cases, .. } => cases.len(),
            Input::Text { cases, .. } => cases.len(),
        }
    }

    fn remove(&mut self, index: usize) {
        match self {
            Input::Number { cases, .. } => _ = cases.remove(index),
            Input::Text { cases, .. } => _ = cases.remove(index),
        }
    }

    fn push(&mut self) {
        match self {
            Input::Number { cases, .. } => cases.push(NumberCase::default()),
            Input::Text { cases, .. } => cases.push(TextCase::default()),
        }
    }

    fn edit_case(&mut self, ui: &mut Ui, reference_store: &ReferenceStore, index: usize) -> bool {
        let mut changed = false;
        match self {
            Input::Number { cases, .. } => {
                let case = cases.get_mut(index).expect("the case index must be valid");
                ui.label("If input is");
                changed |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(
                        ui,
                        &mut case.comparator,
                        vec![
                            (NumberComparator::Equal, "equal"),
                            (NumberComparator::Greater, "greater"),
                            (NumberComparator::GreaterEqual, "greater or equal"),
                            (NumberComparator::Less, "less"),
                            (NumberComparator::LessEqual, "less or equal"),
                        ],
                    )
                    .changed();
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(PropertyEditor::new(&mut case.right, reference_store))
                        .changed()
                });
            }
            Input::Text { cases, .. } => {
                let case = cases.get_mut(index).expect("the case index must be valid");
                ui.label("If input is");
                changed |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(
                        ui,
                        &mut case.comparator,
                        vec![(TextComparator::Like, "like")],
                    )
                    .changed();
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(PropertyEditor::new(&mut case.right, reference_store))
                        .changed();
                });
            }
        }

        changed
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NumberCase {
    right: Property<Number>,
    comparator: NumberComparator,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
enum NumberComparator {
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

#[derive(Serialize, Deserialize, Clone, Default)]
struct TextCase {
    right: Property<Text>,
    comparator: TextComparator,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
enum TextComparator {
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
enum UntypedOutput {
    Number(Output<Number>),
    Text(Output<Text>),
    Tint(Output<Tint>),
    Boolean(Output<Boolean>),
    Texture(Output<Texture>),
}

impl UntypedOutput {
    fn case_count(&self) -> usize {
        match self {
            UntypedOutput::Number(o) => o.cases.len(),
            UntypedOutput::Text(o) => o.cases.len(),
            UntypedOutput::Tint(o) => o.cases.len(),
            UntypedOutput::Boolean(o) => o.cases.len(),
            UntypedOutput::Texture(o) => o.cases.len(),
        }
    }
    fn remove(&mut self, index: usize) {
        match self {
            UntypedOutput::Number(output) => _ = output.cases.remove(index),
            UntypedOutput::Text(output) => _ = output.cases.remove(index),
            UntypedOutput::Tint(output) => _ = output.cases.remove(index),
            UntypedOutput::Boolean(output) => _ = output.cases.remove(index),
            UntypedOutput::Texture(output) => _ = output.cases.remove(index),
        }
    }
    fn push(&mut self) {
        match self {
            UntypedOutput::Number(o) => o.cases.push(Property::default()),
            UntypedOutput::Text(o) => o.cases.push(Property::default()),
            UntypedOutput::Tint(o) => o.cases.push(Property::default()),
            UntypedOutput::Boolean(o) => o.cases.push(Property::default()),
            UntypedOutput::Texture(o) => o.cases.push(Property::default()),
        }
    }
    fn clear(&mut self) {
        match self {
            UntypedOutput::Number(o) => o.cases.clear(),
            UntypedOutput::Text(o) => o.cases.clear(),
            UntypedOutput::Tint(o) => o.cases.clear(),
            UntypedOutput::Boolean(o) => o.cases.clear(),
            UntypedOutput::Texture(o) => o.cases.clear(),
        }
    }
    fn edit_case(
        &mut self,
        ui: &mut Ui,
        reference_store: &ReferenceStore,
        index: usize,
    ) -> Response {
        match self {
            UntypedOutput::Number(output) => output.edit_case(ui, reference_store, index),
            UntypedOutput::Text(output) => output.edit_case(ui, reference_store, index),
            UntypedOutput::Tint(output) => output.edit_case(ui, reference_store, index),
            UntypedOutput::Boolean(output) => output.edit_case(ui, reference_store, index),
            UntypedOutput::Texture(output) => output.edit_case(ui, reference_store, index),
        }
    }

    fn edit_default(&mut self, ui: &mut Ui, reference_store: &ReferenceStore) -> Response {
        match self {
            UntypedOutput::Number(Output { default, .. }) => {
                ui.add(PropertyEditor::new(default, reference_store))
            }
            UntypedOutput::Text(Output { default, .. }) => {
                ui.add(PropertyEditor::new(default, reference_store))
            }
            UntypedOutput::Tint(Output { default, .. }) => {
                ui.add(PropertyEditor::new(default, reference_store))
            }
            UntypedOutput::Boolean(Output { default, .. }) => {
                ui.add(PropertyEditor::new(default, reference_store))
            }
            UntypedOutput::Texture(Output { default, .. }) => {
                ui.add(PropertyEditor::new(default, reference_store))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Output<T> {
    #[serde(rename = "output_cases")]
    cases: Vec<Property<T>>,
    default: Property<T>,
}
impl<T> Output<T>
where
    Property<T>: Default,
{
    fn with_count(count: usize) -> Self {
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

    fn edit_case(&mut self, ui: &mut Ui, reference_store: &ReferenceStore, index: usize) -> Response
    where
        ValueType: ValueTypeOf<T>,
        T: Default + ValueTypeEditor,
    {
        let property = self
            .cases
            .get_mut(index)
            .expect("the case index must be valid");
        ui.add(PropertyEditor::new(property, &reference_store))
    }
}

struct MapProducer<T> {
    cases: Vec<CaseComparison>,
    output: Output<T>,
}

impl<T> ValueProducer<T> for MapProducer<T>
where
    ValueStore: TypedValueResolver<T>,
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
