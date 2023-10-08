use bevy::prelude::Color;
use bevy_egui::egui::{vec2, ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    asset_reference_repo::AssetReferenceRepo,
    asset_repo::{AssetId, AssetReference, AssetSource, AssetType, IntoAssetSource},
    style::properties::{
        BooleanProperty, ColorProperty, ImageProperty, NumberProperty, TextProperty,
    },
};

use super::{fixed_value::StaticNumber, variant_checkbox};

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    #[serde(flatten)]
    id: AssetId,
    input: AssetReference,
    cases: Vec<Case>,
    default: Output,
}

impl IntoAssetSource for Map {
    fn get_asset_source(&self) -> AssetSource {
        AssetSource::Number(Box::new(StaticNumber(1234.0)))
    }

    fn asset_id(&self) -> &AssetId {
        &self.id
    }
}

impl Map {
    pub fn from_id(id: AssetId) -> Self {
        Self {
            id,
            input: AssetReference::default(),
            cases: Vec::new(),
            default: Output::Number(NumberProperty::Fixed(0.0)),
        }
    }
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Map input: ");
            let new_ref = asset_repo.editor(ui, &self.input, |v|
                match v.asset_type{
                    AssetType::Number => true,
                    AssetType::Text => true,
                    _ => false
                } &&
                v.id != self.id.id);
            if let Some(new_ref) = new_ref {
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
                    (&AssetType::Number, "Number"),
                    (&AssetType::Text, "Text"),
                    (&AssetType::Color, "Color"),
                    (&AssetType::Boolean, "Yes/No"),
                    (&AssetType::Image, "Image"),
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

        changed
    }

    pub fn get_id_mut(&mut self) -> &mut AssetId {
        &mut self.id
    }

    fn update_output_type(&mut self) {
        let new_output = match self.id.asset_type {
            AssetType::Number => Output::Number(NumberProperty::Fixed(0.0)),
            AssetType::Text => Output::Text(TextProperty::Fixed(String::new())),
            AssetType::Color => Output::Color(ColorProperty::Fixed(Color::WHITE)),
            AssetType::Boolean => Output::Boolean(BooleanProperty::Fixed(false)),
            AssetType::Image => Output::Image(ImageProperty::None),
        };
        self.default = new_output.clone();
        for case in self.cases.iter_mut() {
            case.output = new_output.clone();
        }
    }

    fn update_comparison_type(&mut self) {
        let new_comparison = match self.input.asset_type {
            AssetType::Number => {
                Comparison::Number(NumberProperty::Fixed(0.0), NumberComparator::Equal)
            }
            AssetType::Text => {
                Comparison::Text(TextProperty::Fixed(String::new()), TextComparator::Like)
            }
            AssetType::Color => unreachable!("Type Color not allowed in comparison"),
            AssetType::Boolean => unreachable!("Type Boolean not allowed in comparison"),
            AssetType::Image => unreachable!("Type Image not allowed in comparison"),
        };
        for case in self.cases.iter_mut() {
            case.comparison = new_comparison.clone();
        }
    }

    fn new_comparison(&self) -> Comparison {
        match self.input.asset_type {
            AssetType::Number => {
                Comparison::Number(NumberProperty::Fixed(0.0), NumberComparator::Equal)
            }
            AssetType::Text => {
                Comparison::Text(TextProperty::Fixed(String::new()), TextComparator::Like)
            }
            AssetType::Color => unreachable!(),
            AssetType::Boolean => unreachable!(),
            AssetType::Image => unreachable!(),
        }
    }

    fn new_output(&self) -> Output {
        match self.id.asset_type {
            AssetType::Number => Output::Number(NumberProperty::Fixed(0.0)),
            AssetType::Text => Output::Text(TextProperty::Fixed(String::new())),
            AssetType::Color => Output::Color(ColorProperty::Fixed(Color::WHITE)),
            AssetType::Boolean => Output::Boolean(BooleanProperty::Fixed(false)),
            AssetType::Image => Output::Image(ImageProperty::None),
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

#[derive(Serialize, Deserialize, Clone)]
struct Case {
    comparison: Comparison,
    output: Output,
    remove: bool,
}

impl Case {
    fn show(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
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
    Number(NumberProperty, NumberComparator),
    Text(TextProperty, TextComparator),
}

impl Comparison {
    fn show(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum TextComparator {
    Like,
}

#[derive(Serialize, Deserialize, Clone)]
enum Output {
    Number(NumberProperty),
    Text(TextProperty),
    Color(ColorProperty),
    Boolean(BooleanProperty),
    Image(ImageProperty),
}

impl Output {
    fn show(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
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
