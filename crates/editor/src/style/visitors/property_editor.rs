use std::ops::ControlFlow;

use backend::style::{
    cell::{Cell, TextAlignment},
    definitions::*,
    variables::{condition::Condition, fixed_value::FixedValue, map::Map, VariableBehavior},
    visitor::NodeVisitorMut,
    StyleNode,
};
use bevy_egui::egui::{ComboBox, DragValue, Ui};

use crate::{
    editor::command::{
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    property_editor::PropertyEditor,
    reference_store::ReferenceStore,
    style::variables,
};

pub struct PropertyEditorVisitor<'a> {
    ui: &'a mut Ui,
    reference_store: &'a ReferenceStore,
    undo_redo_manager: &'a mut UndoRedoManager,
}
impl<'a> PropertyEditorVisitor<'a> {
    pub fn new(
        ui: &'a mut Ui,
        reference_store: &'a ReferenceStore,
        undo_redo_manager: &'a mut UndoRedoManager,
    ) -> Self {
        Self {
            ui,
            reference_store,
            undo_redo_manager,
        }
    }
    pub fn apply_to(mut self, node: &mut dyn StyleNode) {
        node.enter_mut(&mut self);
    }
}
impl<'a> NodeVisitorMut for PropertyEditorVisitor<'a> {
    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            reference_store,
            undo_redo_manager,
            ..
        } = self;

        let mut tower_edit = tower.clone();
        let edit_result = cell_property_editor(ui, &mut tower_edit.cell, reference_store);
        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(tower.id, tower_edit, widget_id));
        }

        ControlFlow::Break(())
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            reference_store,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Row offset:");
        ui.horizontal(|ui| {
            ui.label("Offset x:");
            edit_result |= ui
                .add(PropertyEditor::new(&mut row.row_offset.x, reference_store))
                .into();
        });
        ui.horizontal(|ui| {
            ui.label("Offset y:");
            edit_result |= ui
                .add(PropertyEditor::new(&mut row.row_offset.y, reference_store))
                .into();
        });
        ui.separator();
        edit_result |= cell_property_editor(ui, &mut row.cell, reference_store);

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(row.id, row.clone(), widget_id));
        }

        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            reference_store,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name:");
        edit_result |= ui.text_edit_singleline(&mut column.name).into();
        ui.separator();
        edit_result |= cell_property_editor(ui, &mut column.cell, reference_store).into();

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(column.id, column.clone(), widget_id));
        }
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name:");
        edit_result |= ui.text_edit_singleline(&mut folder.name).into();

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
        }

        ControlFlow::Continue(())
    }

    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name");
        edit_result |= ui.text_edit_singleline(&mut asset.name).into();
        ui.separator();
        ui.label("Path:");
        edit_result |= ui.text_edit_singleline(&mut asset.path).into();

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(asset.id, asset.clone(), widget_id));
        }

        ControlFlow::Continue(())
    }

    fn visit_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name:");
        edit_result |= ui.text_edit_singleline(&mut folder.name).into();

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
        }

        ControlFlow::Continue(())
    }

    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            reference_store,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name:");
        edit_result |= ui.text_edit_singleline(&mut variable.name).into();

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
                    let res = ui.selectable_label(is_fixed_value, "Fixed value");
                    if res.clicked() && !is_fixed_value {
                        variable.behavior = VariableBehavior::FixedValue(FixedValue::default());
                        edit_result |= EditResult::FromId(res.id);
                    }

                    let is_condition = matches!(variable.behavior, VariableBehavior::Condition(_));
                    let res = ui.selectable_label(is_condition, "Condition");
                    if res.clicked() && !is_condition {
                        variable.behavior = VariableBehavior::Condition(Condition::default());
                        edit_result |= EditResult::FromId(res.id);
                    }

                    let is_map = matches!(variable.behavior, VariableBehavior::Map(_));
                    let res = ui.selectable_label(is_map, "Map");
                    if res.clicked() && !is_map {
                        variable.behavior = VariableBehavior::Map(Map::default());
                        edit_result |= EditResult::FromId(res.id);
                    }
                });
        });
        ui.separator();
        edit_result |= match &mut variable.behavior {
            VariableBehavior::FixedValue(value) => {
                if variables::fixed_value::property_editor(ui, value, reference_store) {
                    EditResult::FromId(ui.make_persistent_id("Fixed_value_edit"))
                } else {
                    EditResult::None
                }
            }
            VariableBehavior::Condition(value) => {
                if variables::condition::property_editor(ui, value, reference_store) {
                    EditResult::FromId(ui.make_persistent_id("condition_value_edit"))
                } else {
                    EditResult::None
                }
            }
            VariableBehavior::Map(value) => {
                if variables::map::property_editor(ui, value, reference_store) {
                    EditResult::FromId(ui.make_persistent_id("map_value_edit"))
                } else {
                    EditResult::None
                }
            }
        };

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(variable.id, variable.clone(), widget_id));
        }

        ControlFlow::Continue(())
    }

    fn visit_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            ..
        } = self;

        let mut edit_result = EditResult::None;

        ui.label("Name:");
        edit_result |= ui.text_edit_singleline(&mut folder.name).into();

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(folder.id, folder.clone(), widget_id));
        }
        ControlFlow::Continue(())
    }

    fn visit_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            ..
        } = self;

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
            undo_redo_manager.queue(EditProperty::new(scene.id, scene.clone(), widget_id));
        }
        ControlFlow::Continue(())
    }

    fn visit_clip_area(&mut self, clip_area: &mut dyn DynClipArea) -> ControlFlow<()> {
        let PropertyEditorVisitor {
            ui,
            undo_redo_manager,
            reference_store,
            ..
        } = self;

        let mut data = clip_area.data_mut();
        let mut edit_result = EditResult::None;

        ui.horizontal(|ui| {
            ui.label("Pos x:");
            let res = ui.add(PropertyEditor::new(&mut data.position.x, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            let res = ui.add(PropertyEditor::new(&mut data.position.y, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            let res = ui.add(PropertyEditor::new(&mut data.position.z, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            let res = ui.add(PropertyEditor::new(&mut data.size.x, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            let res = ui.add(PropertyEditor::new(&mut data.size.y, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            let res = ui.add(PropertyEditor::new(&mut data.skew, reference_store));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            let res = ui.add(PropertyEditor::new(
                &mut data.rounding.top_left,
                reference_store,
            ));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            let res = ui.add(PropertyEditor::new(
                &mut data.rounding.top_right,
                reference_store,
            ));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            let res = ui.add(PropertyEditor::new(
                &mut data.rounding.bot_right,
                reference_store,
            ));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            let res = ui.add(PropertyEditor::new(
                &mut data.rounding.bot_left,
                reference_store,
            ));
            if res.changed() {
                edit_result = EditResult::FromId(res.id)
            }
        });

        if let EditResult::FromId(widget_id) = edit_result {
            undo_redo_manager.queue(EditProperty::new(*clip_area.id(), clip_area, widget_id));
        }
        ControlFlow::Continue(())
    }
}

fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        let res = ui.add(PropertyEditor::new(&mut cell.visible, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text:");
        let res = ui.add(PropertyEditor::new(&mut cell.text, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text color:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text size:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_size, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
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
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Left, "Left");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Center, "Center");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Right, "Right");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
    });
    ui.horizontal(|ui| {
        ui.label("Text pos x:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.x,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text pos y:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.y,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Background color:");
        let res = ui.add(PropertyEditor::new(&mut cell.color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Background image:");
        let res = ui.add(PropertyEditor::new(&mut cell.image, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.z, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        let res = ui.add(PropertyEditor::new(&mut cell.skew, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    edit_result
}
