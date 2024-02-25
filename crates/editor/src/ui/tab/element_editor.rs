use backend::{
    exact_variant::ExactVariant,
    graphic::GraphicStates,
    style::{
        graphic::{
            graphic_items::{
                cell::Cell, clip_area::ClipArea, driver_table::DriverTable, root::Root, Attribute,
                GraphicItem, GraphicItemId,
            },
            GraphicDefinition, GraphicStateId,
        },
        StyleDefinition, StyleItem,
    },
    tree_iterator::TreeIteratorMut,
};
use bevy_egui::egui::{
    self, vec2, CollapsingHeader, DragValue, Layout, ScrollArea, Ui, WidgetText,
};
use common::communication::TextAlignment;

use crate::{
    command::{
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    reference_store::ReferenceStore,
    ui::{combo_box::LComboBox, selection_manager::SelectionManager},
};

use super::property_editor::property::PropertyEditor;

pub fn element_editor(
    ui: &mut Ui,
    selection_manager: &mut SelectionManager,
    style: &mut ExactVariant<StyleItem, StyleDefinition>,
    reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
    graphic_states: &mut GraphicStates,
) {
    let Some(style_item_selection) = selection_manager.selected() else {
        return;
    };
    let selection_state = selection_manager.selected_state().unwrap();

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            style.search_mut(style_item_selection, |style_item| {
                if let StyleItem::Graphic(graphic) = style_item {
                    let edit_result = graphic_item(
                        ui,
                        graphic,
                        selection_state.graphic_item.as_ref(),
                        reference_store,
                        graphic_states,
                    );
                    if let EditResult::FromId(widget_id) = edit_result {
                        undo_redo_manager.queue(EditProperty::new(
                            graphic.id,
                            graphic.clone(),
                            widget_id,
                        ));
                    }
                }
            });
        });
}

fn graphic_item(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    graphic_item_selection: Option<&GraphicItemId>,
    reference_store: &ReferenceStore,
    graphic_states: &mut GraphicStates,
) -> EditResult {
    let mut edit_result = EditResult::None;

    if let Some(state) = graphic_states.states.get(&graphic.id).and_then(|state_id| {
        graphic
            .states
            .iter_mut()
            .find(|state| state.id == *state_id)
    }) {
        ui.label("State name:");
        edit_result |= ui.text_edit_singleline(&mut state.name).into();
        ui.separator();
    }

    if let Some(graphic_item_selection) = graphic_item_selection {
        edit_result |= graphic
            .items
            .search_mut(*graphic_item_selection, |graphic_item| {
                editor(
                    ui,
                    graphic_item,
                    graphic_states.states.get(&graphic.id),
                    reference_store,
                )
            })
            .unwrap_or_default();
    }
    // Copy the name of the root graphic item to the graphic to keep them synced.
    if matches!(edit_result, EditResult::FromId(_)) {
        graphic.name = graphic.items.name.clone();
    }
    edit_result
}

fn editor(
    ui: &mut Ui,
    element: &mut GraphicItem,
    state_id: Option<&GraphicStateId>,
    reference_store: &ReferenceStore,
) -> EditResult {
    match element {
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
    state_id: Option<&GraphicStateId>,
    mut add_content: impl FnMut(&mut Ui, &mut T),
) {
    if let Some(state_id) = state_id {
        ui.horizontal(|ui| {
            let mut enabled = attr.has_state(&state_id);
            if ui.checkbox(&mut enabled, "").changed() {
                if enabled {
                    attr.add_state(*state_id);
                } else {
                    attr.remove_state(state_id);
                }
            }
            ui.vertical(|ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    if let Some(attr) = attr.get_state(state_id) {
                        add_content(ui, attr);
                    } else {
                        add_content(ui, attr.template_mut());
                    }
                });
            });
        });
    } else {
        ui.horizontal(|ui| {
            // Add enough space to equal the checkbox.
            ui.add_space(ui.spacing().icon_width);
            ui.add_space(ui.spacing().item_spacing.x);
            ui.add_space(ui.spacing().item_spacing.x);
            ui.vertical(|ui| {
                add_content(ui, attr.template_mut());
            });
        });
    }
}

pub fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    state_id: Option<&GraphicStateId>,
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
            ui.add_enabled_ui(
                state_id.map_or(true, |state_id| cell.corner_offsets.has_state(state_id)),
                |ui| {
                    ui_split(ui, "Corner offsets", |_| {});
                },
            );
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
    state_id: Option<&GraphicStateId>,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui.add_enabled_ui(state_id.is_none(), |ui| {
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
    state_id: Option<&GraphicStateId>,
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
    state_id: Option<&GraphicStateId>,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
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
