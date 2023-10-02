use bevy_egui::egui::{ComboBox, Ui};
use serde::{Deserialize, Serialize};
use tree_view::{TreeNode, TreeNodeConverstions};
use uuid::Uuid;

use crate::variable_repo::{VariableDefinition, VariableId, VariableRepo};

use self::{condition::Condition, fixed_value::FixedValue};

pub mod condition;
pub mod fixed_value;

#[derive(Serialize, Deserialize, Clone)]
pub struct Variables {
    pub id: Uuid,
    pub vars: Vec<VariableBehavior>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableBehavior {
    FixedValue(FixedValue),
    Condition(Condition),
}

impl VariableDefinition for VariableBehavior {
    fn as_variable_source(&self) -> crate::variable_repo::VariableSource {
        match self {
            VariableBehavior::FixedValue(o) => o.as_variable_source(),
            VariableBehavior::Condition(o) => o.as_variable_source(),
        }
    }

    fn get_variable_id(&self) -> &crate::variable_repo::VariableId {
        match self {
            VariableBehavior::FixedValue(o) => o.get_variable_id(),
            VariableBehavior::Condition(o) => o.get_id(),
        }
    }
}

impl TreeNode for Variables {
    fn is_directory(&self) -> bool {
        true
    }

    fn show_label(&self, ui: &mut bevy_egui::egui::Ui) {
        ui.label("Variables");
    }

    fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        self.vars.iter().map(|v| v.as_dyn()).collect()
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        self.vars.iter_mut().map(|v| v.as_dyn_mut()).collect()
    }

    fn remove(&mut self, _id: &uuid::Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}

impl TreeNode for VariableBehavior {
    fn is_directory(&self) -> bool {
        false
    }

    fn show_label(&self, ui: &mut bevy_egui::egui::Ui) {
        ui.label(&VariableDefinition::get_variable_id(self).name);
    }

    fn get_id(&self) -> &Uuid {
        &self.get_variable_id().id
    }

    fn get_children(&self) -> Vec<&dyn TreeNode> {
        vec![]
    }

    fn get_children_mut(&mut self) -> Vec<&mut dyn TreeNode> {
        vec![]
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn insert(&mut self, _drop_action: &tree_view::DropAction, _node: Box<dyn std::any::Any>) {}
}

impl VariableBehavior {
    fn get_id_mut(&mut self) -> &mut VariableId {
        match self {
            VariableBehavior::FixedValue(f) => f.get_id_mut(),
            VariableBehavior::Condition(c) => c.get_id_mut(),
        }
    }
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
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
                        *self = VariableBehavior::FixedValue(FixedValue::from_id(
                            self.get_variable_id().clone(),
                        ))
                    }

                    let is_condition = matches!(self, VariableBehavior::Condition(_));
                    if ui.selectable_label(is_condition, "Condition").clicked() && !is_condition {
                        *self = VariableBehavior::Condition(Condition::from_id(
                            self.get_variable_id().clone(),
                        ))
                    }
                });
        });
        ui.separator();

        match self {
            VariableBehavior::FixedValue(v) => v.property_editor(ui),
            VariableBehavior::Condition(v) => v.property_editor(ui, vars),
        }
    }
}
