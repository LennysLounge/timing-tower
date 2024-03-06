use bevy_egui::egui::{ComboBox, Sense, Ui, Vec2};

use crate::{
    command::edit_property::EditResult,
    reference_store::{any_producer_ref_editor, ReferenceStore},
    ui::{
        combo_box::LComboBox,
        tabs::{element_editor::ui_split, property_editor::property::PropertyEditor},
    },
};
use backend::{
    style::variables::{
        condition::{Comparison, Condition, Output, UntypedOutput},
        BooleanComparator, NumberComparator, TextComparator,
    },
    value_types::ValueType,
};

use super::EguiComboBoxExtension;

pub fn property_editor(
    ui: &mut Ui,
    value: &mut Condition,
    asset_repo: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui_split(ui, "Output type", |ui| {
        edit_result |= ui
            .add(
                LComboBox::new_comparable(&mut value.output, |a, b| {
                    std::mem::discriminant(a) == std::mem::discriminant(b)
                })
                .add_option(UntypedOutput::Number(Output::default()), "Number")
                .add_option(UntypedOutput::Text(Output::default()), "Text")
                .add_option(UntypedOutput::Color(Output::default()), "Tint")
                .add_option(UntypedOutput::Boolean(Output::default()), "Boolean")
                .add_option(UntypedOutput::Image(Output::default()), "Texture"),
            )
            .into();
    });
    ui.separator();

    ui.allocate_at_least(Vec2::new(0.0, 5.0), Sense::hover());

    ui.horizontal(|ui| {
        ui.label("If");
        ui.allocate_at_least(Vec2::new(5.0, 0.0), Sense::hover());

        let mut any_ref = value.comparison.left_side_ref();
        let res = any_producer_ref_editor(ui, asset_repo, &mut any_ref, |v| {
            match v.producer_ref.ty() {
                ValueType::Number => true,
                ValueType::Text => true,
                ValueType::Boolean => true,
                _ => false,
            }
        });
        if res.changed() {
            value.comparison.set_left_side(any_ref);
            edit_result |= res.into();
        }

        ui.label("is");
    });

    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        match &mut value.comparison {
            Comparison::Number { comparator, .. } => {
                edit_result |= ComboBox::from_id_source(ui.next_auto_id())
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
                    .into();
                match comparator {
                    NumberComparator::Equal => ui.label("to"),
                    NumberComparator::Greater => ui.label("than"),
                    NumberComparator::GreaterEqual => ui.label("to"),
                    NumberComparator::Less => ui.label("than"),
                    NumberComparator::LessEqual => ui.label("to"),
                };
            }
            Comparison::Text { comparator: c, .. } => {
                edit_result |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(ui, c, vec![(TextComparator::Like, "like")])
                    .into();
            }
            Comparison::Boolean { comparator: c, .. } => {
                edit_result |= ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .choose(
                        ui,
                        c,
                        vec![
                            (BooleanComparator::Is, "is"),
                            (BooleanComparator::IsNot, "is not"),
                        ],
                    )
                    .into();
            }
        }
    });

    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        // show select for right side
        edit_result |= ui
            .horizontal(|ui| match &mut value.comparison {
                Comparison::Number { right, .. } => ui.add(PropertyEditor::new(right, asset_repo)),
                Comparison::Text { right, .. } => ui.add(PropertyEditor::new(right, asset_repo)),
                Comparison::Boolean { right, .. } => ui.add(PropertyEditor::new(right, asset_repo)),
            })
            .inner
            .into();
    });
    ui.label("then:");
    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        edit_result |= match &mut value.output {
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
        .into();
    });
    ui.label("else:");
    ui.horizontal(|ui| {
        ui.allocate_at_least(Vec2::new(16.0, 0.0), Sense::hover());
        edit_result |= match &mut value.output {
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
        .into();
    });
    edit_result
}
