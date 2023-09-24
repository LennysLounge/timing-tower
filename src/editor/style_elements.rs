use bevy::prelude::{Color, Resource, Vec2, Vec3};
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::{
    properties::{ColorProperty, NumberProperty, TextProperty},
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
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: NumberProperty,
    pub visible: bool,
    pub rounding: Rounding,
    pub text_alginment: TextAlignment,
    pub text_position: Vec2,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ValueSource {
    FixedValue(String),
    DriverName,
    Position,
    CarNumber,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Rounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bot_left: f32,
    pub bot_right: f32,
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
            pos: Vec3::new(10.0, 10.0, 0.0),
            size: Vec2::new(30.0, 30.0),
            skew: NumberProperty::Fixed(12.0),
            visible: true,
            rounding: Rounding::default(),
            text_alginment: TextAlignment::default(),
            text_position: Vec2::new(5.0, 15.0),
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
            ui.add(DragValue::new(&mut self.text_position.x));
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            ui.add(DragValue::new(&mut self.text_position.y));
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            self.color.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            ui.add(DragValue::new(&mut self.pos.x));
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            ui.add(DragValue::new(&mut self.pos.y));
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            ui.add(DragValue::new(&mut self.pos.z));
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(DragValue::new(&mut self.size.x).clamp_range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            ui.add(DragValue::new(&mut self.size.y).clamp_range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            self.skew.editor(ui, vars);
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            ui.add(DragValue::new(&mut self.rounding.top_left).clamp_range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            ui.add(DragValue::new(&mut self.rounding.top_right).clamp_range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            ui.add(DragValue::new(&mut self.rounding.bot_right).clamp_range(0.0..=f32::MAX));
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            ui.add(DragValue::new(&mut self.rounding.bot_left).clamp_range(0.0..=f32::MAX));
        });
    }
}
