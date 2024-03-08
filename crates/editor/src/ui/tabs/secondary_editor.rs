use backend::{
    style::{
        graphic::{
            graphic_items::{
                cell::Cell,
                clip_area::ClipArea,
                driver_table::DriverTable,
                entry_context::{EntryContext, EntrySelection},
                root::Root,
                Attribute, GraphicItem,
            },
            GraphicDefinition, GraphicStateId, TEMPLATE_ID,
        },
        StyleItem,
    },
    tree_iterator::TreeIteratorMut,
};
use bevy_egui::egui::{
    self, vec2, CollapsingHeader, DragValue, Layout, ScrollArea, Ui, Widget, WidgetText,
};
use common::communication::TextAlignment;

use crate::{
    command::edit_property::EditResult,
    reference_store::ReferenceStore,
    ui::{
        combo_box::LComboBox, EditorState, EditorStyle, StyleItemSelection, UiMessage, UiMessages,
    },
};

use super::style_item::property::PropertyEditor;

pub(super) fn editor(
    ui: &mut Ui,
    messages: &mut UiMessages,
    editor_state: &mut EditorState,
    editor_style: &mut EditorStyle,
    reference_store: &ReferenceStore,
) {
    let Some(style_item_selection) = editor_state.style_item_tree_state.selected() else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            editor_style
                .0
                .search_mut(style_item_selection, |style_item| {
                    if let StyleItem::Graphic(graphic) = style_item {
                        graphic_editor(ui, graphic, messages, editor_state, reference_store);
                    }
                });
        });
}

fn graphic_editor(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    messages: &mut UiMessages,
    editor_state: &mut EditorState,
    reference_store: &ReferenceStore,
) {
    let mut edit_result = EditResult::None;

    let selection_data = editor_state
        .style_item_selection_data
        .entry(graphic.id)
        .or_insert(StyleItemSelection::default());

    // Editor for the the state.
    if let Some(selected_state) = selection_data.graphic_state_tree_state.selected() {
        if let Some(state) = graphic
            .states
            .iter_mut()
            .find(|state| state.id == selected_state)
        {
            ui.label("State name:");
            edit_result |= ui.text_edit_singleline(&mut state.name).into();
            ui.separator();
        }
    }

    // Editor for the graphic item
    if let Some(graphic_item_selection) = selection_data.graphic_item_tree_state.selected() {
        edit_result |= graphic
            .items
            .search_mut(graphic_item_selection, |item| {
                graphic_item_editor(
                    ui,
                    item,
                    reference_store,
                    selection_data
                        .graphic_state_tree_state
                        .selected()
                        .unwrap_or(TEMPLATE_ID),
                )
            })
            .unwrap_or_default();
    }
    // Copy the name of the root graphic item to the graphic to keep them synced.
    if matches!(edit_result, EditResult::FromId(_)) {
        graphic.name = graphic.items.name.clone();
    }

    if let EditResult::FromId(widget_id) = edit_result {
        messages.push(UiMessage::StyleItemEdit {
            widget_id,
            item: StyleItem::Graphic(graphic.clone()),
        });
    }
}

fn graphic_item_editor(
    ui: &mut Ui,
    item: &mut GraphicItem,
    reference_store: &ReferenceStore,
    state_id: GraphicStateId,
) -> EditResult {
    match item {
        GraphicItem::Root(root) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut root.name).into();
            ui.separator();
            edit_result |= root_editor(ui, root, state_id, reference_store);
            edit_result
        }
        GraphicItem::Cell(cell) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut cell.name).into();
            ui.separator();
            edit_result |= cell_property_editor(ui, cell, state_id, reference_store).into();
            edit_result
        }
        GraphicItem::ClipArea(clip_area) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut clip_area.name).into();
            ui.separator();
            edit_result |= clip_area_editor(ui, clip_area, state_id, reference_store);
            edit_result
        }
        GraphicItem::DriverTable(driver_table) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut driver_table.name).into();
            ui.separator();
            edit_result |= driver_table_editor(ui, driver_table, state_id, reference_store);
            edit_result
        }
        GraphicItem::EntryContext(entry_context) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut entry_context.name).into();
            ui.separator();
            edit_result |= entry_context_editor(ui, entry_context, state_id, reference_store);
            edit_result
        }
    }
}

pub fn ui_split(ui: &mut Ui, label: impl Into<WidgetText>, right: impl FnMut(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2((ui.available_width()) * 0.35, ui.spacing().interact_size.y),
            Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.add(egui::Label::new(label).truncate(true));
            },
        );
        ui.add_space(ui.spacing().item_spacing.x);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), ui.spacing().interact_size.y),
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            right,
        );
    });
}
fn ui_attribute<T: Clone>(
    ui: &mut Ui,
    attr: &mut Attribute<T>,
    state_id: GraphicStateId,
    mut add_content: impl FnMut(&mut Ui, &mut T),
) {
    ui.horizontal(|ui| {
        let mut enabled = attr.has_state(&state_id) || state_id == TEMPLATE_ID;
        if state_id == TEMPLATE_ID {
            // Add enough space to equal the checkbox.
            ui.add_space(ui.spacing().icon_width);
            ui.add_space(ui.spacing().item_spacing.x);
            ui.add_space(ui.spacing().item_spacing.x);
        } else {
            if ui.checkbox(&mut enabled, "").changed() {
                if enabled {
                    attr.add_state(state_id);
                } else {
                    attr.remove_state(&state_id);
                }
            }
        }

        ui.vertical(|ui| {
            ui.add_enabled_ui(enabled, |ui| {
                let attribute = if attr.has_state(&state_id) {
                    attr.get_state(&state_id).unwrap()
                } else {
                    attr.template_mut()
                };
                add_content(ui, attribute);
            });
        });
    });
}

pub fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    state_id: GraphicStateId,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui_attribute(ui, &mut cell.visible, state_id, |ui, visible| {
            ui_split(ui, "Visible", |ui| {
                edit_result |= ui.add(PropertyEditor::new(visible, reference_store)).into();
            });
        });
        CollapsingHeader::new("Text").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut cell.text, state_id, |ui, attr| {
                ui_split(ui, "Text", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui_attribute(ui, &mut cell.text_color, state_id, |ui, attr| {
                ui_split(ui, "Color", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui_attribute(ui, &mut cell.text_size, state_id, |ui, attr| {
                ui_split(ui, "Size", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui_attribute(ui, &mut cell.text_alginment, state_id, |ui, attr| {
                ui_split(ui, "Alignment", |ui| {
                    edit_result |= ui
                        .add(
                            LComboBox::new(attr)
                                .with_id(ui.make_persistent_id("Text alginment combobox"))
                                .add_option(TextAlignment::Left, "Left")
                                .add_option(TextAlignment::Center, "Center")
                                .add_option(TextAlignment::Right, "Right"),
                        )
                        .into();
                });
            });
            ui_attribute(ui, &mut cell.font, state_id, |ui, attr| {
                ui_split(ui, "Font", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui_attribute(ui, &mut cell.text_position, state_id, |ui, attr| {
                ui_split(ui, "Position X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.y, reference_store))
                        .into();
                });
            });
        });
        CollapsingHeader::new("Position").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut cell.pos, state_id, |ui, attr| {
                ui_split(ui, "Position X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.y, reference_store))
                        .into();
                });
                ui_split(ui, "Z", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.z, reference_store))
                        .into();
                });
            });
        });
        CollapsingHeader::new("Shape").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut cell.size, state_id, |ui, attr| {
                ui_split(ui, "Width", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.x, reference_store))
                        .into();
                });
                ui_split(ui, "Height", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.y, reference_store))
                        .into();
                });
            });
            ui_attribute(ui, &mut cell.skew, state_id, |ui, attr| {
                ui_split(ui, "Skew", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui.add_enabled_ui(cell.corner_offsets.has_state(&state_id), |ui| {
                ui_split(ui, "Corner offsets", |_| {});
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_attribute(ui, &mut cell.corner_offsets, state_id, |ui, attr| {
                ui_split(ui, "Top left X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_left.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_left.y, reference_store))
                        .into();
                });
                ui_split(ui, "Top right X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_right.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_right.y, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom left X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_left.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_left.y, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom right X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_right.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_right.y, reference_store))
                        .into();
                });
            });
        });
        CollapsingHeader::new("Rounding").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut cell.rounding, state_id, |ui, attr| {
                ui_split(ui, "Top left", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_left, reference_store))
                        .into();
                });
                ui_split(ui, "Top right", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_right, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom left", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_left, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom right", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_right, reference_store))
                        .into();
                });
            });
        });
        CollapsingHeader::new("Background").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut cell.color, state_id, |ui, attr| {
                ui_split(ui, "Color", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
            ui_attribute(ui, &mut cell.image, state_id, |ui, attr| {
                ui_split(ui, "Image", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
        });
    });
    edit_result
}

pub fn clip_area_editor(
    ui: &mut Ui,
    clip_area: &mut ClipArea,
    state_id: GraphicStateId,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui.add_enabled_ui(state_id == TEMPLATE_ID, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(ui.spacing().icon_width);
                ui.add_space(ui.spacing().item_spacing.x * 2.0);
                ui_split(ui, "Layer", |ui| {
                    edit_result |= ui
                        .add(DragValue::new(&mut clip_area.render_layer).clamp_range(0..=31))
                        .into();
                });
            });
        });

        CollapsingHeader::new("Position").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut clip_area.pos, state_id, |ui, attr| {
                ui_split(ui, "Position X", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.x, reference_store))
                        .into();
                });
                ui_split(ui, "Y", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.y, reference_store))
                        .into();
                });
                ui_split(ui, "Z", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.z, reference_store))
                        .into();
                });
            });
        });
        CollapsingHeader::new("Shape").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut clip_area.size, state_id, |ui, attr| {
                ui_split(ui, "Width", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.x, reference_store))
                        .into();
                });
                ui_split(ui, "Height", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.y, reference_store))
                        .into();
                });
            });
            ui_attribute(ui, &mut clip_area.skew, state_id, |ui, attr| {
                ui_split(ui, "Skew", |ui| {
                    edit_result |= ui.add(PropertyEditor::new(attr, reference_store)).into();
                });
            });
        });
        CollapsingHeader::new("Rounding").show_unindented(ui, |ui| {
            ui_attribute(ui, &mut clip_area.rounding, state_id, |ui, attr| {
                ui_split(ui, "Top left", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_left, reference_store))
                        .into();
                });
                ui_split(ui, "Top right", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.top_right, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom left", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_left, reference_store))
                        .into();
                });
                ui_split(ui, "Bottom right", |ui| {
                    edit_result |= ui
                        .add(PropertyEditor::new(&mut attr.bot_right, reference_store))
                        .into();
                });
            });
        });
    });
    edit_result
}

pub fn root_editor(
    ui: &mut Ui,
    root: &mut Root,
    state_id: GraphicStateId,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui_attribute(ui, &mut root.position, state_id, |ui, attr| {
            ui_split(ui, "Position X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.x, reference_store))
                    .into();
            });
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.y, reference_store))
                    .into();
            });
        });
    });

    edit_result
}

pub fn driver_table_editor(
    ui: &mut Ui,
    driver_table: &mut DriverTable,
    state_id: GraphicStateId,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui_attribute(ui, &mut driver_table.position, state_id, |ui, attr| {
            ui_split(ui, "Position X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.x, reference_store))
                    .into();
            });
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.y, reference_store))
                    .into();
            });
        });
        ui_attribute(ui, &mut driver_table.row_offset, state_id, |ui, attr| {
            ui_split(ui, "Row offset X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.x, reference_store))
                    .into();
            });
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut attr.y, reference_store))
                    .into();
            });
        });
    });

    edit_result
}

pub fn entry_context_editor(
    ui: &mut Ui,
    entry_context: &mut EntryContext,
    state_id: GraphicStateId,
    _reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui_attribute(ui, &mut entry_context.selection, state_id, |ui, attr| {
            ui_split(ui, "Selected entry", |ui| {
                edit_result |= LComboBox::new(attr)
                    .add_option(EntrySelection::First, "First")
                    .add_option(EntrySelection::Second, "Second")
                    .add_option(EntrySelection::Third, "Third")
                    .add_option(EntrySelection::AheadOfFocus, "Ahead of focus")
                    .add_option(EntrySelection::Focus, "Focus")
                    .add_option(EntrySelection::BehindFocus, "Behind focus")
                    .ui(ui)
                    .into();
            });
        });
    });

    edit_result
}
