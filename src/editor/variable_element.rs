use bevy_egui::egui::{collapsing_header::CollapsingState, ComboBox, Sense, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::variable_repo::{VariableId, VariableRepo, VariableSource};

use self::{condition::Condition, fixed_value::FixedValue};

use super::style_elements::StyleElement;

pub mod condition;
pub mod fixed_value;

#[derive(Serialize, Deserialize, Clone)]
pub struct VariablesElement {
    pub vars: Vec<VariableBehavior>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableBehavior {
    FixedValue(FixedValue),
    Condition(Condition),
}

impl StyleElement for VariablesElement {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let id = ui.next_auto_id();
        let (toggle, header_res, _) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| ui.label("Variables"))
            .body(|ui| {
                if ui.button("+ Add variable").clicked() {
                    let var = VariableBehavior::FixedValue(FixedValue::default());
                    *selected_element = Some(var.get_id().id.clone());
                    self.vars.push(var);
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

impl StyleElement for VariableBehavior {
    fn element_tree(&mut self, ui: &mut Ui, selected_element: &mut Option<Uuid>) {
        let is_selected = selected_element.is_some_and(|uuid| uuid == self.get_id().id);
        if ui
            .selectable_label(is_selected, &self.get_id().name)
            .clicked()
        {
            *selected_element = Some(self.get_id().id.clone());
        }
    }

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleElement> {
        (&self.get_id().id == id).then_some(self as &mut dyn StyleElement)
    }

    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.get_id_mut().name);

        ui.horizontal(|ui| {
            ui.label("Behavior:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self {
                    VariableBehavior::FixedValue(_) => "Fixed value",
                    VariableBehavior::Condition(_) => "Condition",
                })
                .show_ui(ui, |ui| {
                    let is_fixed_value = matches!(self, VariableBehavior::FixedValue(_));
                    if ui.selectable_label(is_fixed_value, "Fixed value").clicked()
                        && !is_fixed_value
                    {
                        *self =
                            VariableBehavior::FixedValue(FixedValue::from_id(self.get_id().clone()))
                    }

                    let is_condition = matches!(self, VariableBehavior::Condition(_));
                    if ui.selectable_label(is_condition, "Condition").clicked() && !is_condition {
                        *self =
                            VariableBehavior::Condition(Condition::from_id(self.get_id().clone()))
                    }
                });
        });
        ui.separator();

        self.property_editor(ui, vars);
    }
}

impl VariableBehavior {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        match self {
            VariableBehavior::FixedValue(v) => v.property_editor(ui),
            VariableBehavior::Condition(v) => v.property_editor(ui, vars),
        }
    }

    pub fn as_variable_source(&self) -> VariableSource {
        match self {
            VariableBehavior::FixedValue(v) => v.as_variable_source(),
            VariableBehavior::Condition(v) => v.as_variable_source(),
        }
    }

    pub fn get_id(&self) -> &VariableId {
        match self {
            VariableBehavior::FixedValue(f) => f.get_id(),
            VariableBehavior::Condition(c) => c.get_id(),
        }
    }
    pub fn get_id_mut(&mut self) -> &mut VariableId {
        match self {
            VariableBehavior::FixedValue(f) => f.get_id_mut(),
            VariableBehavior::Condition(c) => c.get_id_mut(),
        }
    }
}
