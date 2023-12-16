pub mod edit_property;
pub mod insert_node;
pub mod move_node;
pub mod remove_node;

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::StyleDefinition,
};
use bevy::ecs::{event::EventWriter, system::Resource};

use self::{
    edit_property::EditProperty,
    insert_node::{InsertNode, InsertNodeUndo},
    move_node::MoveNode,
    remove_node::{RemoveNode, RemoveNodeUndo},
};

pub enum EditorCommand {
    Undo,
    Redo,
    InsertNode(InsertNode),
    InsertNodeUndo(InsertNodeUndo),
    RemoveNode(RemoveNode),
    RemoveNodeUndo(RemoveNodeUndo),
    MoveNode(MoveNode),
    EditProperty(EditProperty),
}

impl EditorCommand {
    pub fn name(&self) -> &str {
        match self {
            EditorCommand::Undo => "Undo",
            EditorCommand::Redo => "Redo",
            EditorCommand::InsertNode(_) => "Insert node",
            EditorCommand::InsertNodeUndo(_) => "Insert node",
            EditorCommand::RemoveNode(_) => "Remove node",
            EditorCommand::RemoveNodeUndo(_) => "Remove node",
            EditorCommand::MoveNode(_) => "Move node",
            EditorCommand::EditProperty(_) => "Edit property",
        }
    }
    fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        match self {
            EditorCommand::Undo => unreachable!("Undo command should never be executed"),
            EditorCommand::Redo => unreachable!("Redo command should never be executed"),
            EditorCommand::InsertNode(o) => o.execute(style),
            EditorCommand::InsertNodeUndo(o) => o.execute(style),
            EditorCommand::RemoveNode(o) => o.execute(style),
            EditorCommand::RemoveNodeUndo(o) => o.execute(style),
            EditorCommand::MoveNode(o) => o.execute(style),
            EditorCommand::EditProperty(o) => o.execute(style),
        }
    }
}

#[derive(Resource, Default)]
pub struct UndoRedoManager {
    queue: Vec<EditorCommand>,
    undo: Vec<EditorCommand>,
    redo: Vec<EditorCommand>,
}
impl UndoRedoManager {
    pub fn apply_queue(
        &mut self,
        savefile: &mut Savefile,
        mut savefile_changed_event: EventWriter<SavefileChanged>,
    ) {
        if self.queue.is_empty() {
            return;
        }

        let mut style = savefile.style().clone();
        for command in self.queue.drain(0..) {
            match command {
                EditorCommand::Undo => {
                    self.undo
                        .pop()
                        .and_then(|undo| undo.execute(&mut style))
                        .map(|redo| self.redo.push(redo));
                }
                EditorCommand::Redo => {
                    self.redo
                        .pop()
                        .and_then(|redo| redo.execute(&mut style))
                        .map(|undo| self.undo.push(undo));
                }
                command => {
                    command.execute(&mut style).map(|undo| {
                        self.undo.push(undo);
                        self.redo.clear();
                    });
                }
            }
        }
        savefile.set(style, &mut savefile_changed_event);
    }

    pub fn queue<C: Into<EditorCommand>>(&mut self, command: C) {
        self.queue.push(command.into());
    }

    pub fn undo_list(&self) -> &Vec<EditorCommand> {
        &self.undo
    }

    pub fn redo_list(&self) -> &Vec<EditorCommand> {
        &self.redo
    }
}
