use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::asset_reference_repo::AssetReferenceRepo;

use super::properties::{
    BooleanProperty, ColorProperty, ImageProperty, NumberProperty, TextProperty, Vec2Property,
    Vec3Property,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub text: TextProperty,
    pub text_color: ColorProperty,
    pub text_size: NumberProperty,
    pub color: ColorProperty,
    #[serde(default)]
    pub image: ImageProperty,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: NumberProperty,
    pub visible: BooleanProperty,
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
    pub top_left: NumberProperty,
    pub top_right: NumberProperty,
    pub bot_left: NumberProperty,
    pub bot_right: NumberProperty,
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
            text: TextProperty::Fixed("Column".to_string()),
            text_color: ColorProperty::Fixed(Color::BLACK),
            text_size: NumberProperty::Fixed(20.0),
            color: ColorProperty::Fixed(Color::PURPLE),
            pos: Vec3Property {
                x: NumberProperty::Fixed(10.0),
                y: NumberProperty::Fixed(10.0),
                z: NumberProperty::Fixed(0.0),
            },
            size: Vec2Property {
                x: NumberProperty::Fixed(30.0),
                y: NumberProperty::Fixed(30.0),
            },
            skew: NumberProperty::Fixed(12.0),
            visible: BooleanProperty::Fixed(true),
            rounding: Rounding {
                top_left: NumberProperty::Fixed(0.0),
                top_right: NumberProperty::Fixed(0.0),
                bot_left: NumberProperty::Fixed(0.0),
                bot_right: NumberProperty::Fixed(0.0),
            },
            text_alginment: TextAlignment::default(),
            text_position: Vec2Property {
                x: NumberProperty::Fixed(5.0),
                y: NumberProperty::Fixed(15.0),
            },
            image: ImageProperty::default(),
        }
    }
}

impl Cell {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) -> bool {
        let mut changed = false;

        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            changed |= self.visible.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text:");
            changed |= self.text.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text color:");
            changed |= self.text_color.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text size:");
            changed |= self.text_size.editor(ui, asset_repo);
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
            changed |= self.text_position.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            changed |= self.text_position.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            changed |= self.color.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Background image:");
            changed |= self.image.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            changed |= self.pos.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            changed |= self.pos.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            changed |= self.pos.z.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            changed |= self.size.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            changed |= self.size.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            changed |= self.skew.editor(ui, asset_repo);
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            changed |= self.rounding.top_left.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            changed |= self.rounding.top_right.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            changed |= self.rounding.bot_right.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            changed |= self.rounding.bot_left.editor(ui, asset_repo);
        });
        changed
    }
}