use std::time::Instant;

use backend::style::StyleNode;
use bevy_egui::egui::{Context, Ui};
use uuid::Uuid;

use super::command::{
    edit_property::{AnyNewValue, EditProperty, NewValue},
    EditorCommand,
};

#[derive(Clone)]
struct EditPoint {
    last_edit: Instant,
    egui_id: bevy_egui::egui::Id,
    node_id: Uuid,
    new_value: Box<dyn AnyNewValue>,
}
impl EditPoint {
    fn duration_passed(&self) -> bool {
        Instant::now().duration_since(self.last_edit).as_secs() > 1
    }
    fn is_different_to(&self, other: &EditPoint) -> bool {
        self.node_id != other.node_id || self.egui_id != other.egui_id
    }
    fn to_command(self) -> EditorCommand {
        EditProperty {
            id: self.node_id,
            new_value: self.new_value,
        }
        .into()
    }
}

/// The result of a undo/redo context.
pub enum EditResult {
    /// No value were changed.
    None,
    /// The value was changed by a widget with this id.
    FromId(bevy_egui::egui::Id),
}

/// Start an undo/redo context. Changes that occur inside this scope are
/// rememberd in the undo/redo system.
/// If the response returned from the `add_content` closure is marked with
/// a change, then a new edit point is created for this change.
/// Uses the accessor method to access the required property of the subject.
pub fn undo_redo_context<Input>(
    ui: &mut Ui,
    subjet: &mut Input,
    mut add_content: impl FnMut(&mut Ui, &mut Input) -> EditResult,
) where
    Input: Clone + StyleNode + 'static,
{
    let res = add_content(ui, subjet);

    if let EditResult::FromId(egui_id) = res {
        let edit_point = EditPoint {
            last_edit: Instant::now(),
            node_id: *subjet.id(),
            egui_id,
            new_value: Box::new(NewValue {
                new_value: subjet.clone(),
            }),
        };
        ui.data_mut(|d| {
            d.insert_persisted(
                bevy_egui::egui::Id::new("UndoRedoContext Current"),
                edit_point,
            )
        })
    }
}

/// Extrac a `EditProperty` command from the undo redo system.
pub fn extract_undo_redo_command(ctx: &Context) -> Option<EditorCommand> {
    let previous_id = bevy_egui::egui::Id::new("UndoRedoContext Previous");
    let current_id = bevy_egui::egui::Id::new("UndoRedoContext Current");
    let previous_edit: Option<EditPoint> = ctx.data_mut(|d| d.get_persisted(previous_id));
    let current_edit: Option<EditPoint> = ctx.data_mut(|d| d.get_persisted(current_id));

    // Turn the previous edit into a command either if
    // 1) A new different command was issued.
    // 2) The duration of the edit has passed.
    let command = match (previous_edit, &current_edit) {
        (Some(previous), Some(current)) if previous.is_different_to(current) => {
            Some(previous.to_command())
        }
        (Some(previous), None) if previous.duration_passed() => {
            ctx.data_mut(|d| d.remove::<EditPoint>(previous_id));
            Some(previous.to_command())
        }
        _ => None,
    };

    // Remember the current edit for next frame
    if let Some(current_edit) = current_edit {
        ctx.data_mut(|d| d.insert_persisted(previous_id, current_edit));
    }
    ctx.data_mut(|d| d.remove::<EditPoint>(current_id));

    command
}
