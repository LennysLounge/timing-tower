use backend::{
    style::{
        iterator::{NodeIteratorMut, NodeMut},
        variables::{condition::Condition, fixed_value::FixedValue, map::Map, VariableBehavior},
        StyleDefinition, StyleNode,
    },
    value_types::ValueType,
};
use bevy_egui::egui::{ComboBox, DragValue, ScrollArea, Ui};
use rand::{seq::IteratorRandom, thread_rng};
use unified_sim_model::{games::dummy::DummyCommands, Adapter, GameAdapterCommand};
use uuid::Uuid;

use crate::{
    command::{
        adapter_command::AdapterCommand,
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    reference_store::ReferenceStore,
    ui::combo_box::LComboBox,
};

use self::property::PropertyEditor;

mod cell;
mod property;
mod variable;

pub fn property_editor(
    ui: &mut Ui,
    selected_id: &mut Option<Uuid>,
    style: &mut StyleDefinition,
    reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
    game_adapter: &Adapter,
) {
    let Some(selected_id) = selected_id else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            style.as_node_mut().search_mut(*&selected_id, |node| {
                edit_node(ui, node, reference_store, game_adapter, undo_redo_manager);
            });
        });
}

pub fn edit_node(
    ui: &mut Ui,
    node: NodeMut,
    reference_store: &ReferenceStore,
    game_adapter: &Adapter,
    undo_redo_manager: &mut UndoRedoManager,
) {
    match node {
        NodeMut::TimingTower(tower) => {
            let mut tower_edit = tower.clone();
            let edit_result = cell::cell_property_editor(ui, &mut tower_edit.cell, reference_store);
            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(tower.id, tower_edit, widget_id));
            }
        }

        NodeMut::TimingTowerRow(row) => {
            let mut edit_result = EditResult::None;

            ui.label("Row offset:");
            ui.horizontal(|ui| {
                ui.label("Offset x:");
                edit_result |= ui
                    .add(PropertyEditor::new(&mut row.row_offset.x, reference_store))
                    .into();
            });
            ui.horizontal(|ui| {
                ui.label("Offset y:");
                edit_result |= ui
                    .add(PropertyEditor::new(&mut row.row_offset.y, reference_store))
                    .into();
            });
            ui.separator();
            edit_result |= cell::cell_property_editor(ui, &mut row.cell, reference_store);

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(row.id, row.clone(), widget_id));
            }
        }

        NodeMut::TimingTowerColumn(column) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut column.name).into();
            ui.separator();
            edit_result |= cell::cell_property_editor(ui, &mut column.cell, reference_store).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(column.id, column.clone(), widget_id));
            }
        }

        NodeMut::TimingTowerColumnFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        NodeMut::Asset(asset) => {
            let mut edit_result = EditResult::None;

            ui.label("Name");
            edit_result |= ui.text_edit_singleline(&mut asset.name).into();
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Type:");
                edit_result |= ui
                    .add(
                        LComboBox::new(&mut asset.value_type)
                            .add_option(ValueType::Texture, "Image")
                            .add_option(ValueType::Font, "Font"),
                    )
                    .into();
            });
            ui.label("Path:");
            edit_result |= ui.text_edit_singleline(&mut asset.path).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(asset.id, asset.clone(), widget_id));
            }
        }

        NodeMut::AssetFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        NodeMut::Variable(variable) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut variable.name).into();

            ui.horizontal(|ui| {
                ui.label("Behavior:");
                ComboBox::new(ui.next_auto_id(), "")
                    .selected_text(match variable.behavior {
                        VariableBehavior::FixedValue(_) => "Fixed value",
                        VariableBehavior::Condition(_) => "Condition",
                        VariableBehavior::Map(_) => "Map",
                    })
                    .show_ui(ui, |ui| {
                        let is_fixed_value =
                            matches!(variable.behavior, VariableBehavior::FixedValue(_));
                        let res = ui.selectable_label(is_fixed_value, "Fixed value");
                        if res.clicked() && !is_fixed_value {
                            variable.behavior = VariableBehavior::FixedValue(FixedValue::default());
                            edit_result |= EditResult::FromId(res.id);
                        }

                        let is_condition =
                            matches!(variable.behavior, VariableBehavior::Condition(_));
                        let res = ui.selectable_label(is_condition, "Condition");
                        if res.clicked() && !is_condition {
                            variable.behavior = VariableBehavior::Condition(Condition::default());
                            edit_result |= EditResult::FromId(res.id);
                        }

                        let is_map = matches!(variable.behavior, VariableBehavior::Map(_));
                        let res = ui.selectable_label(is_map, "Map");
                        if res.clicked() && !is_map {
                            variable.behavior = VariableBehavior::Map(Map::default());
                            edit_result |= EditResult::FromId(res.id);
                        }
                    });
            });
            ui.separator();
            edit_result |= match &mut variable.behavior {
                VariableBehavior::FixedValue(value) => {
                    if variable::fixed_value::property_editor(ui, value, reference_store) {
                        EditResult::FromId(ui.make_persistent_id("Fixed_value_edit"))
                    } else {
                        EditResult::None
                    }
                }
                VariableBehavior::Condition(value) => {
                    if variable::condition::property_editor(ui, value, reference_store) {
                        EditResult::FromId(ui.make_persistent_id("condition_value_edit"))
                    } else {
                        EditResult::None
                    }
                }
                VariableBehavior::Map(value) => {
                    if variable::map::property_editor(ui, value, reference_store) {
                        EditResult::FromId(ui.make_persistent_id("map_value_edit"))
                    } else {
                        EditResult::None
                    }
                }
            };

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(
                    variable.id,
                    variable.clone(),
                    widget_id,
                ));
            }
        }

        NodeMut::VariableFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        NodeMut::Scene(scene) => {
            let mut edit_result = EditResult::None;

            ui.label("Prefered size:");
            ui.horizontal(|ui| {
                ui.label("width:");
                edit_result |= ui.add(DragValue::new(&mut scene.prefered_size.x)).into();
            });
            ui.horizontal(|ui| {
                ui.label("height:");
                edit_result |= ui.add(DragValue::new(&mut scene.prefered_size.y)).into();
            });
            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(scene.id, scene.clone(), widget_id));
            }

            ui.separator();
            if ui.button("Change focus to random entry").clicked() {
                let model = game_adapter
                    .model
                    .read()
                    .expect("Cannot lock model for reading");
                if let Some(random_entry) = model
                    .current_session()
                    .and_then(|session| session.entries.values().choose(&mut thread_rng()))
                {
                    undo_redo_manager.queue(AdapterCommand {
                        command: unified_sim_model::AdapterCommand::FocusOnCar(random_entry.id),
                    });
                }
            }
            ui.horizontal(|ui| {
                ui.label("Set entry amount:");
                let amount_id = ui.make_persistent_id("entry_amount");
                let mut amount = ui
                    .data_mut(|d| d.get_persisted(amount_id))
                    .unwrap_or(10usize);
                ui.add(DragValue::new(&mut amount).clamp_range(1..=usize::MAX));
                ui.data_mut(|d| d.insert_persisted(amount_id, amount));
                if ui.button("set").clicked() {
                    undo_redo_manager.queue(AdapterCommand {
                        command: unified_sim_model::AdapterCommand::Game(
                            GameAdapterCommand::Dummy(DummyCommands::SetEntryAmount(amount)),
                        ),
                    });
                }
            });
        }

        NodeMut::ClipArea(clip_area) => {
            let data = &mut clip_area.data;
            let mut edit_result = EditResult::None;

            ui.horizontal(|ui| {
                ui.label("Layer:");
                let res = ui.add(DragValue::new(&mut data.render_layer).clamp_range(0..=31));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Pos x:");
                let res = ui.add(PropertyEditor::new(&mut data.pos.x, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Pos y:");
                let res = ui.add(PropertyEditor::new(&mut data.pos.y, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Pos z:");
                let res = ui.add(PropertyEditor::new(&mut data.pos.z, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Width:");
                let res = ui.add(PropertyEditor::new(&mut data.size.x, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                let res = ui.add(PropertyEditor::new(&mut data.size.y, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("Skew:");
                let res = ui.add(PropertyEditor::new(&mut data.skew, reference_store));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.label("Rounding:");
            ui.horizontal(|ui| {
                ui.label("top left:");
                let res = ui.add(PropertyEditor::new(
                    &mut data.rounding.top_left,
                    reference_store,
                ));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("top right:");
                let res = ui.add(PropertyEditor::new(
                    &mut data.rounding.top_right,
                    reference_store,
                ));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("bottom right:");
                let res = ui.add(PropertyEditor::new(
                    &mut data.rounding.bot_right,
                    reference_store,
                ));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
            ui.horizontal(|ui| {
                ui.label("bottom left:");
                let res = ui.add(PropertyEditor::new(
                    &mut data.rounding.bot_left,
                    reference_store,
                ));
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(
                    *clip_area.id(),
                    clip_area.clone(),
                    widget_id,
                ));
            }
        }
        NodeMut::Style(_) => (),
    }
}
