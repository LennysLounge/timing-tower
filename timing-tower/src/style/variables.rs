use bevy_egui::egui::ComboBox;
use serde::{Deserialize, Serialize};
use tree_view::{DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::variable_repo::{VariableDefinition, VariableId};

use self::{condition::Condition, fixed_value::FixedValue};

use super::{
    folder::{Folder, FolderActions},
    StyleTreeNode, StyleTreeUi, TreeViewAction,
};

pub mod condition;
pub mod fixed_value;

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
            VariableBehavior::Condition(o) => o.get_variable_id(),
        }
    }
}

impl StyleTreeUi for VariableBehavior {
    fn property_editor(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        vars: &crate::variable_repo::VariableRepo,
    ) {
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
            VariableBehavior::FixedValue(o) => o.property_editor(ui),
            VariableBehavior::Condition(o) => o.property_editor(ui, vars),
        }
    }

    fn tree_view(&mut self, tree_ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let res = TreeViewBuilder::leaf(self.get_variable_id().id).show(tree_ui, |ui| {
            ui.label(&self.get_variable_id().name);
        });
        res.response.context_menu(|ui| {
            if ui.button("add variable").clicked() {
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(VariableBehavior::new()),
                    position: DropPosition::After(*self.id()),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(Folder::<VariableBehavior>::new()),
                    position: DropPosition::After(*self.id()),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                actions.push(TreeViewAction::Remove { node: *self.id() });
                ui.close_menu();
            }
        });
    }
}

impl StyleTreeNode for VariableBehavior {
    fn id(&self) -> &Uuid {
        &self.get_variable_id().id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        Vec::new()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        Vec::new()
    }

    fn can_insert(&self, _node: &dyn std::any::Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn std::any::Any>, _position: &tree_view::DropPosition) {}
}

impl FolderActions for VariableBehavior {
    type FolderType = Self;

    fn context_menu(
        ui: &mut bevy_egui::egui::Ui,
        folder: &super::folder::Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add variable").clicked() {
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(VariableBehavior::new()),
                position: DropPosition::First,
            });
            ui.close_menu();
        }
        if ui.button("add group").clicked() {
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(Folder::<VariableBehavior>::new()),
                position: DropPosition::First,
            });
            ui.close_menu();
        }
        if ui.button("delete").clicked() {
            actions.push(TreeViewAction::Remove { node: *folder.id() });
            ui.close_menu();
        }
    }
}

impl VariableBehavior {
    fn new() -> Self {
        VariableBehavior::FixedValue(FixedValue::default())
    }
    fn get_id_mut(&mut self) -> &mut VariableId {
        match self {
            VariableBehavior::FixedValue(o) => o.get_id_mut(),
            VariableBehavior::Condition(o) => o.get_id_mut(),
        }
    }
}
