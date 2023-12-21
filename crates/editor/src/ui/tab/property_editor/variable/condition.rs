use bevy_egui::egui::{ComboBox, InnerResponse, Sense, Ui, Vec2};

use crate::{
    editor::reference_store::ReferenceStore, ui::tab::property_editor::property::PropertyEditor,
};
use backend::{
    style::variables::condition::{
        BooleanComparator, BooleanComparison, Comparison, Condition, NumberComparator,
        NumberComparison, Output, TextComparator, TextComparison, UntypedOutput,
    },
    value_types::ValueType,
};

use super::EguiComboBoxExtension;

pub fn property_editor(ui: &mut Ui, value: &mut Condition, asset_repo: &ReferenceStore) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Output type:");
        changed |= ComboBox::from_id_source(ui.next_auto_id())
            .choose(
                ui,
                &mut value.output,
                vec![
                    (UntypedOutput::Number(Output::default()), "Number"),
                    (UntypedOutput::Text(Output::default()), "Text"),
                    (UntypedOutput::Color(Output::default()), "Tint"),
                    (UntypedOutput::Boolean(Output::default()), "Boolean"),
                    (UntypedOutput::Image(Output::default()), "Texture"),
                ],
            )
            .changed();
    });

    ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

    ui.horizontal(|ui| {
        ui.label("If");
        ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());

        let InnerResponse {
            inner: new_untyped_ref,
            response: _,
        } = asset_repo.untyped_editor(ui, value.comparison.left_side_id(), |v| {
            return match v.value_type {
                ValueType::Number => true,
                ValueType::Text => true,
                ValueType::Boolean => true,
                _ => false,
            };
        });

        if let Some(reference) = new_untyped_ref {
            value.comparison.set_left_side(reference);
            changed |= true;
        }
        ui.label("is");
    });

    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        match &mut value.comparison {
            Comparison::Number(NumberComparison { comparator, .. }) => {
                changed |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(
                        ui,
                        comparator,
                        vec![
                            (NumberComparator::Equal, "equal"),
                            (NumberComparator::Greater, "greater"),
                            (NumberComparator::GreaterEqual, "greater or equal"),
                            (NumberComparator::Less, "less"),
                            (NumberComparator::LessEqual, "less or equal"),
                        ],
                    )
                    .changed();
                match comparator {
                    NumberComparator::Equal => ui.label("to"),
                    NumberComparator::Greater => ui.label("than"),
                    NumberComparator::GreaterEqual => ui.label("to"),
                    NumberComparator::Less => ui.label("than"),
                    NumberComparator::LessEqual => ui.label("to"),
                };
            }
            Comparison::Text(TextComparison { comparator: c, .. }) => {
                changed |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(ui, c, vec![(TextComparator::Like, "like")])
                    .changed();
            }
            Comparison::Boolean(BooleanComparison { comparator: c, .. }) => {
                changed |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(
                        ui,
                        c,
                        vec![
                            (BooleanComparator::Is, "is"),
                            (BooleanComparator::IsNot, "is not"),
                        ],
                    )
                    .changed();
            }
        }
    });

    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        // show select for right side
        changed |= ui
            .horizontal(|ui| match &mut value.comparison {
                Comparison::Number(NumberComparison { right, .. }) => {
                    ui.add(PropertyEditor::new(right, asset_repo)).changed()
                }
                Comparison::Text(TextComparison { right, .. }) => {
                    ui.add(PropertyEditor::new(right, asset_repo)).changed()
                }
                Comparison::Boolean(BooleanComparison { right, .. }) => {
                    ui.add(PropertyEditor::new(right, asset_repo)).changed()
                }
            })
            .inner;
    });
    ui.label("then:");
    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        changed |= match &mut value.output {
            UntypedOutput::Number(Output { truee, .. }) => {
                ui.add(PropertyEditor::new(truee, asset_repo))
            }
            UntypedOutput::Text(Output { truee, .. }) => {
                ui.add(PropertyEditor::new(truee, asset_repo))
            }
            UntypedOutput::Color(Output { truee, .. }) => {
                ui.add(PropertyEditor::new(truee, asset_repo))
            }
            UntypedOutput::Boolean(Output { truee, .. }) => {
                ui.add(PropertyEditor::new(truee, asset_repo))
            }
            UntypedOutput::Image(Output { truee, .. }) => {
                ui.add(PropertyEditor::new(truee, asset_repo))
            }
        }
        .changed();
    });
    ui.label("else:");
    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        changed |= match &mut value.output {
            UntypedOutput::Number(output) => {
                ui.add(PropertyEditor::new(&mut output.falsee, asset_repo))
            }
            UntypedOutput::Text(Output { falsee, .. }) => {
                ui.add(PropertyEditor::new(falsee, asset_repo))
            }
            UntypedOutput::Color(Output { falsee, .. }) => {
                ui.add(PropertyEditor::new(falsee, asset_repo))
            }
            UntypedOutput::Boolean(Output { falsee, .. }) => {
                ui.add(PropertyEditor::new(falsee, asset_repo))
            }
            UntypedOutput::Image(Output { falsee, .. }) => {
                ui.add(PropertyEditor::new(falsee, asset_repo))
            }
        }
        .changed();
    });
    changed
}
