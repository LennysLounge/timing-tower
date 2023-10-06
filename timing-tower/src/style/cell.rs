use bevy::prelude::Color;
use bevy_egui::egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};

use crate::asset_reference_repo::AssetReferenceRepo;

use super::properties::{
    BooleanProperty, ColorProperty, NumberProperty, TextProperty, Vec2Property, Vec3Property,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub text: TextProperty,
    pub text_color: ColorProperty,
    pub text_size: NumberProperty,
    pub color: ColorProperty,
    pub image: TextProperty,
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
            image: TextProperty::Fixed(String::new()),
        }
    }
}

impl Cell {
    pub fn property_editor(&mut self, ui: &mut Ui, asset_repo: &AssetReferenceRepo) {
        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            self.visible.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text:");
            self.text.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text color:");
            self.text_color.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text size:");
            self.text_size.editor(ui, asset_repo);
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
                    ui.selectable_value(&mut self.text_alginment, TextAlignment::Left, "Left");
                    ui.selectable_value(&mut self.text_alginment, TextAlignment::Center, "Center");
                    ui.selectable_value(&mut self.text_alginment, TextAlignment::Right, "Right");
                });
        });
        ui.horizontal(|ui| {
            ui.label("Text pos x:");
            self.text_position.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            self.text_position.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            self.color.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Background image:");
            self.image.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            self.pos.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            self.pos.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            self.pos.z.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            self.size.x.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            self.size.y.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            self.skew.editor(ui, asset_repo);
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            self.rounding.top_left.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            self.rounding.top_right.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            self.rounding.bot_right.editor(ui, asset_repo);
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            self.rounding.bot_left.editor(ui, asset_repo);
        });
    }
}
