use backend::{
    style::{
        variables::{condition::Condition, fixed_value::FixedValue, map::Map, VariableBehavior},
        StyleItem,
    },
    tree_iterator::TreeIteratorMut,
    value_types::ValueType,
};
use bevy_egui::egui::{DragValue, ScrollArea, Ui};
use unified_sim_model::Adapter;

use crate::{
    reference_store::ReferenceStore,
    ui::{
        combo_box::LComboBox, EditResult, EditorState, EditorStyle, StyleItemSelection, UiMessage, UiMessages
    },
};

use self::graphic::graphic_property_editor;

use super::secondary_editor::ui_split;

mod graphic;
pub mod property;
mod variable;

pub(super) fn editor(
    ui: &mut Ui,
    messages: &mut UiMessages,
    editor_style: &mut EditorStyle,
    editor_state: &mut EditorState,
    reference_store: &ReferenceStore,
    game_adapter: Option<&Adapter>,
) {
    let Some(style_item) = editor_state.style_item_tree_state.selected() else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            editor_style.0.search_mut(style_item, |node| {
                edit_node(
                    ui,
                    node,
                    messages,
                    reference_store,
                    editor_state,
                    game_adapter,
                );
            });
        });
}

fn edit_node(
    ui: &mut Ui,
    node: &mut StyleItem,
    messages: &mut UiMessages,
    reference_store: &ReferenceStore,
    editor_state: &mut EditorState,
    game_adapter: Option<&Adapter>,
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
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
            }
        }

        StyleItem::AssetFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
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
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
            }
        }

        StyleItem::VariableFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
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
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
            }

            ui.separator();
            let connected = game_adapter.is_some_and(|adapter| !adapter.is_finished());
            ui.add_enabled_ui(connected, |ui| {
                if ui.button("Change focus to random entry").clicked() {
                    messages.push(UiMessage::GameAdapterSelectRandomEntry);
                }
            });
            if ui.button("Set Race").clicked() {
                messages.push(UiMessage::GameAdapterDummySetSessionType(
                    unified_sim_model::model::SessionType::Race,
                ));
            }
            if ui.button("Set Quali").clicked() {
                messages.push(UiMessage::GameAdapterDummySetSessionType(
                    unified_sim_model::model::SessionType::Qualifying,
                ));
            }
            if ui.button("Set Practice").clicked() {
                messages.push(UiMessage::GameAdapterDummySetSessionType(
                    unified_sim_model::model::SessionType::Practice,
                ));
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
                    messages.push(UiMessage::GameAdapterDummySetEntryAmount(amount));
                }
            });
        }

        StyleItem::Graphic(graphic) => {
            let style_item_selection = editor_state
                .style_item_selection_data
                .entry(graphic.id)
                .or_insert(StyleItemSelection::default());

            ui.push_id(graphic.id, |ui| {
                graphic_property_editor(ui, graphic, messages, style_item_selection);
            });
        }
        StyleItem::GraphicFolder(folder) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut folder.name).into();

            if let EditResult::FromId(widget_id) = edit_result {
                messages.push(UiMessage::StyleItemEdit {
                    widget_id,
                    item: node.clone(),
                });
            }
        }

        StyleItem::Style(_) => (),
    }
}
