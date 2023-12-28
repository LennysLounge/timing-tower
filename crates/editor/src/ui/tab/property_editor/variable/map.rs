use bevy_egui::egui::{vec2, ComboBox, InnerResponse, Response, Ui};

use crate::{
    reference_store::ReferenceStore,
    ui::tab::property_editor::property::{PropertyEditor, ValueTypeEditor},
};
use backend::{
    style::variables::map::{Input, Map, NumberComparator, Output, TextComparator, UntypedOutput},
    value_types::{ValueType, ValueTypeOf},
};

use super::EguiComboBoxExtension;

pub fn property_editor(ui: &mut Ui, value: &mut Map, asset_repo: &ReferenceStore) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Map input: ");

        let InnerResponse {
            inner: new_untyped_ref,
            response: _,
        } = asset_repo.untyped_editor(ui, &value.input.input_id(), |v| match v.value_type {
            ValueType::Number => true,
            ValueType::Text => true,
            _ => false,
        });

        if let Some(new_untyped_ref) = new_untyped_ref {
            // Only update the actual input reference
            if new_untyped_ref.value_type == value.input.value_type() {
                match &mut value.input {
                    Input::Number { input, .. } => *input = new_untyped_ref.typed(),
                    Input::Text { input, .. } => *input = new_untyped_ref.typed(),
                }
            } else {
                // Change the entire type of the input to match the new reference.
                value.input = match new_untyped_ref.value_type {
                    ValueType::Number => Input::Number {
                        input: new_untyped_ref.typed(),
                        cases: Vec::new(),
                    },
                    ValueType::Text => Input::Text {
                        input: new_untyped_ref.typed(),
                        cases: Vec::new(),
                    },
                    value_type @ _ => {
                        unreachable!("Type {} not allowed in comparison", value_type.name())
                    }
                };
                value.output.clear();
            }
            changed |= true;
        };
    });
    ui.horizontal(|ui| {
        ui.label("to type: ");

        let count = value.input.case_count();
        changed |= ComboBox::from_id_source(ui.next_auto_id())
            .choose(
                ui,
                &mut value.output,
                vec![
                    (UntypedOutput::Number(Output::with_count(count)), "Number"),
                    (UntypedOutput::Text(Output::with_count(count)), "Text"),
                    (UntypedOutput::Tint(Output::with_count(count)), "Color"),
                    (UntypedOutput::Boolean(Output::with_count(count)), "Yes/No"),
                    (UntypedOutput::Texture(Output::with_count(count)), "Image"),
                ],
            )
            .changed();
    });
    ui.separator();

    let mut remove_case = None;
    for index in 0..value.input.case_count() {
        ui.horizontal(|ui| {
            changed |= input_edit_case(&mut value.input, ui, asset_repo, index);
            ui.allocate_space(vec2(10.0, 0.0));
            ui.label("then");
            ui.allocate_space(vec2(10.0, 0.0));
            ui.horizontal(|ui| {
                changed |=
                    untyped_output_edit_case(&mut value.output, ui, asset_repo, index).changed();
            });
        });
        if ui.small_button("remove").clicked() {
            remove_case = Some(index);
        }
        ui.separator();
    }

    if let Some(index) = remove_case {
        value.input.remove(index);
        value.output.remove(index);
    }

    ui.horizontal(|ui| {
        ui.label("Default:");
        changed |= untyped_output_edit_default(&mut value.output, ui, asset_repo).changed();
    });
    ui.separator();

    if ui.button("add case").clicked() {
        value.input.push();
        value.output.push();
    }

    if value.input.case_count() != value.output.case_count() {
        panic!(
            "Case counts in map are different. This should never happen. inputs: {}, outputs:{}",
            value.input.case_count(),
            value.output.case_count()
        );
    }

    changed
}

fn input_edit_case(
    me: &mut Input,
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    index: usize,
) -> bool {
    let mut changed = false;
    match me {
        Input::Number { cases, .. } => {
            let case = cases.get_mut(index).expect("the case index must be valid");
            ui.label("If input is");
            changed |= ComboBox::from_id_source(ui.next_auto_id())
                .width(50.0)
                .choose(
                    ui,
                    &mut case.comparator,
                    vec![
                        (NumberComparator::Equal, "equal"),
                        (NumberComparator::Greater, "greater"),
                        (NumberComparator::GreaterEqual, "greater or equal"),
                        (NumberComparator::Less, "less"),
                        (NumberComparator::LessEqual, "less or equal"),
                    ],
                )
                .changed();
            ui.horizontal(|ui| {
                changed |= ui
                    .add(PropertyEditor::new(&mut case.right, reference_store))
                    .changed()
            });
        }
        Input::Text { cases, .. } => {
            let case = cases.get_mut(index).expect("the case index must be valid");
            ui.label("If input is");
            changed |= ComboBox::from_id_source(ui.next_auto_id())
                .width(50.0)
                .choose(
                    ui,
                    &mut case.comparator,
                    vec![(TextComparator::Like, "like")],
                )
                .changed();
            ui.horizontal(|ui| {
                changed |= ui
                    .add(PropertyEditor::new(&mut case.right, reference_store))
                    .changed();
            });
        }
    }

    changed
}

fn untyped_output_edit_case(
    me: &mut UntypedOutput,
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    index: usize,
) -> Response {
    match me {
        UntypedOutput::Number(output) => output_edit_case(output, ui, reference_store, index),
        UntypedOutput::Text(output) => output_edit_case(output, ui, reference_store, index),
        UntypedOutput::Tint(output) => output_edit_case(output, ui, reference_store, index),
        UntypedOutput::Boolean(output) => output_edit_case(output, ui, reference_store, index),
        UntypedOutput::Texture(output) => output_edit_case(output, ui, reference_store, index),
    }
}

fn untyped_output_edit_default(
    me: &mut UntypedOutput,
    ui: &mut Ui,
    reference_store: &ReferenceStore,
) -> Response {
    match me {
        UntypedOutput::Number(Output { default, .. }) => {
            ui.add(PropertyEditor::new(default, reference_store))
        }
        UntypedOutput::Text(Output { default, .. }) => {
            ui.add(PropertyEditor::new(default, reference_store))
        }
        UntypedOutput::Tint(Output { default, .. }) => {
            ui.add(PropertyEditor::new(default, reference_store))
        }
        UntypedOutput::Boolean(Output { default, .. }) => {
            ui.add(PropertyEditor::new(default, reference_store))
        }
        UntypedOutput::Texture(Output { default, .. }) => {
            ui.add(PropertyEditor::new(default, reference_store))
        }
    }
}

fn output_edit_case<T>(
    me: &mut Output<T>,
    ui: &mut Ui,
    reference_store: &ReferenceStore,
    index: usize,
) -> Response
where
    ValueType: ValueTypeOf<T>,
    T: Default + ValueTypeEditor,
{
    let property = me
        .cases
        .get_mut(index)
        .expect("the case index must be valid");
    ui.add(PropertyEditor::new(property, &reference_store))
}
