use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    asset_repo::{
        AssetId, AssetReference, AssetRepo, AssetSource, AssetType, BooleanSource, ColorSource,
        IntoAssetSource, NumberSource, TextSource,
    },
    style::properties::{BooleanProperty, ColorProperty, NumberProperty, TextProperty},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    #[serde(flatten)]
    id: AssetId,
    left: AssetReference,
    right: RightHandSide,
    true_output: Output,
    false_output: Output,
}

#[derive(Serialize, Deserialize, Clone)]
enum RightHandSide {
    Number(NumberProperty, NumberComparator),
    Text(TextProperty, TextComparator),
    Boolean(BooleanProperty, BooleanComparator),
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
    Number(NumberProperty),
    Text(TextProperty),
    Color(ColorProperty),
    Boolean(BooleanProperty),
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

impl IntoAssetSource for Condition {
    fn get_asset_source(&self) -> AssetSource {
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
            AssetType::Number => AssetSource::Number(Box::new(source)),
            AssetType::Text => AssetSource::Text(Box::new(source)),
            AssetType::Color => AssetSource::Color(Box::new(source)),
            AssetType::Boolean => AssetSource::Bool(Box::new(source)),
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
                AssetType::Number => Output::Number(NumberProperty::Fixed(0.0)),
                AssetType::Text => Output::Text(TextProperty::Fixed(String::new())),
                AssetType::Color => Output::Color(ColorProperty::Fixed(Color::WHITE)),
                AssetType::Boolean => Output::Boolean(BooleanProperty::Fixed(true)),
            },
            false_output: match &id.asset_type {
                AssetType::Number => Output::Number(NumberProperty::Fixed(0.0)),
                AssetType::Text => Output::Text(TextProperty::Fixed(String::new())),
                AssetType::Color => Output::Color(ColorProperty::Fixed(Color::WHITE)),
                AssetType::Boolean => Output::Boolean(BooleanProperty::Fixed(false)),
            },
            id,
            ..Default::default()
        }
    }
    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        ui.horizontal(|ui| {
            ui.label("Output type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.id.asset_type {
                    AssetType::Number => "Number",
                    AssetType::Text => "Text",
                    AssetType::Color => "Color",
                    AssetType::Boolean => "Yes/No",
                })
                .show_ui(ui, |ui| {
                    let is_number = self.id.asset_type == AssetType::Number;
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.id.asset_type = AssetType::Number;
                        self.true_output = Output::Number(NumberProperty::Fixed(0.0));
                        self.false_output = Output::Number(NumberProperty::Fixed(0.0));
                    }
                    let is_text = self.id.asset_type == AssetType::Text;
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.id.asset_type = AssetType::Text;
                        self.true_output = Output::Text(TextProperty::Fixed(String::new()));
                        self.false_output = Output::Text(TextProperty::Fixed(String::new()));
                    }
                    let is_color = self.id.asset_type == AssetType::Color;
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.id.asset_type = AssetType::Color;
                        self.true_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                        self.false_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                    }
                    let is_boolean = self.id.asset_type == AssetType::Boolean;
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.id.asset_type = AssetType::Boolean;
                        self.true_output = Output::Boolean(BooleanProperty::Fixed(true));
                        self.false_output = Output::Boolean(BooleanProperty::Fixed(false));
                    }
                });
        });
        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

        ui.horizontal(|ui| {
            ui.label("If");
            ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());
            let new_ref = asset_repo.editor(ui, &mut self.left, |v| {
                return match v.asset_type {
                    AssetType::Number => true,
                    AssetType::Text => true,
                    AssetType::Boolean => true,
                    _ => false,
                } && v.id != self.id.id;
            });
            if let Some(reference) = new_ref {
                // Channge the value type of the right side if necessary
                if self.left.asset_type != reference.asset_type {
                    self.right = match reference.asset_type {
                        AssetType::Number => RightHandSide::Number(
                            NumberProperty::Fixed(0.0),
                            NumberComparator::Equal,
                        ),
                        AssetType::Text => RightHandSide::Text(
                            TextProperty::Fixed(String::new()),
                            TextComparator::Like,
                        ),
                        AssetType::Boolean => RightHandSide::Boolean(
                            BooleanProperty::Fixed(true),
                            BooleanComparator::Is,
                        ),
                        AssetType::Color => unreachable!(),
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
                        .width(50.0)
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
                        .width(50.0)
                        .selected_text(match c {
                            TextComparator::Like => "like",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(c, TextComparator::Like, "like")
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
                            ui.selectable_value(c, BooleanComparator::Is, "is");
                            ui.selectable_value(c, BooleanComparator::IsNot, "is not");
                        });
                }
            }
        });

        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            // show select for right side
            ui.horizontal(|ui| match &mut self.right {
                RightHandSide::Number(n, _) => n.editor(ui, asset_repo),
                RightHandSide::Text(t, _) => t.editor(ui, asset_repo),
                RightHandSide::Boolean(b, _) => b.editor(ui, asset_repo),
            });
        });
        ui.label("then:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.true_output {
                Output::Number(n) => n.editor(ui, asset_repo),
                Output::Text(t) => t.editor(ui, asset_repo),
                Output::Color(c) => c.editor(ui, asset_repo),
                Output::Boolean(b) => b.editor(ui, asset_repo),
            }
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            match &mut self.false_output {
                Output::Number(n) => n.editor(ui, asset_repo),
                Output::Text(t) => t.editor(ui, asset_repo),
                Output::Color(c) => c.editor(ui, asset_repo),
                Output::Boolean(b) => b.editor(ui, asset_repo),
            }
        });
    }
}

struct ConditionSource {
    comparison: Comparison,
    true_value: Output,
    false_value: Output,
}

impl ConditionSource {
    fn evaluate_condition(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        match &self.comparison {
            Comparison::Number(n) => n.evaluate(vars, entry),
            Comparison::Text(t) => t.evaluate(vars, entry),
            Comparison::Boolean(b) => b.evaluate(vars, entry),
        }
    }
}

impl NumberSource for ConditionSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32> {
        let condition = self.evaluate_condition(vars, entry)?;
        if condition {
            match &self.true_value {
                Output::Number(n) => vars.get_number_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Number(n) => vars.get_number_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl TextSource for ConditionSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<String> {
        let condition = self.evaluate_condition(vars, entry)?;
        if condition {
            match &self.true_value {
                Output::Text(n) => vars.get_text_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Text(n) => vars.get_text_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}
impl ColorSource for ConditionSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<Color> {
        let condition = self.evaluate_condition(vars, entry)?;
        if condition {
            match &self.true_value {
                Output::Color(n) => vars.get_color_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Color(n) => vars.get_color_property(&n, entry),
                _ => unreachable!(),
            }
        }
    }
}

impl BooleanSource for ConditionSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        let condition = self.evaluate_condition(vars, entry)?;
        if condition {
            match &self.true_value {
                Output::Boolean(b) => vars.get_bool_property(&b, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Boolean(b) => vars.get_bool_property(&b, entry),
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
    left: AssetReference,
    comparator: NumberComparator,
    right: NumberProperty,
}

impl NumberComparison {
    fn evaluate(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_number(&self.left, entry)?;
        let right = vars.get_number_property(&self.right, entry)?;
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
    left: AssetReference,
    comparator: TextComparator,
    right: TextProperty,
}

impl TextComparison {
    fn evaluate(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_text(&self.left, entry)?;
        let right = vars.get_text_property(&self.right, entry)?;
        Some(match self.comparator {
            TextComparator::Like => left == right,
        })
    }
}

struct BooleanComparison {
    left: AssetReference,
    comparator: BooleanComparator,
    right: BooleanProperty,
}
impl BooleanComparison {
    fn evaluate(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_bool(&self.left, entry)?;
        let right = vars.get_bool_property(&self.right, entry)?;
        Some(match self.comparator {
            BooleanComparator::Is => left == right,
            BooleanComparator::IsNot => left != right,
        })
    }
}
