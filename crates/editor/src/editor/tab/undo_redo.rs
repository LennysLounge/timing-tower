use bevy_egui::egui::Ui;

use crate::editor::command::{self, UndoRedoManager};

pub fn undo_redo(ui: &mut Ui, undo_redo_manager: &mut UndoRedoManager) {
    ui.horizontal(|ui| {
        if ui.button("Undo").clicked() {
            undo_redo_manager.queue(command::EditorCommand::Undo);
        }
        if ui.button("Redo").clicked() {
            undo_redo_manager.queue(command::EditorCommand::Redo);
        }
    });
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for future_command in undo_redo_manager.redo_list().iter() {
            ui.horizontal(|ui| {
                ui.add_space(17.0);
                ui.label(future_command.name());
            });
        }
        ui.label(">> Now");
        for past_command in undo_redo_manager.undo_list().iter().rev() {
            ui.horizontal(|ui| {
                ui.add_space(17.0);
                ui.label(past_command.name());
            });
        }
    });
}
