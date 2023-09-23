use bevy::prelude::{Color, Resource, Vec2, Vec3};
use bevy_egui::egui::{ComboBox, DragValue, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{scope::Scope, timing_tower_elements::TimingTowerElement};

pub trait StyleElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>);
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement>;
    fn property_editor(&mut self, ui: &mut Ui);
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct SceneElement {
    pub id: Uuid,
    pub scope: Scope,
    pub timing_tower: TimingTowerElement,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CellElement {
    pub value_source: ValueSource,
    pub color: Color,
    pub pos: Vec3,
    pub size: Vec2,
    pub skew: f32,
    pub visible: bool,
    pub rounding: Rounding,
    pub text_alginment: TextAlignment,
    pub text_position: Vec2,
}

pub enum NumberProperty {
    Fixed(f32),
    Ref(String),
}

pub enum TextProperty {
    Fixed(String),
    Ref(String),
}

pub enum ColorProperty {
    Fixed(Color),
    Ref(String),
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
pub enum PropertyValue {
    VarRef(String),
    #[serde(untagged)]
    Text(String),
    #[serde(untagged)]
    Color(Color),
    #[serde(untagged)]
    Number(f32),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableDef {
    Number(f32),
    Text(String),
    Color(Color),
}

impl StyleElement for SceneElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        self.timing_tower.element_tree(ui, selected_element);
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.timing_tower.find_mut(id)
    }

    fn property_editor(&mut self, ui: &mut Ui) {
        ui.label("Scene");
    }
}

impl Default for CellElement {
    fn default() -> Self {
        Self {
            value_source: ValueSource::FixedValue("Column".to_string()),
            color: Color::PURPLE,
            pos: Vec3::new(10.0, 10.0, 0.0),
            size: Vec2::new(30.0, 30.0),
            skew: 0.0,
            visible: true,
            rounding: Rounding::default(),
            text_alginment: TextAlignment::default(),
            text_position: Vec2::new(5.0, 15.0),
        }
    }
}

pub fn cell_style_editor(ui: &mut Ui, style: &mut CellElement) {
    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        ui.checkbox(&mut style.visible, "");
    });
    ui.horizontal(|ui| {
        ui.label("value source:");
        ComboBox::from_id_source("cell value source")
            .selected_text(match &style.value_source {
                ValueSource::FixedValue(_) => "Fixed value",
                ValueSource::DriverName => "Driver name",
                ValueSource::Position => "Position",
                ValueSource::CarNumber => "Car number",
            })
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::FixedValue(_)),
                        "Fixed value",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::FixedValue("".to_string());
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::DriverName),
                        "Driver name",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::DriverName;
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::Position),
                        "Position",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::Position;
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::CarNumber),
                        "Car number",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::CarNumber;
                };
            });
    });
    if let ValueSource::FixedValue(s) = &mut style.value_source {
        ui.horizontal(|ui| {
            ui.label("Text:");
            ui.text_edit_singleline(s);
        });
    }
    ui.horizontal(|ui| {
        ui.label("Text alginment:");
        ComboBox::from_id_source("Text alginment combobox")
            .selected_text(match style.text_alginment {
                TextAlignment::Left => "Left",
                TextAlignment::Center => "Center",
                TextAlignment::Right => "Right",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Left, "Left");
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Center, "Center");
                ui.selectable_value(&mut style.text_alginment, TextAlignment::Right, "Right");
            });
    });
    ui.horizontal(|ui| {
        ui.label("Text pos x:");
        ui.add(DragValue::new(&mut style.text_position.x));
    });
    ui.horizontal(|ui| {
        ui.label("Text pos y:");
        ui.add(DragValue::new(&mut style.text_position.y));
    });
    ui.horizontal(|ui| {
        ui.label("Background color:");
        let mut color = style.color.as_rgba_f32();
        ui.color_edit_button_rgba_unmultiplied(&mut color);
        style.color = color.into();
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        ui.add(DragValue::new(&mut style.pos.x));
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        ui.add(DragValue::new(&mut style.pos.y));
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        ui.add(DragValue::new(&mut style.pos.z));
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(DragValue::new(&mut style.size.x).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(DragValue::new(&mut style.size.y).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        ui.add(DragValue::new(&mut style.skew));
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        ui.add(DragValue::new(&mut style.rounding.top_left).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        ui.add(DragValue::new(&mut style.rounding.top_right).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        ui.add(DragValue::new(&mut style.rounding.bot_right).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        ui.add(DragValue::new(&mut style.rounding.bot_left).clamp_range(0.0..=f32::MAX));
    });
}
