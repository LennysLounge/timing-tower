use bevy_egui::egui::{ComboBox, InnerResponse, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::Uuid;

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
pub struct Condition {
    #[serde(flatten)]
    comparison: Comparison,
    #[serde(flatten)]
    output: UntypedOutput,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            comparison: Comparison::Number(NumberComparison {
                left: ValueRef::default(),
                comparator: NumberComparator::Equal,
                right: Property::default(),
            }),
            output: UntypedOutput::Number(Output::default()),
        }
    }
}
impl Condition {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Output type:");
            changed |= ComboBox::from_id_source(ui.next_auto_id())
                .choose(
                    ui,
                    &mut self.output,
                    vec![
                        (UntypedOutput::Number(Output::default()), "Number"),
                        (UntypedOutput::Text(Output::default()), "Text"),
                        (UntypedOutput::Color(Output::default()), "Tint"),
                        (UntypedOutput::Boolean(Output::default()), "Boolean"),
                        (UntypedOutput::Image(Output::default()), "Texture"),
                    ],
                )
                .changed();
        });

        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

        ui.horizontal(|ui| {
            ui.label("If");
            ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());

            let InnerResponse {
                inner: new_untyped_ref,
                response: _,
            } = asset_repo.untyped_editor(ui, self.comparison.left_side_id(), |v| {
                return match v.value_type {
                    ValueType::Number => true,
                    ValueType::Text => true,
                    ValueType::Boolean => true,
                    _ => false,
                };
            });

            if let Some(reference) = new_untyped_ref {
                self.comparison.set_left_side(reference);
                changed |= true;
            }
            ui.label("is");
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.comparison {
                Comparison::Number(NumberComparison { comparator, .. }) => {
                    changed |= ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .choose(
                            ui,
                            comparator,
                            vec![
                                (NumberComparator::Equal, "equal"),
                                (NumberComparator::Greater, "greater"),
                                (NumberComparator::GreaterEqual, "greater or equal"),
                                (NumberComparator::Less, "less"),
                                (NumberComparator::LessEqual, "less or equal"),
                            ],
                        )
                        .changed();
                    match comparator {
                        NumberComparator::Equal => ui.label("to"),
                        NumberComparator::Greater => ui.label("than"),
                        NumberComparator::GreaterEqual => ui.label("to"),
                        NumberComparator::Less => ui.label("than"),
                        NumberComparator::LessEqual => ui.label("to"),
                    };
                }
                Comparison::Text(TextComparison { comparator: c, .. }) => {
                    changed |= ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .choose(ui, c, vec![(TextComparator::Like, "like")])
                        .changed();
                }
                Comparison::Boolean(BooleanComparison { comparator: c, .. }) => {
                    changed |= ComboBox::from_id_source(ui.next_auto_id())
                        .width(50.0)
                        .choose(
                            ui,
                            c,
                            vec![
                                (BooleanComparator::Is, "is"),
                                (BooleanComparator::IsNot, "is not"),
                            ],
                        )
                        .changed();
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
                UntypedOutput::Number(Output { truee, .. }) => {
                    ui.add(PropertyEditor::new(truee, asset_repo))
                }
                UntypedOutput::Text(Output { truee, .. }) => {
                    ui.add(PropertyEditor::new(truee, asset_repo))
                }
                UntypedOutput::Color(Output { truee, .. }) => {
                    ui.add(PropertyEditor::new(truee, asset_repo))
                }
                UntypedOutput::Boolean(Output { truee, .. }) => {
                    ui.add(PropertyEditor::new(truee, asset_repo))
                }
                UntypedOutput::Image(Output { truee, .. }) => {
                    ui.add(PropertyEditor::new(truee, asset_repo))
                }
            }
            .changed();
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.output {
                UntypedOutput::Number(output) => {
                    ui.add(PropertyEditor::new(&mut output.falsee, asset_repo))
                }
                UntypedOutput::Text(Output { falsee, .. }) => {
                    ui.add(PropertyEditor::new(falsee, asset_repo))
                }
                UntypedOutput::Color(Output { falsee, .. }) => {
                    ui.add(PropertyEditor::new(falsee, asset_repo))
                }
                UntypedOutput::Boolean(Output { falsee, .. }) => {
                    ui.add(PropertyEditor::new(falsee, asset_repo))
                }
                UntypedOutput::Image(Output { falsee, .. }) => {
                    ui.add(PropertyEditor::new(falsee, asset_repo))
                }
            }
            .changed();
        });
        changed
    }

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
    fn value_type(&self) -> ValueType {
        match self {
            Comparison::Number(_) => ValueType::Number,
            Comparison::Text(_) => ValueType::Text,
            Comparison::Boolean(_) => ValueType::Boolean,
        }
    }

    fn set_left_side(&mut self, new_untyped_ref: UntypedValueRef) {
        if self.value_type() == new_untyped_ref.value_type {
            // Update the left side if the types are the same.
            match self {
                Comparison::Number(number_comparison) => {
                    number_comparison.left = new_untyped_ref.typed()
                }
                Comparison::Boolean(boolean_comparison) => {
                    boolean_comparison.left = new_untyped_ref.typed()
                }
                Comparison::Text(text_comparison) => text_comparison.left = new_untyped_ref.typed(),
            }
        } else {
            // Otherwise change the type of the entire comparison.
            *self = match new_untyped_ref.value_type {
                ValueType::Number => Comparison::Number(NumberComparison {
                    left: new_untyped_ref.typed(),
                    comparator: NumberComparator::Equal,
                    right: Property::default(),
                }),
                ValueType::Text => Comparison::Text(TextComparison {
                    left: new_untyped_ref.typed(),
                    comparator: TextComparator::Like,
                    right: Property::default(),
                }),
                ValueType::Boolean => Comparison::Boolean(BooleanComparison {
                    left: new_untyped_ref.typed(),
                    comparator: BooleanComparator::Is,
                    right: Property::default(),
                }),
                ValueType::Tint => unreachable!("Type color not allowed for if condition"),
                ValueType::Texture => {
                    unreachable!("Type image not allowed for if condition")
                }
            }
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum NumberComparator {
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum TextComparator {
    Like,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum BooleanComparator {
    Is,
    IsNot,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "output_type")]
enum UntypedOutput {
    Number(Output<Number>),
    Text(Output<Text>),
    Color(Output<Tint>),
    Boolean(Output<Boolean>),
    Image(Output<Texture>),
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Output<T> {
    truee: Property<T>,
    falsee: Property<T>,
}

struct ConditionProducer<T> {
    comparison: Comparison,
    output: Output<T>,
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
            value_store.get_property(&self.output.truee, entry)
        } else {
            value_store.get_property(&self.output.falsee, entry)
        }
    }
}
