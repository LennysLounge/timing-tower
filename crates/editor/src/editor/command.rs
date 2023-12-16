use bevy::ecs::system::Resource;

pub enum EditorCommand {}

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
}
