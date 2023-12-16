use bevy::ecs::system::Resource;

pub enum EditorCommand {
    Undo,
    Redo,
}
impl EditorCommand {
    pub fn name(&self) -> &str {
        match self {
            EditorCommand::Undo => "Undo",
            EditorCommand::Redo => "Redo",
        }
    }
}

pub trait UndoRedo {
    fn redo(&self);
    fn undo(&self);
}

#[derive(Resource, Default)]
pub struct UndoRedoManager {
    queue: Vec<EditorCommand>,
    past: Vec<EditorCommand>,
    future: Vec<EditorCommand>,
}
impl UndoRedoManager {
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
