#![allow(unused)]

use std::collections::HashMap;

use bevy::prelude::{Color, Resource, Vec2};
use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, DragValue, Ui};
use uuid::Uuid;

use crate::style_def::{
    CellStyleDef, ColumnStyleDef, PropertyValue, RowStyleDef, SceneStyleDef, TableStyleDef,
    TextAlignment, TimingTowerStyleDef, ValueSource, VariableDef,
};

use super::{
    scope::{Scope, Variable},
    timing_tower_elements::TimingTowerElement,
};

pub trait StyleElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>);
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement>;
    fn property_editor(&mut self, ui: &mut Ui);
}

#[derive(Resource)]
pub struct SceneElement {
    pub id: Uuid,
    pub scope: Scope,
    pub timing_tower: TimingTowerElement,
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

impl SceneElement {
    pub fn from_style_def(style: &SceneStyleDef) -> Self {
        Self {
            id: Uuid::new_v4(),
            timing_tower: TimingTowerElement::from_style_def(&style.timing_tower),
            scope: Scope::from_style_def(&style.vars),
        }
    }
    pub fn to_style_def(&self) -> SceneStyleDef {
        SceneStyleDef {
            timing_tower: TimingTowerElement::to_style_def(&self.timing_tower),
            vars: HashMap::new(),
        }
    }
}

impl Scope {
    pub fn from_style_def(style: &HashMap<String, VariableDef>) -> Self {
        let mut vars = HashMap::new();
        for (var_name, def) in style {
            vars.insert(
                var_name.clone(),
                match def {
                    VariableDef::Number(n) => Variable::Number(*n),
                    VariableDef::Text(s) => Variable::Text(s.clone()),
                    VariableDef::Color(c) => Variable::Color(c.clone()),
                },
            );
        }
        Self { vars }
    }
}

impl NumberProperty {
    pub fn from_style_def(prop: &PropertyValue) -> Option<Self> {
        match prop {
            PropertyValue::Number(n) => Some(NumberProperty::Fixed(n.clone())),
            PropertyValue::Text(_) => None,
            PropertyValue::Color(_) => None,
            PropertyValue::VarRef(name) => Some(NumberProperty::Ref(name.clone())),
        }
    }
    pub fn to_style_def(&self) -> PropertyValue {
        match self {
            NumberProperty::Fixed(n) => PropertyValue::Number(n.clone()),
            NumberProperty::Ref(name) => PropertyValue::VarRef(name.clone()),
        }
    }
}

impl TextProperty {
    pub fn from_style_def(prop: &PropertyValue) -> Option<Self> {
        match prop {
            PropertyValue::Number(n) => Some(TextProperty::Fixed(format!("{n}"))),
            PropertyValue::Text(s) => Some(TextProperty::Fixed(s.clone())),
            PropertyValue::Color(_) => None,
            PropertyValue::VarRef(name) => Some(TextProperty::Ref(name.clone())),
        }
    }
    pub fn to_style_def(&self) -> PropertyValue {
        match self {
            TextProperty::Fixed(t) => PropertyValue::Text(t.clone()),
            TextProperty::Ref(name) => PropertyValue::VarRef(name.clone()),
        }
    }
}

impl ColorProperty {
    pub fn from_style_def(prop: &PropertyValue) -> Option<Self> {
        match prop {
            PropertyValue::Number(_) => None,
            PropertyValue::Text(_) => None,
            PropertyValue::Color(c) => Some(ColorProperty::Fixed(c.clone())),
            PropertyValue::VarRef(name) => Some(ColorProperty::Ref(name.clone())),
        }
    }

    pub fn to_style_def(&self) -> PropertyValue {
        match self {
            ColorProperty::Fixed(c) => PropertyValue::Color(c.clone()),
            ColorProperty::Ref(name) => PropertyValue::VarRef(name.clone()),
        }
    }
}

pub fn cell_style_editor(ui: &mut Ui, style: &mut CellStyleDef) {
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
