use bevy::prelude::Color;
use bevy_egui::egui::{vec2, ComboBox, Ui};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    reference_store::ReferenceStore,
    style::properties::Property,
    value_store::{
        types::{Boolean, Number, Text, Texture, Tint},
        AssetId, IntoValueProducer, TypedValueProducer, UntypedValueRef, ValueProducer, ValueStore,
        ValueType,
    },
};

use super::variant_checkbox;

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    #[serde(flatten)]
    id: AssetId,
    input: UntypedValueRef,
    cases: Vec<Case>,
    default: Output,
}

impl Map {
    pub fn from_id(id: AssetId) -> Self {
        Self {
            id,
            input: UntypedValueRef::default(),
            cases: Vec::new(),
            default: Output::Number(Property::Fixed(Number(0.0))),
        }
    }
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
                self.update_comparison_type();
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
                self.update_output_type();
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

    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

    fn update_output_type(&mut self) {
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

    fn update_comparison_type(&mut self) {
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
    fn new_case(&self) -> Case {
        Case {
            comparison: self.new_comparison(),
            output: self.new_output(),
            remove: false,
        }
    }
}

impl IntoValueProducer for Map {
    fn asset_id(&self) -> &AssetId {
        &self.id
    }

    fn get_value_producer(&self) -> TypedValueProducer {
        let mut cases = Vec::new();
        for case in self.cases.iter() {
            match self.input.value_type {
                ValueType::Number => {
                    let case_comp = match &case.comparison {
                        Comparison::Number(property, comp) => {
                            (self.input.clone(), comp.clone(), property.clone())
                        }
                        _ => unreachable!(),
                    };
                    cases.push((CaseComparison::Number(case_comp), case.output.clone()));
                }
                ValueType::Text => {
                    let case_comp = match &case.comparison {
                        Comparison::Text(property, comp) => {
                            (self.input.clone(), comp.clone(), property.clone())
                        }
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
                    changed |= np.editor(ui, asset_repo);
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
                    changed |= tp.editor(ui, asset_repo);
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
            Output::Number(p) => p.editor(ui, asset_repo),
            Output::Text(p) => p.editor(ui, asset_repo),
            Output::Color(p) => p.editor(ui, asset_repo),
            Output::Boolean(p) => p.editor(ui, asset_repo),
            Output::Image(p) => p.editor(ui, asset_repo),
        };

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
    Number((UntypedValueRef, NumberComparator, Property<Number>)),
    Text((UntypedValueRef, TextComparator, Property<Text>)),
}
impl CaseComparison {
    fn test(&self, asset_repo: &ValueStore, entry: Option<&Entry>) -> bool {
        match self {
            CaseComparison::Number((reference, comp, prop)) => {
                let value = asset_repo.get_number(reference, entry);
                let pivot = asset_repo.get_property(prop, entry);
                if let (Some(value), Some(pivot)) = (value, pivot) {
                    comp.compare(value, pivot.0)
                } else {
                    false
                }
            }
            CaseComparison::Text((reference, comp, prop)) => {
                let value = asset_repo.get_text(reference, entry);
                let pivot = asset_repo.get_property(prop, entry);
                if let (Some(value), Some(pivot)) = (value, pivot) {
                    comp.compare(&value, &pivot.0)
                } else {
                    false
                }
            }
        }
    }
}
