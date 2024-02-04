use backend::style::cell::{Cell, ClipArea};
use bevy_egui::egui::{self, vec2, CollapsingHeader, DragValue, Layout, Ui, WidgetText};
use common::communication::TextAlignment;

use crate::{
    command::edit_property::EditResult, reference_store::ReferenceStore, ui::combo_box::LComboBox,
};

use super::property::PropertyEditor;

fn ui_split(ui: &mut Ui, label: impl Into<WidgetText>, right: impl FnMut(&mut Ui)) {
    ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width() * 0.4, 20.0),
            Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.add(egui::Label::new(label).truncate(true));
            },
        )
        .response;
        ui.add_space(ui.spacing().item_spacing.x);
        ui.scope(right);
    });
}

pub fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.scope(|ui| {
        ui.visuals_mut().collapsing_header_frame = false;
        CollapsingHeader::new("Visibility").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Visible", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.visible, reference_store))
                    .into();
            });
            ui.add_space(10.0);
        });
        CollapsingHeader::new("Text").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Text", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.text, reference_store))
                    .into();
            });
            ui_split(ui, "Color", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.text_color, reference_store))
                    .into();
            });
            ui_split(ui, "Size", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.text_size, reference_store))
                    .into();
            });
            ui_split(ui, "Alignment", |ui| {
                edit_result |= ui
                    .add(
                        LComboBox::new(&mut cell.text_alginment)
                            .with_id(ui.make_persistent_id("Text alginment combobox"))
                            .add_option(TextAlignment::Left, "Left")
                            .add_option(TextAlignment::Center, "Center")
                            .add_option(TextAlignment::Right, "Right"),
                    )
                    .into();
            });
            ui_split(ui, "Font", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.font, reference_store))
                    .into();
            });

            ui_split(ui, "Position X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.text_position.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.text_position.y,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(10.0);
        });
        CollapsingHeader::new("Position").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Position X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.pos.x, reference_store))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.pos.y, reference_store))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Z", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.pos.z, reference_store))
                    .into();
            });
            ui.add_space(10.0);
        });
        CollapsingHeader::new("Shape").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Width", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.size.x, reference_store))
                    .into();
            });
            ui_split(ui, "Height", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.size.y, reference_store))
                    .into();
            });
            ui_split(ui, "Skew", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.skew, reference_store))
                    .into();
            });
            ui_split(ui, "Corner offsets", |_| {});
            ui_split(ui, "Top left X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.top_left.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.top_left.x,
                        reference_store,
                    ))
                    .into();
            });

            ui_split(ui, "Top right X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.top_right.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.top_right.x,
                        reference_store,
                    ))
                    .into();
            });

            ui_split(ui, "Bottom left X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.bot_left.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.bot_left.x,
                        reference_store,
                    ))
                    .into();
            });

            ui_split(ui, "Bottom right X", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.bot_right.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(-ui.spacing().item_spacing.y);
            ui_split(ui, "Y", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.corner_offsets.bot_right.x,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(10.0);
        });
        CollapsingHeader::new("Rounding").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Top left", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.rounding.top_left,
                        reference_store,
                    ))
                    .into();
            });
            ui_split(ui, "Top right", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.rounding.top_right,
                        reference_store,
                    ))
                    .into();
            });
            ui_split(ui, "Bottom left", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.rounding.bot_left,
                        reference_store,
                    ))
                    .into();
            });
            ui_split(ui, "Bottom right", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(
                        &mut cell.rounding.bot_right,
                        reference_store,
                    ))
                    .into();
            });
            ui.add_space(10.0);
        });
        CollapsingHeader::new("Background").show_unindented(ui, |ui| {
            ui.add_space(10.0);
            ui_split(ui, "Color", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.color, reference_store))
                    .into();
            });
            ui_split(ui, "Image", |ui| {
                edit_result |= ui
                    .add(PropertyEditor::new(&mut cell.image, reference_store))
                    .into();
            });
            ui.add_space(10.0);
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

    ui.horizontal(|ui| {
        ui.label("Layer:");
        let res = ui.add(DragValue::new(&mut clip_area.render_layer).clamp_range(0..=31));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.pos.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.pos.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.pos.z, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.size.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.size.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        let res = ui.add(PropertyEditor::new(&mut clip_area.skew, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        let res = ui.add(PropertyEditor::new(
            &mut clip_area.rounding.top_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        let res = ui.add(PropertyEditor::new(
            &mut clip_area.rounding.top_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        let res = ui.add(PropertyEditor::new(
            &mut clip_area.rounding.bot_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        let res = ui.add(PropertyEditor::new(
            &mut clip_area.rounding.bot_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });

    edit_result
}
