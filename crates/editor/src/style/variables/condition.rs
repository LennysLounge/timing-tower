use bevy::prelude::{Color, Handle, Image};
use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    style::properties::{
        BooleanProperty, ColorProperty, ImageProperty, NumberProperty, TextProperty,
    },
    value_store::{
        types::{Boolean, Number, Text, Texture, Tint},
        AssetId, AssetReference, AssetType, BooleanSource, ColorSource, ImageSource,
        IntoValueProducer, NumberSource, TextSource, ValueProducer, ValueStore,
    },
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
    Image(ImageProperty),
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

impl IntoValueProducer for Condition {
    fn get_value_producer(&self) -> Box<dyn ValueProducer + Send + Sync> {
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

        Box::new(source)
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
                AssetType::Image => Output::Image(ImageProperty::default()),
            },
            false_output: match &id.asset_type {
                AssetType::Number => Output::Number(NumberProperty::Fixed(0.0)),
                AssetType::Text => Output::Text(TextProperty::Fixed(String::new())),
                AssetType::Color => Output::Color(ColorProperty::Fixed(Color::WHITE)),
                AssetType::Boolean => Output::Boolean(BooleanProperty::Fixed(false)),
                AssetType::Image => Output::Image(ImageProperty::default()),
            },
            id,
            ..Default::default()
        }
    }
    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Output type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.id.asset_type {
                    AssetType::Number => "Number",
                    AssetType::Text => "Text",
                    AssetType::Color => "Color",
                    AssetType::Boolean => "Yes/No",
                    AssetType::Image => "Image",
                })
                .show_ui(ui, |ui| {
                    let is_number = self.id.asset_type == AssetType::Number;
                    if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                        self.id.asset_type = AssetType::Number;
                        self.true_output = Output::Number(NumberProperty::Fixed(0.0));
                        self.false_output = Output::Number(NumberProperty::Fixed(0.0));
                        changed |= true;
                    }
                    let is_text = self.id.asset_type == AssetType::Text;
                    if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                        self.id.asset_type = AssetType::Text;
                        self.true_output = Output::Text(TextProperty::Fixed(String::new()));
                        self.false_output = Output::Text(TextProperty::Fixed(String::new()));
                        changed |= true;
                    }
                    let is_color = self.id.asset_type == AssetType::Color;
                    if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                        self.id.asset_type = AssetType::Color;
                        self.true_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                        self.false_output = Output::Color(ColorProperty::Fixed(Color::WHITE));
                        changed |= true;
                    }
                    let is_boolean = self.id.asset_type == AssetType::Boolean;
                    if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                        self.id.asset_type = AssetType::Boolean;
                        self.true_output = Output::Boolean(BooleanProperty::Fixed(true));
                        self.false_output = Output::Boolean(BooleanProperty::Fixed(false));
                        changed |= true;
                    }
                    let is_image = self.id.asset_type == AssetType::Image;
                    if ui.selectable_label(is_image, "Image").clicked() && !is_image {
                        self.id.asset_type = AssetType::Image;
                        self.true_output = Output::Image(ImageProperty::default());
                        self.false_output = Output::Image(ImageProperty::default());
                        changed |= true;
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
                        AssetType::Color => unreachable!("Type color not allowed for if condition"),
                        AssetType::Image => unreachable!("Type image not allowed for if condition"),
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
                    RightHandSide::Number(n, _) => n.editor(ui, asset_repo),
                    RightHandSide::Text(t, _) => t.editor(ui, asset_repo),
                    RightHandSide::Boolean(b, _) => b.editor(ui, asset_repo),
                })
                .inner;
        });
        ui.label("then:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.true_output {
                Output::Number(n) => n.editor(ui, asset_repo),
                Output::Text(t) => t.editor(ui, asset_repo),
                Output::Color(c) => c.editor(ui, asset_repo),
                Output::Boolean(b) => b.editor(ui, asset_repo),
                Output::Image(i) => i.editor(ui, asset_repo),
            };
        });
        ui.label("else:");
        ui.horizontal(|ui| {
            ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
            changed |= match &mut self.false_output {
                Output::Number(n) => n.editor(ui, asset_repo),
                Output::Text(t) => t.editor(ui, asset_repo),
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

impl ValueProducer for ConditionSource {
    fn get_number(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Number> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Number(n) => value_store.get_number_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Number(n) => value_store.get_number_property(&n, entry),
                _ => unreachable!(),
            }
        }
        .map(|n| Number(n))
    }

    fn get_text(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Text> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Text(n) => value_store.get_text_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Text(n) => value_store.get_text_property(&n, entry),
                _ => unreachable!(),
            }
        }
        .map(|n| Text(n))
    }

    fn get_tint(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Tint> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Color(n) => value_store.get_color_property(&n, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Color(n) => value_store.get_color_property(&n, entry),
                _ => unreachable!(),
            }
        }
        .map(|n| Tint(n))
    }

    fn get_boolean(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Boolean> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Boolean(b) => value_store.get_bool_property(&b, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Boolean(b) => value_store.get_bool_property(&b, entry),
                _ => unreachable!(),
            }
        }
        .map(|n| Boolean(n))
    }

    fn get_texture(&self, value_store: &ValueStore, entry: Option<&Entry>) -> Option<Texture> {
        let condition = self.evaluate_condition(value_store, entry)?;
        if condition {
            match &self.true_value {
                Output::Image(i) => value_store.get_image_property(&i, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Image(i) => value_store.get_image_property(&i, entry),
                _ => unreachable!(),
            }
        }
        .map(|n| Texture(n))
    }
}

impl NumberSource for ConditionSource {
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<f32> {
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
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<String> {
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
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<Color> {
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
    fn resolve(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
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

impl ImageSource for ConditionSource {
    fn resolve(&self, repo: &ValueStore, entry: Option<&Entry>) -> Option<Handle<Image>> {
        let condition = self.evaluate_condition(repo, entry)?;
        if condition {
            match &self.true_value {
                Output::Image(i) => repo.get_image_property(&i, entry),
                _ => unreachable!(),
            }
        } else {
            match &self.false_value {
                Output::Image(i) => repo.get_image_property(&i, entry),
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
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
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
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
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
    fn evaluate(&self, vars: &ValueStore, entry: Option<&Entry>) -> Option<bool> {
        let left = vars.get_bool(&self.left, entry)?;
        let right = vars.get_bool_property(&self.right, entry)?;
        Some(match self.comparator {
            BooleanComparator::Is => left == right,
            BooleanComparator::IsNot => left != right,
        })
    }
}
