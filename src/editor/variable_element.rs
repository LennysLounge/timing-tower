use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, Sense, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::{VariableRepo, VariableSource};

use self::{condition::Condition, fixed_value::FixedValue};

use super::style_elements::StyleElement;

pub mod condition;
pub mod fixed_value;

#[derive(Serialize, Deserialize, Clone)]
pub struct VariablesElement {
    pub vars: Vec<VariableDefinition>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VariableDefinition {
    pub id: Uuid,
    pub name: String,
    pub behavior: VariableBehavior,
    pub output_type: VariableOutputType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableBehavior {
    FixedValue(FixedValue),
    Condition(Condition),
    #[serde(skip)]
    Game,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum VariableOutputType {
    Number,
    Text,
    Color,
}

impl StyleElement for VariablesElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let id = ui.next_auto_id();
        let (toggle, header_res, _) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| ui.label("Variables"))
            .body(|ui| {
                if ui.button("+ Add variable").clicked() {
                    let uuid = Uuid::new_v4();
                    self.vars.push(VariableDefinition {
                        id: uuid.clone(),
                        name: "Variable".to_string(),
                        behavior: VariableBehavior::FixedValue(FixedValue::StaticNumber(12.0)),
                        output_type: VariableOutputType::Number,
                    });
                    *selected_element = Some(uuid);
                }
                for var in self.vars.iter_mut() {
                    var.element_tree(ui, selected_element);
                }
            });
        if ui
            .interact(header_res.response.rect, id, Sense::click())
            .clicked()
            && !toggle.clicked()
        {
            if let Some(mut state) = CollapsingState::load(ui.ctx(), id) {
                state.toggle(ui);
                state.store(ui.ctx());
            }
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        self.vars.iter_mut().find_map(|v| v.find_mut(id))
    }

    fn property_editor(&mut self, _ui: &mut Ui, _vars: &VariableRepo) {}
}

impl StyleElement for VariableDefinition {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid == self.id);
        if ui.selectable_label(is_selected, &self.name).clicked() {
            *selected_element = Some(self.id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        (&self.id == id).then_some(self as &mut dyn StyleElement)
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
        ui.horizontal(|ui| {
            ui.label("Type:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.output_type {
                    VariableOutputType::Number => "Number",
                    VariableOutputType::Text => "Text",
                    VariableOutputType::Color => "Color",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.output_type,
                        VariableOutputType::Number,
                        "Number",
                    );
                    ui.selectable_value(&mut self.output_type, VariableOutputType::Text, "Text");
                    ui.selectable_value(&mut self.output_type, VariableOutputType::Color, "Color");
                });
        });
        ui.horizontal(|ui| {
            ui.label("Behavior:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self.behavior {
                    VariableBehavior::FixedValue(_) => "Fixed value",
                    VariableBehavior::Condition(_) => "Condition",
                    VariableBehavior::Game => unreachable!(),
                })
                .show_ui(ui, |ui| {
                    let is_fixed_value = matches!(self.behavior, VariableBehavior::FixedValue(_));
                    if ui.selectable_label(is_fixed_value, "Fixed value").clicked()
                        && !is_fixed_value
                    {
                        self.behavior = VariableBehavior::FixedValue(FixedValue::default())
                    }

                    let is_condition = matches!(self.behavior, VariableBehavior::Condition(_));
                    if ui.selectable_label(is_condition, "Condition").clicked() && !is_condition {
                        self.behavior = VariableBehavior::Condition(Condition::default())
                    }
                });
        });
        ui.separator();

        self.behavior.property_editor(ui, &self.output_type, vars);
    }
}

impl VariableBehavior {
    pub fn property_editor(
        &mut self,
        ui: &mut Ui,
        output_type: &VariableOutputType,
        vars: &VariableRepo,
    ) {
        match self {
            VariableBehavior::FixedValue(v) => v.property_editor(ui, output_type),
            VariableBehavior::Condition(v) => v.property_editor(ui, output_type, vars),
            VariableBehavior::Game => unreachable!(),
        }
    }

    pub fn as_variable_source(&self) -> VariableSource {
        match self {
            VariableBehavior::FixedValue(v) => v.as_variable_source(),
            VariableBehavior::Condition(v) => v.as_variable_source(),
            VariableBehavior::Game => unreachable!(),
        }
    }
}
