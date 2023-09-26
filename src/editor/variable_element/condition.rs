use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    editor::{
        properties::{ColorProperty, NumberProperty, TextProperty},
        style_elements::reference_editor,
    },
    variable_repo::{Reference, StaticNumber, ValueType, VariableId, VariableRepo, VariableSource},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    id: VariableId,
    left: Reference,
    right: RightHandSide,
    true_output: Output,
    false_output: Output,
}

#[derive(Serialize, Deserialize, Clone)]
enum RightHandSide {
    Number(NumberProperty, NumberComparator),
    Text(TextProperty, TextComparator),
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

#[derive(Serialize, Deserialize, Clone)]
enum Output {
    Number(NumberProperty),
    Text(TextProperty),
    Color(ColorProperty),
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            id: Default::default(),
            left: Default::default(),
            right: RightHandSide::Number(NumberProperty::Fixed(0.0), NumberComparator::Equal),
            true_output: Output::Number(NumberProperty::Fixed(0.0)),
            false_output: Output::Number(NumberProperty::Fixed(0.0)),
        }
    }
}

impl Condition {
    pub fn from_id(id: VariableId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> &VariableId {
        &self.id
    }
    pub fn get_id_mut(&mut self) -> &mut VariableId {
        &mut self.id
    }

    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.id.value_type {
                    ValueType::Number => "Number",
                    ValueType::Text => "Text",
                    ValueType::Color => "Color",
                })
                .show_ui(ui, |ui| {
                    let is_number = self.id.value_type == ValueType::Number;
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.id.value_type = ValueType::Number;
                        self.true_output = Output::Number(NumberProperty::Fixed(0.0));
                        self.false_output = Output::Number(NumberProperty::Fixed(0.0));
                    }
                    let is_text = self.id.value_type == ValueType::Text;
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.id.value_type = ValueType::Text;
                        self.true_output = Output::Text(TextProperty::Fixed(String::new()));
                        self.false_output = Output::Text(TextProperty::Fixed(String::new()));
                    }
                    let is_color = self.id.value_type == ValueType::Color;
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.id.value_type = ValueType::Color;
                        self.true_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                        self.false_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                    }
                });
        });
        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

        ui.horizontal(|ui| {
            ui.label("If");
            ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());
            let new_ref = reference_editor(ui, vars, &mut self.left, |v| match v.value_type {
                ValueType::Number => true,
                ValueType::Text => true,
                _ => false,
            });
            if let Some(reference) = new_ref {
                // Channge the value type of the right side if necessary
                if self.left.value_type != reference.value_type {
                    self.right = match reference.value_type {
                        ValueType::Number => RightHandSide::Number(
                            NumberProperty::Fixed(0.0),
                            NumberComparator::Equal,
                        ),
                        ValueType::Text => RightHandSide::Text(
                            TextProperty::Fixed(String::new()),
                            TextComparator::Like,
                        ),
                        ValueType::Color => unreachable!(),
                    }
                }
                self.left = reference;
            }
            ui.label("is");
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.right {
                RightHandSide::Number(_, c) => {
                    ComboBox::from_id_source(ui.next_auto_id())
                        .selected_text(match c {
                            NumberComparator::Equal => "equal",
                            NumberComparator::Greater => "greater",
                            NumberComparator::GreaterEqual => "greater or equal",
                            NumberComparator::Less => "less",
                            NumberComparator::LessEqual => "less or equal",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(c, NumberComparator::Equal, "equal");
                            ui.selectable_value(c, NumberComparator::Greater, "greater");
                            ui.selectable_value(
                                c,
                                NumberComparator::GreaterEqual,
                                "greater or equal",
                            );
                            ui.selectable_value(c, NumberComparator::Less, "less");
                            ui.selectable_value(c, NumberComparator::LessEqual, "less or equal");
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
                        .selected_text(match c {
                            TextComparator::Like => "like",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(c, TextComparator::Like, "like")
                        });
                }
            }
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            // show select for right side
            ui.horizontal(|ui| match &mut self.right {
                RightHandSide::Number(n, _) => n.editor(ui, vars),
                RightHandSide::Text(t, _) => t.editor(ui, vars),
            });
        });
        ui.label("then:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.true_output {
                Output::Number(n) => n.editor(ui, vars),
                Output::Text(t) => t.editor(ui, vars),
                Output::Color(c) => c.editor(ui, vars),
            }
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.false_output {
                Output::Number(n) => n.editor(ui, vars),
                Output::Text(t) => t.editor(ui, vars),
                Output::Color(c) => c.editor(ui, vars),
            }
        });
    }

    pub fn as_variable_source(&self) -> VariableSource {
        VariableSource::Number(Box::new(StaticNumber(0.0)))
    }
}
