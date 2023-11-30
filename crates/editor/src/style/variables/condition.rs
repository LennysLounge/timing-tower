use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    reference_store::ReferenceStore,
    style::properties::{Property, PropertyEditor},
    value_store::{
        types::{Boolean, Number, Text, Texture, Tint},
        AssetId, IntoValueProducer, TypedValueProducer, UntypedValueRef, ValueProducer, ValueStore,
        ValueType,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    id: AssetId,
    left: UntypedValueRef,
    right: RightHandSide,
    true_output: Output,
    false_output: Output,
}

#[derive(Serialize, Deserialize, Clone)]
enum RightHandSide {
    Number(Property<Number>, NumberComparator),
    Text(Property<Text>, TextComparator),
    Boolean(Property<Boolean>, BooleanComparator),
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
enum Output {
    Number(Property<Number>),
    Text(Property<Text>),
    Color(Property<Tint>),
    Boolean(Property<Boolean>),
    Image(Property<Texture>),
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            id: Default::default(),
            left: Default::default(),
            right: RightHandSide::Number(Property::Fixed(Number(0.0)), NumberComparator::Equal),
            true_output: Output::Number(Property::Fixed(Number(0.0))),
            false_output: Output::Number(Property::Fixed(Number(0.0))),
        }
    }
}

impl IntoValueProducer for Condition {
    fn get_value_producer(&self) -> TypedValueProducer {
        let source = ConditionSource {
            comparison: match &self.right {
                RightHandSide::Number(np, c) => Comparison::Number(NumberComparison {
                    left: self.left.clone(),
                    comparator: c.clone(),
                    right: np.clone(),
                }),
                RightHandSide::Text(tp, c) => Comparison::Text(TextComparison {
                    left: self.left.clone(),
                    comparator: c.clone(),
                    right: tp.clone(),
                }),
                RightHandSide::Boolean(bp, c) => Comparison::Boolean(BooleanComparison {
                    left: self.left.clone(),
                    comparator: c.clone(),
                    right: bp.clone(),
                }),
            },
            true_value: self.true_output.clone(),
            false_value: self.false_output.clone(),
        };

        match self.id.asset_type {
            ValueType::Number => TypedValueProducer::Number(Box::new(source)),
            ValueType::Text => TypedValueProducer::Text(Box::new(source)),
            ValueType::Tint => TypedValueProducer::Tint(Box::new(source)),
            ValueType::Boolean => TypedValueProducer::Boolean(Box::new(source)),
            ValueType::Texture => TypedValueProducer::Texture(Box::new(source)),
        }
    }

    fn asset_id(&self) -> &AssetId {
        &self.id
    }
}

impl Condition {
    pub fn from_id(id: AssetId) -> Self {
        Self {
            true_output: match &id.asset_type {
                ValueType::Number => Output::Number(Property::Fixed(Number(0.0))),
                ValueType::Text => Output::Text(Property::Fixed(Text(String::new()))),
                ValueType::Tint => Output::Color(Property::Fixed(Tint(Color::WHITE))),
                ValueType::Boolean => Output::Boolean(Property::Fixed(Boolean(true))),
                ValueType::Texture => Output::Image(Property::Fixed(Texture::None)),
            },
            false_output: match &id.asset_type {
                ValueType::Number => Output::Number(Property::Fixed(Number(0.0))),
                ValueType::Text => Output::Text(Property::Fixed(Text(String::new()))),
                ValueType::Tint => Output::Color(Property::Fixed(Tint(Color::WHITE))),
                ValueType::Boolean => Output::Boolean(Property::Fixed(Boolean(false))),
                ValueType::Texture => Output::Image(Property::Fixed(Texture::None)),
            },
            id,
            ..Default::default()
        }
    }
    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Output type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.id.asset_type {
                    ValueType::Number => "Number",
                    ValueType::Text => "Text",
                    ValueType::Tint => "Color",
                    ValueType::Boolean => "Yes/No",
                    ValueType::Texture => "Image",
                })
                .show_ui(ui, |ui| {
                    let is_number = self.id.asset_type == ValueType::Number;
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.id.asset_type = ValueType::Number;
                        self.true_output = Output::Number(Property::Fixed(Number(0.0)));
                        self.false_output = Output::Number(Property::Fixed(Number(0.0)));
                        changed |= true;
                    }
                    let is_text = self.id.asset_type == ValueType::Text;
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.id.asset_type = ValueType::Text;
                        self.true_output = Output::Text(Property::Fixed(Text(String::new())));
                        self.false_output = Output::Text(Property::Fixed(Text(String::new())));
                        changed |= true;
                    }
                    let is_color = self.id.asset_type == ValueType::Tint;
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.id.asset_type = ValueType::Tint;
                        self.true_output = Output::Color(Property::Fixed(Tint(Color::WHITE)));
                        self.false_output = Output::Color(Property::Fixed(Tint(Color::WHITE)));
                        changed |= true;
                    }
                    let is_boolean = self.id.asset_type == ValueType::Boolean;
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.id.asset_type = ValueType::Boolean;
                        self.true_output = Output::Boolean(Property::Fixed(Boolean(true)));
                        self.false_output = Output::Boolean(Property::Fixed(Boolean(false)));
                        changed |= true;
                    }
                    let is_image = self.id.asset_type == ValueType::Texture;
                    if ui.selectable_label(is_image, "Image").clicked() && !is_image {
                        self.id.asset_type = ValueType::Texture;
                        self.true_output = Output::Image(Property::Fixed(Texture::None));
                        self.false_output = Output::Image(Property::Fixed(Texture::None));
                        changed |= true;
                    }
                });
        });
        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

        ui.horizontal(|ui| {
            ui.label("If");
            ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());
            let new_ref = asset_repo.untyped_editor(ui, &mut self.left.id, |v| {
                return match v.asset_type {
                    ValueType::Number => true,
                    ValueType::Text => true,
                    ValueType::Boolean => true,
                    _ => false,
                } && v.id != self.id.id;
            });
            if let Some(reference) = new_ref.inner {
                // Channge the value type of the right side if necessary
                if self.left.value_type != reference.value_type {
                    self.right = match reference.value_type {
                        ValueType::Number => RightHandSide::Number(
                            Property::Fixed(Number(0.0)),
                            NumberComparator::Equal,
                        ),
                        ValueType::Text => RightHandSide::Text(
                            Property::Fixed(Text(String::new())),
                            TextComparator::Like,
                        ),
                        ValueType::Boolean => RightHandSide::Boolean(
                            Property::Fixed(Boolean(true)),
                            BooleanComparator::Is,
                        ),
                        ValueType::Tint => unreachable!("Type color not allowed for if condition"),
                        ValueType::Texture => {
                            unreachable!("Type image not allowed for if condition")
                        }
                    }
                }
                self.left = reference;
                changed |= true;
            }
            ui.label("is");
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.right {
                RightHandSide::Number(_, c) => {
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
                            ui.selectable_value(
                                c,
                                NumberComparator::GreaterEqual,
                                "greater or equal",
                            )
                            .changed();
                            changed |= true;
                            ui.selectable_value(c, NumberComparator::Less, "less")
                                .changed();
                            changed |= true;
                            ui.selectable_value(c, NumberComparator::LessEqual, "less or equal")
                                .changed();
                        });
                    match c {
                        NumberComparator::Equal => ui.label("to"),
                        NumberComparator::Greater => ui.label("than"),
                        NumberComparator::GreaterEqual => ui.label("to"),
                        NumberComparator::Less => ui.label("than"),
                        NumberComparator::LessEqual => ui.label("to"),
                    };
                }
                RightHandSide::Text(_, c) => {
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
                RightHandSide::Boolean(_, c) => {
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
                .horizontal(|ui| match &mut self.right {
                    RightHandSide::Number(n, _) => {
                        ui.add(PropertyEditor::new(n, asset_repo)).changed()
                    }
                    RightHandSide::Text(t, _) => {
                        ui.add(PropertyEditor::new(t, asset_repo)).changed()
                    }
                    RightHandSide::Boolean(b, _) => b.editor(ui, asset_repo),
                })
                .inner;
        });
        ui.label("then:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.true_output {
                Output::Number(n) => ui.add(PropertyEditor::new(n, asset_repo)).changed(),
                Output::Text(t) => ui.add(PropertyEditor::new(t, asset_repo)).changed(),
                Output::Color(c) => c.editor(ui, asset_repo),
                Output::Boolean(b) => b.editor(ui, asset_repo),
                Output::Image(i) => i.editor(ui, asset_repo),
            };
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.false_output {
                Output::Number(n) => ui.add(PropertyEditor::new(n, asset_repo)).changed(),
                Output::Text(t) => ui.add(PropertyEditor::new(t, asset_repo)).changed(),
                Output::Color(c) => c.editor(ui, asset_repo),
                Output::Boolean(b) => b.editor(ui, asset_repo),
                Output::Image(i) => i.editor(ui, asset_repo),
            };
        });
        changed
    }
}

struct ConditionSource {
    comparison: Comparison,
    true_value: Output,
    false_value: Output,
}

impl ConditionSource {
    fn evaluate_condition(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        match &self.comparison {
            Comparison::Number(n) => n.evaluate(vars, entry),
            Comparison::Text(t) => t.evaluate(vars, entry),
            Comparison::Boolean(b) => b.evaluate(vars, entry),
        }
    }
}

impl ValueProducer<Number> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Number(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Number(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Text> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Text(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Text(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Tint> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Color(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Color(n) => value_store.get_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Boolean> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Boolean(b) => value_store.get_property(&b, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Boolean(b) => value_store.get_property(&b, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Texture> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Image(i) => value_store.get_property(&i, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Image(i) => value_store.get_property(&i, entry),
                _ => unreachable!(),
            }
        }
    }
}

enum Comparison {
    Number(NumberComparison),
    Text(TextComparison),
    Boolean(BooleanComparison),
}

struct NumberComparison {
    left: UntypedValueRef,
    comparator: NumberComparator,
    right: Property<Number>,
}

impl NumberComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_number(&self.left, entry)?;
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

struct TextComparison {
    left: UntypedValueRef,
    comparator: TextComparator,
    right: Property<Text>,
}

impl TextComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_text(&self.left, entry)?;
        let right = vars.get_property(&self.right, entry)?;
        Some(match self.comparator {
            TextComparator::Like => left == right.0,
        })
    }
}

struct BooleanComparison {
    left: UntypedValueRef,
    comparator: BooleanComparator,
    right: Property<Boolean>,
}
impl BooleanComparison {
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_bool(&self.left, entry)?;
        let right = vars.get_property(&self.right, entry)?.0;
        Some(match self.comparator {
            BooleanComparator::Is => left == right,
            BooleanComparator::IsNot => left != right,
        })
    }
}
