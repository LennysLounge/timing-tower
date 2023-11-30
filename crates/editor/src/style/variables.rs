use std::mem::discriminant;

use bevy_egui::egui::{ComboBox, Response, Ui};
use serde::{Deserialize, Serialize};
use tree_view::{DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::{
    reference_store::ReferenceStore,
    value_store::{AssetId, IntoValueProducer, TypedValueProducer},
};

use self::{condition::Condition, fixed_value::FixedValue, map::Map};

use super::{
    folder::{Folder, FolderActions},
    StyleTreeNode, StyleTreeUi, TreeViewAction,
};

pub mod condition;
pub mod fixed_value;
pub mod map;

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableBehavior {
    FixedValue(FixedValue),
    Condition(Condition),
    Map(Map),
}

impl IntoValueProducer for VariableBehavior {
    fn asset_id(&self) -> &crate::value_store::AssetId {
        match self {
            VariableBehavior::FixedValue(o) => o.asset_id(),
            VariableBehavior::Condition(o) => o.asset_id(),
            VariableBehavior::Map(o) => o.asset_id(),
        }
    }

    fn get_value_producer(&self) -> TypedValueProducer {
        match self {
            VariableBehavior::FixedValue(o) => o.get_value_producer(),
            VariableBehavior::Condition(o) => o.get_value_producer(),
            VariableBehavior::Map(o) => o.get_value_producer(),
        }
    }
}

impl StyleTreeUi for VariableBehavior {
    fn property_editor(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        asset_repo: &ReferenceStore,
    ) -> bool {
        let mut changed = false;

        ui.label("Name:");
        changed |= ui
            .text_edit_singleline(&mut self.get_id_mut().name)
            .changed();

        ui.horizontal(|ui| {
            ui.label("Behavior:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match self {
                    VariableBehavior::FixedValue(_) => "Fixed value",
                    VariableBehavior::Condition(_) => "Condition",
                    VariableBehavior::Map(_) => "Map",
                })
                .show_ui(ui, |ui| {
                    let is_fixed_value = matches!(self, VariableBehavior::FixedValue(_));
                    if ui.selectable_label(is_fixed_value, "Fixed value").clicked()
                        && !is_fixed_value
                    {
                        *self = VariableBehavior::FixedValue(FixedValue::from_id(
                            self.asset_id().clone(),
                        ));
                        changed |= true;
                    }

                    let is_condition = matches!(self, VariableBehavior::Condition(_));
                    if ui.selectable_label(is_condition, "Condition").clicked() && !is_condition {
                        *self = VariableBehavior::Condition(Condition::from_id(
                            self.asset_id().clone(),
                        ));
                        changed = true;
                    }
                    let is_map = matches!(self, VariableBehavior::Map(_));
                    if ui.selectable_label(is_map, "Map").clicked() && !is_map {
                        *self = VariableBehavior::Map(Map::from_id(self.asset_id().clone()));
                        changed = true;
                    }
                });
        });

        ui.separator();
        changed |= match self {
            VariableBehavior::FixedValue(o) => o.property_editor(ui),
            VariableBehavior::Condition(o) => o.property_editor(ui, asset_repo),
            VariableBehavior::Map(o) => o.property_editor(ui, asset_repo),
        };
        changed
    }

    fn tree_view(&mut self, tree_ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let res = TreeViewBuilder::leaf(self.asset_id().id).show(tree_ui, |ui| {
            ui.label(&self.asset_id().name);
        });
        res.response.context_menu(|ui| {
            if ui.button("add variable").clicked() {
                let var = VariableBehavior::new();
                actions.push(TreeViewAction::Select { node: *var.id() });
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(var),
                    position: DropPosition::After(*self.id()),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                let folder = Folder::<VariableBehavior>::new();
                actions.push(TreeViewAction::Select { node: folder.id });
                actions.push(TreeViewAction::Insert {
                    target: tree_ui.parent_id.unwrap(),
                    node: Box::new(folder),
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
        &self.asset_id().id
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
            let var = VariableBehavior::new();
            actions.push(TreeViewAction::Select { node: *var.id() });
            actions.push(TreeViewAction::Insert {
                target: *folder.id(),
                node: Box::new(var),
                position: DropPosition::Last,
            });
            ui.close_menu();
        }
        if ui.button("add group").clicked() {
            let new_folder = Folder::<VariableBehavior>::new();
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

impl VariableBehavior {
    fn new() -> Self {
        VariableBehavior::FixedValue(FixedValue::default())
    }
    fn get_id_mut(&mut self) -> &mut AssetId {
        match self {
            VariableBehavior::FixedValue(o) => o.get_id_mut(),
            VariableBehavior::Condition(o) => o.get_id_mut(),
            VariableBehavior::Map(o) => o.get_id_mut(),
        }
    }
}

fn variant_checkbox<T: Clone>(ui: &mut Ui, thing: &mut T, other_things: &[(&T, &str)]) -> Response {
    let mut changed = false;
    let mut res = ComboBox::new(ui.next_auto_id(), "")
        .selected_text({
            other_things
                .iter()
                .find_map(|(other, name)| {
                    (discriminant(thing) == discriminant(other)).then_some(*name)
                })
                .unwrap_or("Not Found")
        })
        .show_ui(ui, |ui| {
            for (other, name) in other_things {
                let is_same = discriminant(thing) == discriminant(other);
                if ui.selectable_label(is_same, *name).clicked() && !is_same {
                    *thing = (*other).clone();
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
