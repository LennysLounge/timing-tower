use backend::value_types::{Property, Vec2Property, Vec3Property};
use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::reference_store::ReferenceStore;

use super::properties::PropertyEditor;
use backend::value_types::{Boolean, Number, Text, Texture, Tint};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub text: Property<Text>,
    pub text_color: Property<Tint>,
    pub text_size: Property<Number>,
    pub color: Property<Tint>,
    #[serde(default)]
    pub image: Property<Texture>,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub visible: Property<Boolean>,
    pub rounding: Rounding,
    pub text_alginment: TextAlignment,
    pub text_position: Vec2Property,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ValueSource {
    FixedValue(String),
    DriverName,
    Position,
    CarNumber,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rounding {
    pub top_left: Property<Number>,
    pub top_right: Property<Number>,
    pub bot_left: Property<Number>,
    pub bot_right: Property<Number>,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            text: Property::Fixed(Text("Column".to_string())),
            text_color: Property::Fixed(Tint(Color::BLACK)),
            text_size: Property::Fixed(Number(20.0)),
            color: Property::Fixed(Tint(Color::PURPLE)),
            pos: Vec3Property {
                x: Property::Fixed(Number(10.0)),
                y: Property::Fixed(Number(10.0)),
                z: Property::Fixed(Number(0.0)),
            },
            size: Vec2Property {
                x: Property::Fixed(Number(30.0)),
                y: Property::Fixed(Number(30.0)),
            },
            skew: Property::Fixed(Number(12.0)),
            visible: Property::Fixed(Boolean(true)),
            rounding: Rounding {
                top_left: Property::Fixed(Number(0.0)),
                top_right: Property::Fixed(Number(0.0)),
                bot_left: Property::Fixed(Number(0.0)),
                bot_right: Property::Fixed(Number(0.0)),
            },
            text_alginment: TextAlignment::default(),
            text_position: Vec2Property {
                x: Property::Fixed(Number(5.0)),
                y: Property::Fixed(Number(15.0)),
            },
            image: Property::<Texture>::default(),
        }
    }
}

impl Cell {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.visible, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text color:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_color, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text size:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_size, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text alginment:");
            ComboBox::from_id_source("Text alginment combobox")
                .selected_text(match self.text_alginment {
                    TextAlignment::Left => "Left",
                    TextAlignment::Center => "Center",
                    TextAlignment::Right => "Right",
                })
                .show_ui(ui, |ui| {
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Left, "Left")
                        .changed();
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Center, "Center")
                        .changed();
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Right, "Right")
                        .changed();
                });
        });
        ui.horizontal(|ui| {
            ui.label("Text pos x:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_position.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_position.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.color, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Background image:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.image, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.z, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.size.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.size.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.skew, asset_repo))
                .changed();
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.rounding.top_left, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            changed |= ui
                .add(PropertyEditor::new(
                    &mut self.rounding.top_right,
                    asset_repo,
                ))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            changed |= ui
                .add(PropertyEditor::new(
                    &mut self.rounding.bot_right,
                    asset_repo,
                ))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.rounding.bot_left, asset_repo))
                .changed();
        });
        changed
    }
}
