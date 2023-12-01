use bevy_egui::egui::{ComboBox, InnerResponse, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::{
    reference_store::ReferenceStore,
    style::properties::{Property, PropertyEditor},
    value_store::{TypedValueProducer, TypedValueResolver, ValueProducer, ValueRef, ValueStore},
    value_types::{Boolean, Number, Text, Texture, Tint, ValueType},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    comparison: Comparison,
    #[serde(flatten)]
    output: Output,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "comparison_type")]
enum Comparison {
    Number(NumberComparison),
    Text(TextComparison),
    Boolean(BooleanComparison),
}

impl Comparison {
    fn left_side_id(&self) -> &Uuid {
        match self {
            Comparison::Number(n) => &n.left.id,
            Comparison::Text(n) => &n.left.id,
            Comparison::Boolean(n) => &n.left.id,
        }
    }
    fn left_side_type(&self) -> ValueType {
        match self {
            Comparison::Number(_) => ValueType::Number,
            Comparison::Text(_) => ValueType::Text,
            Comparison::Boolean(_) => ValueType::Boolean,
        }
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum TextComparator {
    Like,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum BooleanComparator {
    Is,
    IsNot,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
enum Output {
    Number {
        truee: Property<Number>,
        falsee: Property<Number>,
    },
    Text {
        truee: Property<Text>,
        falsee: Property<Text>,
    },
    Color {
        truee: Property<Tint>,
        falsee: Property<Tint>,
    },
    Boolean {
        truee: Property<Boolean>,
        falsee: Property<Boolean>,
    },
    Image {
        truee: Property<Texture>,
        falsee: Property<Texture>,
    },
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            comparison: Comparison::Number(NumberComparison {
                left: ValueRef::default(),
                comparator: NumberComparator::Equal,
                right: Property::default(),
            }),
            output: Output::Number {
                truee: Property::Fixed(Number::default()),
                falsee: Property::Fixed(Number::default()),
            },
        }
    }
}
impl Condition {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Output type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.output_type() {
                    ValueType::Number => "Number",
                    ValueType::Text => "Text",
                    ValueType::Tint => "Color",
                    ValueType::Boolean => "Yes/No",
                    ValueType::Texture => "Image",
                })
                .show_ui(ui, |ui| {
                    let is_number = self.output_type() == ValueType::Number;
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.output = Output::Number {
                            truee: Property::default(),
                            falsee: Property::default(),
                        };
                        changed |= true;
                    }
                    let is_text = self.output_type() == ValueType::Text;
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.output = Output::Text {
                            truee: Property::default(),
                            falsee: Property::default(),
                        };
                        changed |= true;
                    }
                    let is_color = self.output_type() == ValueType::Tint;
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.output = Output::Color {
                            truee: Property::default(),
                            falsee: Property::default(),
                        };
                        changed |= true;
                    }
                    let is_boolean = self.output_type() == ValueType::Boolean;
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.output = Output::Boolean {
                            truee: Property::default(),
                            falsee: Property::default(),
                        };
                        changed |= true;
                    }
                    let is_image = self.output_type() == ValueType::Texture;
                    if ui.selectable_label(is_image, "Image").clicked() && !is_image {
                        self.output = Output::Image {
                            truee: Property::default(),
                            falsee: Property::default(),
                        };
                        changed |= true;
                    }
                });
        });
        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

        ui.horizontal(|ui| {
            ui.label("If");
            ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());

            let InnerResponse {
                inner: new_untyped_ref,
                response: _,
            } = asset_repo.untyped_editor(ui, self.comparison.left_side_id(), |v| {
                return match v.asset_type {
                    ValueType::Number => true,
                    ValueType::Text => true,
                    ValueType::Boolean => true,
                    _ => false,
                };
            });

            if let Some(reference) = new_untyped_ref {
                if self.comparison.left_side_type() != reference.value_type {
                    self.comparison = match reference.value_type {
                        ValueType::Number => Comparison::Number(NumberComparison {
                            left: reference.typed(),
                            comparator: NumberComparator::Equal,
                            right: Property::default(),
                        }),
                        ValueType::Text => Comparison::Text(TextComparison {
                            left: reference.typed(),
                            comparator: TextComparator::Like,
                            right: Property::default(),
                        }),
                        ValueType::Boolean => Comparison::Boolean(BooleanComparison {
                            left: reference.typed(),
                            comparator: BooleanComparator::Is,
                            right: Property::default(),
                        }),
                        ValueType::Tint => unreachable!("Type color not allowed for if condition"),
                        ValueType::Texture => {
                            unreachable!("Type image not allowed for if condition")
                        }
                    }
                }
                changed |= true;
            }
            ui.label("is");
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.comparison {
                Comparison::Number(NumberComparison { comparator, .. }) => {
                    ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .selected_text(match comparator {
                            NumberComparator::Equal => "equal",
                            NumberComparator::Greater => "greater",
                            NumberComparator::GreaterEqual => "greater or equal",
                            NumberComparator::Less => "less",
                            NumberComparator::LessEqual => "less or equal",
                        })
                        .show_ui(ui, |ui| {
                            changed |= true;
                            ui.selectable_value(comparator, NumberComparator::Equal, "equal")
                                .changed();
                            changed |= true;
                            ui.selectable_value(comparator, NumberComparator::Greater, "greater")
                                .changed();
                            changed |= true;
                            ui.selectable_value(
                                comparator,
                                NumberComparator::GreaterEqual,
                                "greater or equal",
                            )
                            .changed();
                            changed |= true;
                            ui.selectable_value(comparator, NumberComparator::Less, "less")
                                .changed();
                            changed |= true;
                            ui.selectable_value(
                                comparator,
                                NumberComparator::LessEqual,
                                "less or equal",
                            )
                            .changed();
                        });
                    match comparator {
                        NumberComparator::Equal => ui.label("to"),
                        NumberComparator::Greater => ui.label("than"),
                        NumberComparator::GreaterEqual => ui.label("to"),
                        NumberComparator::Less => ui.label("than"),
                        NumberComparator::LessEqual => ui.label("to"),
                    };
                }
                Comparison::Text(TextComparison { comparator: c, .. }) => {
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
                }
                Comparison::Boolean(BooleanComparison { comparator: c, .. }) => {
                    ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .selected_text(match c {
                            BooleanComparator::Is => "is",
                            BooleanComparator::IsNot => "is not",
                        })
                        .show_ui(ui, |ui| {
                            changed |= true;
                            ui.selectable_value(c, BooleanComparator::Is, "is")
                                .changed();
                            changed |= true;
                            ui.selectable_value(c, BooleanComparator::IsNot, "is not")
                                .changed();
                        });
                }
            }
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            // show select for right side
            changed |= ui
                .horizontal(|ui| match &mut self.comparison {
                    Comparison::Number(NumberComparison { right, .. }) => {
                        ui.add(PropertyEditor::new(right, asset_repo)).changed()
                    }
                    Comparison::Text(TextComparison { right, .. }) => {
                        ui.add(PropertyEditor::new(right, asset_repo)).changed()
                    }
                    Comparison::Boolean(BooleanComparison { right, .. }) => {
                        ui.add(PropertyEditor::new(right, asset_repo)).changed()
                    }
                })
                .inner;
        });
        ui.label("then:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.output {
                Output::Number { truee, .. } => ui.add(PropertyEditor::new(truee, asset_repo)),
                Output::Text { truee, .. } => ui.add(PropertyEditor::new(truee, asset_repo)),
                Output::Color { truee, .. } => ui.add(PropertyEditor::new(truee, asset_repo)),
                Output::Boolean { truee, .. } => ui.add(PropertyEditor::new(truee, asset_repo)),
                Output::Image { truee, .. } => ui.add(PropertyEditor::new(truee, asset_repo)),
            }
            .changed();
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.output {
                Output::Number { falsee, .. } => ui.add(PropertyEditor::new(falsee, asset_repo)),
                Output::Text { falsee, .. } => ui.add(PropertyEditor::new(falsee, asset_repo)),
                Output::Color { falsee, .. } => ui.add(PropertyEditor::new(falsee, asset_repo)),
                Output::Boolean { falsee, .. } => ui.add(PropertyEditor::new(falsee, asset_repo)),
                Output::Image { falsee, .. } => ui.add(PropertyEditor::new(falsee, asset_repo)),
            }
            .changed();
        });
        changed
    }
}

impl Condition {
    pub fn as_typed_producer(&self) -> TypedValueProducer {
        match &self.output {
            Output::Number { truee, falsee } => TypedValueProducer::Number(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    true_output: truee.clone(),
                    false_output: falsee.clone(),
                }
            })),
            Output::Text { truee, falsee } => TypedValueProducer::Text(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    true_output: truee.clone(),
                    false_output: falsee.clone(),
                }
            })),
            Output::Color { truee, falsee } => TypedValueProducer::Tint(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    true_output: truee.clone(),
                    false_output: falsee.clone(),
                }
            })),
            Output::Boolean { truee, falsee } => TypedValueProducer::Boolean(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    true_output: truee.clone(),
                    false_output: falsee.clone(),
                }
            })),
            Output::Image { truee, falsee } => TypedValueProducer::Texture(Box::new({
                ConditionProducer {
                    comparison: self.comparison.clone(),
                    true_output: truee.clone(),
                    false_output: falsee.clone(),
                }
            })),
        }
    }
}
impl Condition {
    pub fn output_type(&self) -> ValueType {
        match self.output {
            Output::Number { .. } => ValueType::Number,
            Output::Text { .. } => ValueType::Text,
            Output::Color { .. } => ValueType::Tint,
            Output::Boolean { .. } => ValueType::Boolean,
            Output::Image { .. } => ValueType::Texture,
        }
    }
}

struct ConditionProducer<T> {
    comparison: Comparison,
    true_output: Property<T>,
    false_output: Property<T>,
}

impl<T> ConditionProducer<T> {
    fn evaluate_condition(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        match &self.comparison {
            Comparison::Number(n) => n.evaluate(vars, entry),
            Comparison::Text(t) => t.evaluate(vars, entry),
            Comparison::Boolean(b) => b.evaluate(vars, entry),
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
            value_store.get_property(&self.true_output, entry)
        } else {
            value_store.get_property(&self.false_output, entry)
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct NumberComparison {
    left: ValueRef<Number>,
    comparator: NumberComparator,
    right: Property<Number>,
}

impl NumberComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get(&self.left, entry)?.0;
        let right = vars.get_property(&self.right, entry)?.0;
        Some(match self.comparator {
            NumberComparator::Equal => left == right,
            NumberComparator::Greater => left > right,
            NumberComparator::GreaterEqual => left >= right,
            NumberComparator::Less => left < right,
            NumberComparator::LessEqual => left <= right,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct TextComparison {
    left: ValueRef<Text>,
    comparator: TextComparator,
    right: Property<Text>,
}

impl TextComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get(&self.left, entry)?.0;
        let right = vars.get_property(&self.right, entry)?.0;
        Some(match self.comparator {
            TextComparator::Like => left == right,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct BooleanComparison {
    left: ValueRef<Boolean>,
    comparator: BooleanComparator,
    right: Property<Boolean>,
}
impl BooleanComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get(&self.left, entry)?.0;
        let right = vars.get_property(&self.right, entry)?.0;
        Some(match self.comparator {
            BooleanComparator::Is => left == right,
            BooleanComparator::IsNot => left != right,
        })
    }
}
