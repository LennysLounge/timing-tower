pub mod insert_node;

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::StyleDefinition,
};
use bevy::ecs::{event::EventWriter, system::Resource};

use self::insert_node::InsertNode;

pub enum EditorCommand {
    Undo,
    Redo,
    InsertNode(InsertNode),
}
impl EditorCommand {
    pub fn name(&self) -> &str {
        match self {
            EditorCommand::Undo => "Undo",
            EditorCommand::Redo => "Redo",
            EditorCommand::InsertNode(_) => "Insert node",
        }
    }
    fn redo(&self, style: &mut StyleDefinition) {
        match self {
            EditorCommand::Undo => (),
            EditorCommand::Redo => (),
            EditorCommand::InsertNode(o) => o.redo(style),
        }
    }
    fn undo(&self, style: &mut StyleDefinition) {
        match self {
            EditorCommand::Undo => (),
            EditorCommand::Redo => (),
            EditorCommand::InsertNode(o) => o.undo(style),
        }
    }
}

#[derive(Resource, Default)]
pub struct UndoRedoManager {
    queue: Vec<EditorCommand>,
    past: Vec<EditorCommand>,
    future: Vec<EditorCommand>,
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
                    if let Some(command) = self.past.pop() {
                        command.undo(&mut style);
                        self.future.push(command);
                    }
                }
                EditorCommand::Redo => {
                    if let Some(command) = self.future.pop() {
                        command.redo(&mut style);
                        self.past.push(command);
                    }
                }
                command => {
                    command.redo(&mut style);
                    self.past.push(command);
                    self.future.clear();
                }
            }
        }
        savefile.set(style, &mut savefile_changed_event);
    }

    pub fn queue(&mut self, command: EditorCommand) {
        self.queue.push(command);
    }

    pub fn past(&self) -> &Vec<EditorCommand> {
        &self.past
    }

    pub fn future(&self) -> &Vec<EditorCommand> {
        &self.future
    }
}
