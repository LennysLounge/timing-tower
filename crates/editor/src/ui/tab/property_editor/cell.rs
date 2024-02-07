use backend::style::graphic_items::{
    cell::{Attribute, Cell},
    clip_area::ClipArea,
};
use bevy_egui::egui::{self, vec2, CollapsingHeader, DragValue, Layout, Ui, WidgetText};
use common::communication::TextAlignment;
use uuid::Uuid;

use crate::{
    command::edit_property::EditResult, reference_store::ReferenceStore, ui::combo_box::LComboBox,
};

use super::property::PropertyEditor;

pub fn ui_split(ui: &mut Ui, label: impl Into<WidgetText>, right: impl FnMut(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2((ui.available_width()) * 0.35, 18.0),
            Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.add(egui::Label::new(label).truncate(true));
            },
        );
        ui.add_space(ui.spacing().item_spacing.x);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 18.0),
            Layout::left_to_right(egui::Align::Min).with_main_justify(true),
            right,
        );
    });
}
fn ui_attribute<T: Clone>(
    ui: &mut Ui,
    attr: &mut Attribute<T>,
    state_id: Option<&Uuid>,
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
    state_id: Option<&Uuid>,
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
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui.visuals_mut().collapsing_header_frame = false;
        ui_split(ui, "Layer", |ui| {
            edit_result |= ui
                .add_sized(
                    vec2(ui.available_width(), 0.0),
                    DragValue::new(&mut clip_area.render_layer).clamp_range(0..=31), //PropertyEditor::new(&mut clip_area.render_layer, reference_store),
                )
                .into();
        });

        CollapsingHeader::new("Position").show_unindented(ui, |ui| {
            ui_split(ui, "Position X", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.pos.x, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.pos.y, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Z", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.pos.z, reference_store),
                    )
                    .into();
            });
        });
        CollapsingHeader::new("Shape").show_unindented(ui, |ui| {
            ui_split(ui, "Width", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.size.x, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Height", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.size.y, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Skew", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.skew, reference_store),
                    )
                    .into();
            });
        });
        CollapsingHeader::new("Rounding").show_unindented(ui, |ui| {
            ui_split(ui, "Top left", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.rounding.top_left, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Top right", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.rounding.top_right, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Bottom left", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.rounding.bot_left, reference_store),
                    )
                    .into();
            });
            ui_split(ui, "Bottom right", |ui| {
                edit_result |= ui
                    .add_sized(
                        vec2(ui.available_width(), 0.0),
                        PropertyEditor::new(&mut clip_area.rounding.bot_right, reference_store),
                    )
                    .into();
            });
        });
    });
    edit_result
}
