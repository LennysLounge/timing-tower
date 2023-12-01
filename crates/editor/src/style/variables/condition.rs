use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    reference_store::ReferenceStore,
    style::properties::{Property, PropertyEditor},
    value_store::{TypedValueProducer, UntypedValueRef, ValueProducer, ValueRef, ValueStore},
    value_types::{Boolean, Number, Text, Texture, Tint, ValueType},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    left: UntypedValueRef,
    right: RightHandSide,
    output: Output,
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
            left: Default::default(),
            right: RightHandSide::Number(Property::Fixed(Number(0.0)), NumberComparator::Equal),
            output: Output::Number {
                truee: Property::Fixed(Number::default()),
                falsee: Property::Fixed(Number::default()),
            },
        }
    }
}

impl Condition {
    pub fn as_typed_producer(&self) -> TypedValueProducer {
        let source = ConditionSource {
            comparison: match &self.right {
                RightHandSide::Number(np, c) => Comparison::Number(NumberComparison {
                    left: ValueRef {
                        id: self.left.id,
                        phantom: std::marker::PhantomData,
                    },
                    comparator: c.clone(),
                    right: np.clone(),
                }),
                RightHandSide::Text(tp, c) => Comparison::Text(TextComparison {
                    left: ValueRef {
                        id: self.left.id,
                        phantom: std::marker::PhantomData,
                    },
                    comparator: c.clone(),
                    right: tp.clone(),
                }),
                RightHandSide::Boolean(bp, c) => Comparison::Boolean(BooleanComparison {
                    left: ValueRef {
                        id: self.left.id,
                        phantom: std::marker::PhantomData,
                    },
                    comparator: c.clone(),
                    right: bp.clone(),
                }),
            },
            output: self.output.clone(),
        };

        match self.output_type() {
            ValueType::Number => TypedValueProducer::Number(Box::new(source)),
            ValueType::Text => TypedValueProducer::Text(Box::new(source)),
            ValueType::Tint => TypedValueProducer::Tint(Box::new(source)),
            ValueType::Boolean => TypedValueProducer::Boolean(Box::new(source)),
            ValueType::Texture => TypedValueProducer::Texture(Box::new(source)),
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
            let new_ref = asset_repo.untyped_editor(ui, &mut self.left.id, |v| {
                return match v.asset_type {
                    ValueType::Number => true,
                    ValueType::Text => true,
                    ValueType::Boolean => true,
                    _ => false,
                };
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
                    RightHandSide::Boolean(b, _) => {
                        ui.add(PropertyEditor::new(b, asset_repo)).changed()
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

struct ConditionSource {
    comparison: Comparison,
    output: Output,
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
            match &self.output {
                Output::Number { truee, .. } => value_store.get_property(truee, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.output {
                Output::Number { falsee, .. } => value_store.get_property(falsee, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Text> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.output {
                Output::Text { truee, .. } => value_store.get_property(truee, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.output {
                Output::Text { falsee, .. } => value_store.get_property(falsee, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Tint> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.output {
                Output::Color { truee, .. } => value_store.get_property(truee, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.output {
                Output::Color { falsee, .. } => value_store.get_property(falsee, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Boolean> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.output {
                Output::Boolean { truee, .. } => value_store.get_property(truee, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.output {
                Output::Boolean { falsee, .. } => value_store.get_property(falsee, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ValueProducer<Texture> for ConditionSource {
    fn get(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.output {
                Output::Image { truee, .. } => value_store.get_property(truee, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.output {
                Output::Image { falsee, .. } => value_store.get_property(falsee, entry),
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
