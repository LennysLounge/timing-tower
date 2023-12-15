use std::ops::ControlFlow;

use backend::style::{
    cell::{Cell, TextAlignment},
    definitions::*,
    variables::{condition::Condition, fixed_value::FixedValue, map::Map, VariableBehavior},
    visitor::{NodeVisitorMut, StyleNode},
};
use bevy_egui::egui::{ComboBox, Ui};

use crate::{property_editor::PropertyEditor, reference_store::ReferenceStore, style::variables};

pub struct PropertyEditorVisitor<'a> {
    ui: &'a mut Ui,
    reference_store: &'a ReferenceStore,
    changed: bool,
}
impl<'a> PropertyEditorVisitor<'a> {
    pub fn new(ui: &'a mut Ui, reference_store: &'a ReferenceStore) -> Self {
        Self {
            ui,
            reference_store,
            changed: false,
        }
    }
    pub fn apply_to(mut self, node: &mut dyn StyleNode) -> bool {
        node.enter_mut(&mut self);
        self.changed
    }
}
impl<'a> NodeVisitorMut for PropertyEditorVisitor<'a> {
    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            changed,
            reference_store: _,
        } = self;

        if folder.renameable() {
            ui.label("Name:");
            *changed |= ui.text_edit_singleline(folder.name_mut()).changed();
        }
        ControlFlow::Continue(())
    }

    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        self.changed |= cell_property_editor(self.ui, &mut tower.cell, self.reference_store);
        ControlFlow::Break(())
    }

    fn visit_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            changed,
            reference_store,
        } = self;

        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            *changed |= ui
                .add(PropertyEditor::new(
                    &mut table.row_offset.x,
                    reference_store,
                ))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            *changed |= ui
                .add(PropertyEditor::new(
                    &mut table.row_offset.y,
                    reference_store,
                ))
                .changed();
        });
        ui.separator();
        *changed |= cell_property_editor(ui, &mut table.cell, reference_store);
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        self.changed |= cell_property_editor(self.ui, &mut row.cell, self.reference_store);
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            changed,
            reference_store,
        } = self;

        ui.label("Name:");
        *changed |= ui.text_edit_singleline(&mut column.name).changed();
        ui.separator();
        *changed |= cell_property_editor(ui, &mut column.cell, reference_store);
        ControlFlow::Continue(())
    }

    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            changed,
            reference_store: _,
        } = self;

        ui.label("Name");
        *changed |= ui.text_edit_singleline(&mut asset.name).changed();
        ui.separator();
        ui.label("Path:");
        *changed |= ui.text_edit_singleline(&mut asset.path).changed();
        ControlFlow::Continue(())
    }

    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            changed,
            reference_store,
        } = self;

        ui.label("Name:");
        *changed |= ui.text_edit_singleline(&mut variable.name).changed();

        ui.horizontal(|ui| {
            ui.label("Behavior:");
            ComboBox::new(ui.next_auto_id(), "")
                .selected_text(match variable.behavior {
                    VariableBehavior::FixedValue(_) => "Fixed value",
                    VariableBehavior::Condition(_) => "Condition",
                    VariableBehavior::Map(_) => "Map",
                })
                .show_ui(ui, |ui| {
                    let is_fixed_value =
                        matches!(variable.behavior, VariableBehavior::FixedValue(_));
                    if ui.selectable_label(is_fixed_value, "Fixed value").clicked()
                        && !is_fixed_value
                    {
                        variable.behavior = VariableBehavior::FixedValue(FixedValue::default());
                        *changed |= true;
                    }

                    let is_condition = matches!(variable.behavior, VariableBehavior::Condition(_));
                    if ui.selectable_label(is_condition, "Condition").clicked() && !is_condition {
                        variable.behavior = VariableBehavior::Condition(Condition::default());
                        *changed = true;
                    }
                    let is_map = matches!(variable.behavior, VariableBehavior::Map(_));
                    if ui.selectable_label(is_map, "Map").clicked() && !is_map {
                        variable.behavior = VariableBehavior::Map(Map::default());
                        *changed = true;
                    }
                });
        });

        ui.separator();
        *changed |= match &mut variable.behavior {
            VariableBehavior::FixedValue(value) => {
                variables::fixed_value::property_editor(ui, value, reference_store)
            }
            VariableBehavior::Condition(value) => {
                variables::condition::property_editor(ui, value, reference_store)
            }
            VariableBehavior::Map(value) => {
                variables::map::property_editor(ui, value, reference_store)
            }
        };
        ControlFlow::Continue(())
    }
}

fn cell_property_editor(ui: &mut Ui, cell: &mut Cell, reference_store: &ReferenceStore) -> bool {
    let mut changed = false;

    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.visible, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Text:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.text, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Text color:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.text_color, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Text size:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.text_size, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Text alginment:");
        ComboBox::from_id_source("Text alginment combobox")
            .selected_text(match cell.text_alginment {
                TextAlignment::Left => "Left",
                TextAlignment::Center => "Center",
                TextAlignment::Right => "Right",
            })
            .show_ui(ui, |ui| {
                changed |= ui
                    .selectable_value(&mut cell.text_alginment, TextAlignment::Left, "Left")
                    .changed();
                changed |= ui
                    .selectable_value(&mut cell.text_alginment, TextAlignment::Center, "Center")
                    .changed();
                changed |= ui
                    .selectable_value(&mut cell.text_alginment, TextAlignment::Right, "Right")
                    .changed();
            });
    });
    ui.horizontal(|ui| {
        ui.label("Text pos x:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.text_position.x,
                reference_store,
            ))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Text pos y:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.text_position.y,
                reference_store,
            ))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Background color:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.color, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Background image:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.image, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.pos.x, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.pos.y, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.pos.z, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.size.x, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.size.y, reference_store))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        changed |= ui
            .add(PropertyEditor::new(&mut cell.skew, reference_store))
            .changed();
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.rounding.top_left,
                reference_store,
            ))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.rounding.top_right,
                reference_store,
            ))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.rounding.bot_right,
                reference_store,
            ))
            .changed();
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        changed |= ui
            .add(PropertyEditor::new(
                &mut cell.rounding.bot_left,
                reference_store,
            ))
            .changed();
    });
    changed
}
