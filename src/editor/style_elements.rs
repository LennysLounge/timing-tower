use bevy::prelude::{Color, Resource};
use bevy_egui::egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::{
    properties::{ColorProperty, NumberProperty, TextProperty, Vec2Property, Vec3Property},
    timing_tower_elements::TimingTowerElement,
    variable_element::VariablesElement,
};

pub trait StyleElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>);
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement>;
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo);
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct RootElement {
    pub id: Uuid,
    pub vars: VariablesElement,
    pub timing_tower: TimingTowerElement,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellElement {
    pub value_source: TextProperty,
    pub color: ColorProperty,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: NumberProperty,
    pub visible: bool,
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

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableDef {
    Number(f32),
    Text(String),
    Color(Color),
}

impl StyleElement for RootElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        self.vars.element_tree(ui, selected_element);
        self.timing_tower.element_tree(ui, selected_element);
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.vars
            .find_mut(id)
            .or_else(|| self.timing_tower.find_mut(id))
    }

    fn property_editor(&mut self, ui: &mut Ui, _vars: &VariableRepo) {
        ui.label("Scene");
    }
}

impl Default for CellElement {
    fn default() -> Self {
        Self {
            value_source: TextProperty::Fixed("Column".to_string()),
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
            visible: true,
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
        }
    }
}

impl CellElement {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            ui.checkbox(&mut self.visible, "");
        });
        ui.horizontal(|ui| {
            ui.label("value source:");
            self.value_source.editor(ui, vars);
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
            self.text_position.x.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            self.text_position.y.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            self.color.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            self.pos.x.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            self.pos.y.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            self.pos.z.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            self.size.x.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            self.size.y.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            self.skew.editor(ui, vars);
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            self.rounding.top_left.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            self.rounding.top_right.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            self.rounding.bot_right.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            self.rounding.bot_left.editor(ui, vars);
        });
    }
}
