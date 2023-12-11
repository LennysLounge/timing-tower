use std::mem::discriminant;

use bevy_egui::egui::{Response, Ui};
use tree_view::DropPosition;
use unified_sim_model::model::Entry;
use uuid::Uuid;

use crate::reference_store::{IntoProducerData, ProducerData};
use backend::{
    style::{
        folder::Folder,
        variables::{VariableBehavior, VariableDefinition},
    },
    value_store::{ValueProducer, ValueStore},
};

use super::{folder::FolderActions, StyleTreeNode, StyleTreeUi, TreeViewAction};

pub mod condition;
pub mod fixed_value;
pub mod map;

impl IntoProducerData for VariableDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id,
            name: self.name.clone(),
            value_type: match &self.behavior {
                VariableBehavior::FixedValue(o) => o.output_type(),
                VariableBehavior::Condition(o) => o.output_type(),
                VariableBehavior::Map(o) => o.output_type(),
            },
        }
    }
}

impl StyleTreeUi for VariableDefinition {}

impl StyleTreeNode for VariableDefinition {
    fn id(&self) -> &Uuid {
        &self.id
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

impl FolderActions for VariableDefinition {
    type FolderType = Self;

    fn context_menu(
        ui: &mut bevy_egui::egui::Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
        if ui.button("add variable").clicked() {
            let var = VariableDefinition::new();
            actions.push(TreeViewAction::Select { node: *var.id() });
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(var),
                position: DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("add group").clicked() {
            let new_folder = Folder::<VariableDefinition>::new();
            actions.push(TreeViewAction::Select {
                node: new_folder.id,
            });
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(new_folder),
                position: DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("delete").clicked() {
            actions.push(TreeViewAction::Remove { node: *folder.id() });
            ui.close_menu();
        }
    }
}

pub struct StaticValueProducer<T>(pub T);
impl<T> ValueProducer<T> for StaticValueProducer<T>
where
    T: Clone,
{
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<T> {
        Some(self.0.clone())
    }
}

trait EguiComboBoxExtension {
    /// Shows the combobox with one entry for each variant.
    /// Compares variants based on their discriminants and not PartialEq.
    fn choose<T>(self, ui: &mut Ui, current: &mut T, other: Vec<(T, &str)>) -> Response;
}
impl EguiComboBoxExtension for bevy_egui::egui::ComboBox {
    fn choose<T>(self, ui: &mut Ui, current: &mut T, other: Vec<(T, &str)>) -> Response {
        let mut changed = false;
        let mut res = self
            .selected_text({
                other
                    .iter()
                    .find_map(|(other, name)| {
                        (discriminant(current) == discriminant(other)).then_some(*name)
                    })
                    .unwrap_or("Not Found")
            })
            .show_ui(ui, |ui| {
                for (other, name) in other {
                    let is_same = discriminant(current) == discriminant(&other);
                    if ui.selectable_label(is_same, name).clicked() && !is_same {
                        *current = other;
                        changed = true;
                    }
                }
            })
            .response;
        if changed {
            res.mark_changed();
        }
        res
    }
}
