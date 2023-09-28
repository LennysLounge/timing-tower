use bevy::prelude::{Color, Resource};
use bevy_egui::egui::{self, ComboBox, Response, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::{Reference, Variable, VariableId, VariableRepo};

use super::{
    properties::{
        BooleanProperty, ColorProperty, NumberProperty, TextProperty, Vec2Property, Vec3Property,
    },
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
    pub text: TextProperty,
    pub text_color: ColorProperty,
    pub text_size: NumberProperty,
    pub color: ColorProperty,
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

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableDef {
    Number(f32),
    Text(String),
    Color(Color),
}

impl StyleElement for RootElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        self.vars.element_tree(ui, selected_element);
        ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());
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
        }
    }
}

impl CellElement {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            self.visible.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Text:");
            self.text.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Text color:");
            self.text_color.editor(ui, vars);
        });
        ui.horizontal(|ui| {
            ui.label("Text size:");
            self.text_size.editor(ui, vars);
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

pub fn reference_editor(
    ui: &mut Ui,
    vars: &VariableRepo,
    reference: &mut Reference,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let var_name = match vars.get_var_def(reference) {
        Some(def) => def.name.as_str(),
        None => "Ref",
    };
    let popup_button = ui.button(var_name);
    reference_popup(ui, &popup_button, vars, use_type)
}

pub fn reference_editor_small(
    ui: &mut Ui,
    vars: &VariableRepo,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let popup_button = ui.button("R");
    reference_popup(ui, &popup_button, vars, use_type)
}

fn reference_popup(
    ui: &mut Ui,
    button_response: &Response,
    vars: &VariableRepo,
    use_type: impl Fn(&VariableId) -> bool,
) -> Option<Reference> {
    let popup_id = ui.next_auto_id();
    if button_response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }
    egui::popup::popup_below_widget(ui, popup_id, &button_response, |ui| {
        ui.set_min_width(200.0);
        let mut color_vars: Vec<&Variable> =
            vars.vars.values().filter(|var| use_type(&var.id)).collect();
        color_vars.sort_by(|v1, v2| v1.id.name.cmp(&v2.id.name));

        let mut result = None;
        for var in color_vars {
            if ui.selectable_label(false, &var.id.name).clicked() && result.is_none() {
                result = Some(var.get_ref());
                ui.memory_mut(|mem| mem.close_popup());
            }
        }
        result
    })
    .flatten()
}
