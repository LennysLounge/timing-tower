use std::marker::PhantomData;

use bevy::prelude::Color;
use bevy_egui::egui::{vec2, ComboBox, InnerResponse, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    reference_store::ReferenceStore,
    style::properties::{Property, PropertyEditor},
    value_store::{
        TypedValueProducer, TypedValueResolver, UntypedValueRef, ValueProducer, ValueRef,
        ValueStore,
    },
    value_types::{Boolean, Number, Text, Texture, Tint, ValueType},
};

use super::EguiComboBoxExtension;

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    output_type: ValueType,
    input: UntypedValueRef,
    cases: Vec<Case>,
    output_cases: UntypedOutput,
}

#[derive(Serialize, Deserialize, Clone)]
enum UntypedOutput {
    Number(Output2<Number>),
    Text(Output2<Text>),
    Tint(Output2<Tint>),
    Boolean(Output2<Boolean>),
    Texture(Output2<Texture>),
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Output2<T> {
    cases: Vec<Property<T>>,
    default: Property<T>,
}
impl<T> Output2<T>
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
}

impl Default for Map {
    fn default() -> Self {
        Self {
            output_type: ValueType::Number,
            input: UntypedValueRef::default(),
            cases: Vec::new(),
            output_cases: UntypedOutput::Number(Output2::default()),
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
            } = asset_repo.untyped_editor(ui, &self.input.id, |v| match v.asset_type {
                ValueType::Number => true,
                ValueType::Text => true,
                _ => false,
            });

            if let Some(new_untyped_ref) = new_untyped_ref {
                if self.input.value_type != new_untyped_ref.value_type {
                    let new_comparison = match self.input.value_type {
                        ValueType::Number => Comparison::Number(
                            Property::Fixed(Number(0.0)),
                            NumberComparator::Equal,
                        ),
                        ValueType::Text => Comparison::Text(
                            Property::Fixed(Text(String::new())),
                            TextComparator::Like,
                        ),
                        ValueType::Tint => unreachable!("Type Color not allowed in comparison"),
                        ValueType::Boolean => {
                            unreachable!("Type Boolean not allowed in comparison")
                        }
                        ValueType::Texture => unreachable!("Type Image not allowed in comparison"),
                    };
                    for case in self.cases.iter_mut() {
                        case.comparison = new_comparison.clone();
                    }
                }
                self.input = new_untyped_ref;
                changed |= true;
            };
        });
        ui.horizontal(|ui| {
            ui.label("to type: ");

            let res = ComboBox::from_id_source(ui.next_auto_id()).choose(
                ui,
                &mut self.output_type,
                vec![
                    (ValueType::Number, "Number"),
                    (ValueType::Text, "Text"),
                    (ValueType::Tint, "Color"),
                    (ValueType::Boolean, "Yes/No"),
                    (ValueType::Texture, "Image"),
                ],
            );

            changed |= res.changed();
            if res.changed() {
                println!("Update output types");
                let new_output = match self.output_type {
                    ValueType::Number => Output::Number(Property::Fixed(Number(0.0))),
                    ValueType::Text => Output::Text(Property::Fixed(Text(String::new()))),
                    ValueType::Tint => Output::Color(Property::Fixed(Tint(Color::WHITE))),
                    ValueType::Boolean => Output::Boolean(Property::Fixed(Boolean(false))),
                    ValueType::Texture => Output::Image(Property::Fixed(Texture::None)),
                };
                for case in self.cases.iter_mut() {
                    case.output = new_output.clone();
                }
                let case_count = self.cases.len();
                self.output_cases = match self.output_type {
                    ValueType::Number => UntypedOutput::Number(Output2::with_count(case_count)),
                    ValueType::Text => UntypedOutput::Text(Output2::with_count(case_count)),
                    ValueType::Tint => UntypedOutput::Tint(Output2::with_count(case_count)),
                    ValueType::Boolean => UntypedOutput::Boolean(Output2::with_count(case_count)),
                    ValueType::Texture => UntypedOutput::Texture(Output2::with_count(case_count)),
                };
            }
        });
        ui.separator();

        for case in self.cases.iter_mut() {
            changed |= case.show(ui, asset_repo);
        }

        self.cases.retain(|c| !c.remove);

        if ui.button("add case").clicked() {
            self.cases.push(self.new_case());
        }

        ui.label("Default:");
        ui.horizontal(|ui| {
            changed |= match &mut self.output_cases {
                UntypedOutput::Number(Output2 { default, .. }) => {
                    ui.add(PropertyEditor::new(default, asset_repo))
                }
                UntypedOutput::Text(Output2 { default, .. }) => {
                    ui.add(PropertyEditor::new(default, asset_repo))
                }
                UntypedOutput::Tint(Output2 { default, .. }) => {
                    ui.add(PropertyEditor::new(default, asset_repo))
                }
                UntypedOutput::Boolean(Output2 { default, .. }) => {
                    ui.add(PropertyEditor::new(default, asset_repo))
                }
                UntypedOutput::Texture(Output2 { default, .. }) => {
                    ui.add(PropertyEditor::new(default, asset_repo))
                }
            }
            .changed();
        });

        changed
    }

    fn new_case(&self) -> Case {
        Case {
            comparison: self.new_comparison(),
            output: self.new_output(),
            remove: false,
        }
    }

    fn new_comparison(&self) -> Comparison {
        match self.input.value_type {
            ValueType::Number => {
                Comparison::Number(Property::Fixed(Number(0.0)), NumberComparator::Equal)
            }
            ValueType::Text => {
                Comparison::Text(Property::Fixed(Text(String::new())), TextComparator::Like)
            }
            ValueType::Tint => unreachable!(),
            ValueType::Boolean => unreachable!(),
            ValueType::Texture => unreachable!(),
        }
    }

    fn new_output(&self) -> Output {
        match self.output_type {
            ValueType::Number => Output::Number(Property::Fixed(Number(0.0))),
            ValueType::Text => Output::Text(Property::Fixed(Text(String::new()))),
            ValueType::Tint => Output::Color(Property::Fixed(Tint(Color::WHITE))),
            ValueType::Boolean => Output::Boolean(Property::Fixed(Boolean(false))),
            ValueType::Texture => Output::Image(Property::Fixed(Texture::None)),
        }
    }
}

impl Map {
    pub fn output_type(&self) -> ValueType {
        self.output_type
    }

    pub fn as_typed_producer(&self) -> TypedValueProducer {
        let cases = self.generate_cases();
        match &self.output_cases {
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
        let mut cases = Vec::new();
        for case in self.cases.iter() {
            match self.input.value_type {
                ValueType::Number => {
                    let case_comp = match &case.comparison {
                        Comparison::Number(property, comp) => (
                            ValueRef {
                                id: self.input.id,
                                phantom: PhantomData,
                            },
                            comp.clone(),
                            property.clone(),
                        ),
                        _ => unreachable!(),
                    };
                    cases.push(CaseComparison::Number(case_comp));
                }
                ValueType::Text => {
                    let case_comp = match &case.comparison {
                        Comparison::Text(property, comp) => (
                            ValueRef {
                                id: self.input.id,
                                phantom: PhantomData,
                            },
                            comp.clone(),
                            property.clone(),
                        ),
                        _ => unreachable!(),
                    };
                    cases.push(CaseComparison::Text(case_comp));
                }
                _ => unreachable!(),
            };
        }
        cases
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Case {
    comparison: Comparison,
    output: Output,
    remove: bool,
}

impl Case {
    fn show(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            changed |= self.comparison.show(ui, asset_repo);
            ui.allocate_space(vec2(10.0, 0.0));
            ui.label("then");
            ui.allocate_space(vec2(10.0, 0.0));
            ui.vertical(|ui| {
                ui.label("output");
                ui.horizontal(|ui| {
                    changed |= self.output.show(ui, asset_repo);
                });
            });
        });
        if ui.small_button("remove").clicked() {
            self.remove = true;
        }
        ui.separator();

        changed
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum Comparison {
    Number(Property<Number>, NumberComparator),
    Text(Property<Text>, TextComparator),
}

impl Comparison {
    fn show(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.vertical(|ui| match self {
            Comparison::Number(np, c) => {
                ui.label("If input is");
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .selected_text(match c {
                        NumberComparator::Equal => "equal",
                        NumberComparator::Greater => "greater",
                        NumberComparator::GreaterEqual => "greater or equal",
                        NumberComparator::Less => "less",
                        NumberComparator::LessEqual => "less or equal",
                    })
                    .show_ui(ui, |ui| {
                        changed |= true;
                        ui.selectable_value(c, NumberComparator::Equal, "equal")
                            .changed();
                        changed |= true;
                        ui.selectable_value(c, NumberComparator::Greater, "greater")
                            .changed();
                        changed |= true;
                        ui.selectable_value(c, NumberComparator::GreaterEqual, "greater or equal")
                            .changed();
                        changed |= true;
                        ui.selectable_value(c, NumberComparator::Less, "less")
                            .changed();
                        changed |= true;
                        ui.selectable_value(c, NumberComparator::LessEqual, "less or equal")
                            .changed();
                    });
                ui.horizontal(|ui| {
                    changed |= ui.add(PropertyEditor::new(np, asset_repo)).changed()
                });
            }
            Comparison::Text(tp, c) => {
                ui.label("If input is");
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .selected_text(match c {
                        TextComparator::Like => "like",
                    })
                    .show_ui(ui, |ui| {
                        changed |= true;
                        ui.selectable_value(c, TextComparator::Like, "like")
                            .changed()
                    });
                ui.horizontal(|ui| {
                    changed |= ui.add(PropertyEditor::new(tp, asset_repo)).changed();
                });
            }
        });

        changed
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum NumberComparator {
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum TextComparator {
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
enum Output {
    Number(Property<Number>),
    Text(Property<Text>),
    Color(Property<Tint>),
    Boolean(Property<Boolean>),
    Image(Property<Texture>),
}

impl Output {
    fn show(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        changed |= match self {
            Output::Number(p) => ui.add(PropertyEditor::new(p, asset_repo)),
            Output::Text(p) => ui.add(PropertyEditor::new(p, asset_repo)),
            Output::Color(p) => ui.add(PropertyEditor::new(p, asset_repo)),
            Output::Boolean(p) => ui.add(PropertyEditor::new(p, asset_repo)),
            Output::Image(p) => ui.add(PropertyEditor::new(p, asset_repo)),
        }
        .changed();

        changed
    }
}

struct MapProducer<T> {
    cases: Vec<CaseComparison>,
    output: Output2<T>,
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

// impl ValueProducer<Number> for MapSource {
//     fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
//         self.cases
//             .iter()
//             .find_map(|(case, output)| {
//                 if case.test(value_store, entry) {
//                     match output {
//                         Output::Number(n) => value_store.get_property(n, entry),
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .or_else(|| match &self.default {
//                 Output::Number(p) => value_store.get_property(p, entry),
//                 _ => unreachable!(),
//             })
//     }
// }
// impl ValueProducer<Text> for MapSource {
//     fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
//         self.cases
//             .iter()
//             .find_map(|(case, output)| {
//                 if case.test(value_store, entry) {
//                     match output {
//                         Output::Text(n) => value_store.get_property(n, entry),
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .or_else(|| match &self.default {
//                 Output::Text(p) => value_store.get_property(p, entry),
//                 _ => unreachable!(),
//             })
//     }
// }
// impl ValueProducer<Tint> for MapSource {
//     fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
//         self.cases
//             .iter()
//             .find_map(|(case, output)| {
//                 if case.test(value_store, entry) {
//                     match output {
//                         Output::Color(n) => value_store.get_property(n, entry),
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .or_else(|| match &self.default {
//                 Output::Color(p) => value_store.get_property(p, entry),
//                 _ => unreachable!(),
//             })
//     }
// }
// impl ValueProducer<Boolean> for MapSource {
//     fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
//         self.cases
//             .iter()
//             .find_map(|(case, output)| {
//                 if case.test(value_store, entry) {
//                     match output {
//                         Output::Boolean(n) => value_store.get_property(n, entry),
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .or_else(|| match &self.default {
//                 Output::Boolean(p) => value_store.get_property(p, entry),
//                 _ => unreachable!(),
//             })
//     }
// }
// impl ValueProducer<Texture> for MapSource {
//     fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
//         self.cases
//             .iter()
//             .find_map(|(case, output)| {
//                 if case.test(value_store, entry) {
//                     match output {
//                         Output::Image(n) => value_store.get_property(n, entry),
//                         _ => unreachable!(),
//                     }
//                 } else {
//                     None
//                 }
//             })
//             .or_else(|| match &self.default {
//                 Output::Image(p) => value_store.get_property(p, entry),
//                 _ => unreachable!(),
//             })
//     }
// }

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
