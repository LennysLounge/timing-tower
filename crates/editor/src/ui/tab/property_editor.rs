use backend::{
    exact_variant::ExactVariant,
    graphic::GraphicStates,
    style::{
        graphic::graphic_items::GraphicItemId,
        variables::{condition::Condition, fixed_value::FixedValue, map::Map, VariableBehavior},
        StyleDefinition, StyleId, StyleItem,
    },
    tree_iterator::TreeIteratorMut,
    value_types::ValueType,
};
use bevy_egui::egui::{self, DragValue, ScrollArea, Ui};
use rand::{seq::IteratorRandom, thread_rng};
use unified_sim_model::{games::dummy::DummyCommands, Adapter, GameAdapterCommand};

use crate::{
    command::{
        adapter_command::AdapterCommand,
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    reference_store::ReferenceStore,
    ui::combo_box::LComboBox,
};

use self::graphic::graphic_property_editor;

use super::element_editor::ui_split;

mod graphic;
pub mod property;
mod variable;

pub fn property_editor(
    ui: &mut Ui,
    selected_id: &mut Option<StyleId>,
    secondary_selection: &mut Option<GraphicItemId>,
    style: &mut ExactVariant<StyleItem, StyleDefinition>,
    reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
    game_adapter: Option<&Adapter>,
    graphic_states: &mut GraphicStates,
) {
    let Some(selected_id) = selected_id else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            style.search_mut(*selected_id, |node| {
                edit_node(
                    ui,
                    node,
                    reference_store,
                    game_adapter,
                    undo_redo_manager,
                    secondary_selection,
                    graphic_states,
                );
            });
        });
}

pub fn edit_node(
    ui: &mut Ui,
    node: &mut StyleItem,
    reference_store: &ReferenceStore,
    game_adapter: Option<&Adapter>,
    undo_redo_manager: &mut UndoRedoManager,
    graphic_item_selection: &mut Option<GraphicItemId>,
    graphic_states: &mut GraphicStates,
) {
    match node {
        StyleItem::Asset(asset) => {
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

        StyleItem::AssetFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        StyleItem::Variable(variable) => {
            let mut edit_result = EditResult::None;

            ui_split(ui, "Name", |ui| {
                edit_result |= ui.text_edit_singleline(&mut variable.name).into();
            });
            ui_split(ui, "Behavior", |ui| {
                edit_result |= ui
                    .add(
                        LComboBox::new_comparable(&mut variable.behavior, |a, b| {
                            std::mem::discriminant(a) == std::mem::discriminant(b)
                        })
                        .add_option(
                            VariableBehavior::FixedValue(FixedValue::default()),
                            "Fixed value",
                        )
                        .add_option(
                            VariableBehavior::Condition(Condition::default()),
                            "Condition",
                        )
                        .add_option(VariableBehavior::Map(Map::default()), "Map"),
                    )
                    .into();
            });

            edit_result |= match &mut variable.behavior {
                VariableBehavior::FixedValue(value) => {
                    variable::fixed_value::property_editor(ui, value, reference_store)
                }
                VariableBehavior::Condition(value) => {
                    variable::condition::property_editor(ui, value, reference_store)
                }
                VariableBehavior::Map(value) => {
                    variable::map::property_editor(ui, value, reference_store)
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

        StyleItem::VariableFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        StyleItem::Scene(scene) => {
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
            match game_adapter {
                Some(game_adapter) => {
                    if ui.button("Change focus to random entry").clicked() {
                        let model = game_adapter.model.read_raw();
                        if let Some(random_entry) = model
                            .current_session()
                            .and_then(|session| session.entries.values().choose(&mut thread_rng()))
                        {
                            undo_redo_manager.queue(AdapterCommand {
                                command: unified_sim_model::AdapterCommand::FocusOnCar(
                                    random_entry.id,
                                ),
                            });
                        }
                    }
                }
                None => {
                    ui.add_enabled(false, egui::Button::new("Change focus to random entry"));
                }
            }
            if ui.button("Set Race").clicked() {
                undo_redo_manager.queue(AdapterCommand {
                    command: unified_sim_model::AdapterCommand::Game(GameAdapterCommand::Dummy(
                        DummyCommands::SetSessionType(unified_sim_model::model::SessionType::Race),
                    )),
                });
            }
            if ui.button("Set Quali").clicked() {
                undo_redo_manager.queue(AdapterCommand {
                    command: unified_sim_model::AdapterCommand::Game(GameAdapterCommand::Dummy(
                        DummyCommands::SetSessionType(
                            unified_sim_model::model::SessionType::Qualifying,
                        ),
                    )),
                });
            }
            if ui.button("Set Practice").clicked() {
                undo_redo_manager.queue(AdapterCommand {
                    command: unified_sim_model::AdapterCommand::Game(GameAdapterCommand::Dummy(
                        DummyCommands::SetSessionType(
                            unified_sim_model::model::SessionType::Practice,
                        ),
                    )),
                });
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

        StyleItem::Graphic(component) => {
            ui.push_id(component.id, |ui| {
                graphic_property_editor(
                    ui,
                    component,
                    graphic_item_selection,
                    undo_redo_manager,
                    graphic_states,
                );
            });
        }
        StyleItem::GraphicFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
            }
        }

        StyleItem::Style(_) => (),
    }
}
