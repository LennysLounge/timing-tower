use backend::{
    style::variables::fixed_value::FixedValue,
    value_types::{Boolean, Number, Text, Tint},
};
use bevy_egui::egui::{ComboBox, DragValue, Ui};

use crate::{reference_store::ReferenceStore, ui::combo_box::LComboBox};

pub fn property_editor(
    ui: &mut Ui,
    value: &mut FixedValue,
    _reference_store: &ReferenceStore,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Type:");

        changed |= ui
            .add(
                LComboBox::new_comparable(value, |a, b| {
                    std::mem::discriminant(a) == std::mem::discriminant(b)
                })
                .add_option(FixedValue::Number(Number::default()), "Number")
                .add_option(FixedValue::Text(Text::default()), "Text")
                .add_option(FixedValue::Tint(Tint::default()), "Color")
                .add_option(FixedValue::Boolean(Boolean::default()), "Yes/No"),
            )
            .changed();
    });

    ui.horizontal(|ui| match value {
        FixedValue::Number(Number(number)) => {
            ui.label("Value");
            changed |= ui.add(DragValue::new(number)).changed();
        }
        FixedValue::Text(Text(text)) => {
            ui.label("Text");
            changed |= ui.text_edit_singleline(text).changed();
        }
        FixedValue::Tint(Tint(tint)) => {
            ui.label("Color:");
            let mut color_local = tint.as_rgba_f32();
            changed |= ui
                .color_edit_button_rgba_unmultiplied(&mut color_local)
                .changed();
            *tint = color_local.into();
        }
        FixedValue::Boolean(Boolean(boolean)) => {
            ui.label("Value:");
            ComboBox::from_id_source(ui.next_auto_id())
                .width(50.0)
                .selected_text(match boolean {
                    true => "Yes",
                    false => "No",
                })
                .show_ui(ui, |ui| {
                    changed |= ui.selectable_value(boolean, true, "Yes").changed();
                    changed |= ui.selectable_value(boolean, false, "No").changed();
                });
        }
    });
    changed
}
