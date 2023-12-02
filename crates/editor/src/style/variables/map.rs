use std::marker::PhantomData;

use bevy::prelude::Color;
use bevy_egui::egui::{vec2, ComboBox, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    reference_store::{ProducerData, ReferenceStore},
    style::properties::{Property, PropertyEditor},
    value_store::{TypedValueProducer, UntypedValueRef, ValueProducer, ValueRef, ValueStore},
    value_types::{Boolean, Number, Text, Texture, Tint, ValueType},
};

use super::variant_checkbox;

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    #[serde(flatten)]
    id: ProducerData,
    input: UntypedValueRef,
    cases: Vec<Case>,
    default: Output,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            id: ProducerData::default(),
            input: UntypedValueRef::default(),
            cases: Vec::new(),
            default: Output::Number(Property::Fixed(Number(0.0))),
        }
    }
}

impl Map {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Map input: ");
            let new_ref = asset_repo.untyped_editor(ui, &self.input.id, |v|
                match v.asset_type{
                    ValueType::Number => true,
                    ValueType::Text => true,
                    _ => false
                } &&
                v.id != self.id.id);
            if let Some(new_ref) = new_ref.inner {
                self.input = new_ref;
                changed |= true;
                let new_comparison = match self.input.value_type {
                    ValueType::Number => {
                        Comparison::Number(Property::Fixed(Number(0.0)), NumberComparator::Equal)
                    }
                    ValueType::Text => {
                        Comparison::Text(Property::Fixed(Text(String::new())), TextComparator::Like)
                    }
                    ValueType::Tint => unreachable!("Type Color not allowed in comparison"),
                    ValueType::Boolean => unreachable!("Type Boolean not allowed in comparison"),
                    ValueType::Texture => unreachable!("Type Image not allowed in comparison"),
                };
                for case in self.cases.iter_mut() {
                    case.comparison = new_comparison.clone();
                }
            };
        });
        ui.horizontal(|ui| {
            ui.label("to type: ");
            let output_type_before = self.id.asset_type;
            let res = variant_checkbox(
                ui,
                &mut self.id.asset_type,
                &[
                    (&ValueType::Number, "Number"),
                    (&ValueType::Text, "Text"),
                    (&ValueType::Tint, "Color"),
                    (&ValueType::Boolean, "Yes/No"),
                    (&ValueType::Texture, "Image"),
                ],
            );
            changed |= res.changed();
            if res.changed() && output_type_before != self.id.asset_type {
                println!("Update output types");
                let new_output = match self.id.asset_type {
                    ValueType::Number => Output::Number(Property::Fixed(Number(0.0))),
                    ValueType::Text => Output::Text(Property::Fixed(Text(String::new()))),
                    ValueType::Tint => Output::Color(Property::Fixed(Tint(Color::WHITE))),
                    ValueType::Boolean => Output::Boolean(Property::Fixed(Boolean(false))),
                    ValueType::Texture => Output::Image(Property::Fixed(Texture::None)),
                };
                self.default = new_output.clone();
                for case in self.cases.iter_mut() {
                    case.output = new_output.clone();
                }
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
            changed |= self.default.show(ui, asset_repo);
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
        match self.id.asset_type {
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
        self.id.asset_type
    }

    pub fn as_typed_producer(&self) -> TypedValueProducer {
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
                    cases.push((CaseComparison::Number(case_comp), case.output.clone()));
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
                    cases.push((CaseComparison::Text(case_comp), case.output.clone()));
                }
                _ => unreachable!(),
            };
        }

        let source = MapSource {
            cases,
            default: self.default.clone(),
        };

        match self.id.asset_type {
            ValueType::Number => TypedValueProducer::Number(Box::new(source)),
            ValueType::Text => TypedValueProducer::Text(Box::new(source)),
            ValueType::Tint => TypedValueProducer::Tint(Box::new(source)),
            ValueType::Boolean => TypedValueProducer::Boolean(Box::new(source)),
            ValueType::Texture => TypedValueProducer::Texture(Box::new(source)),
        }
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

struct MapSource {
    cases: Vec<(CaseComparison, Output)>,
    default: Output,
}
impl ValueProducer<Number> for MapSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        self.cases
            .iter()
            .find_map(|(case, output)| {
                if case.test(value_store, entry) {
                    match output {
                        Output::Number(n) => value_store.get_property(n, entry),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .or_else(|| match &self.default {
                Output::Number(p) => value_store.get_property(p, entry),
                _ => unreachable!(),
            })
    }
}
impl ValueProducer<Text> for MapSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        self.cases
            .iter()
            .find_map(|(case, output)| {
                if case.test(value_store, entry) {
                    match output {
                        Output::Text(n) => value_store.get_property(n, entry),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .or_else(|| match &self.default {
                Output::Text(p) => value_store.get_property(p, entry),
                _ => unreachable!(),
            })
    }
}
impl ValueProducer<Tint> for MapSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        self.cases
            .iter()
            .find_map(|(case, output)| {
                if case.test(value_store, entry) {
                    match output {
                        Output::Color(n) => value_store.get_property(n, entry),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .or_else(|| match &self.default {
                Output::Color(p) => value_store.get_property(p, entry),
                _ => unreachable!(),
            })
    }
}
impl ValueProducer<Boolean> for MapSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        self.cases
            .iter()
            .find_map(|(case, output)| {
                if case.test(value_store, entry) {
                    match output {
                        Output::Boolean(n) => value_store.get_property(n, entry),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .or_else(|| match &self.default {
                Output::Boolean(p) => value_store.get_property(p, entry),
                _ => unreachable!(),
            })
    }
}
impl ValueProducer<Texture> for MapSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        self.cases
            .iter()
            .find_map(|(case, output)| {
                if case.test(value_store, entry) {
                    match output {
                        Output::Image(n) => value_store.get_property(n, entry),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .or_else(|| match &self.default {
                Output::Image(p) => value_store.get_property(p, entry),
                _ => unreachable!(),
            })
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
