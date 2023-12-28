pub mod adapter_command;
pub mod edit_property;
pub mod insert_node;
pub mod move_node;
pub mod remove_node;

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::StyleDefinition,
};
use bevy::{
    app::Plugin,
    ecs::{event::EventWriter, system::Resource},
};
use unified_sim_model::Adapter;

use self::{
    adapter_command::AdapterCommand,
    edit_property::EditProperty,
    insert_node::{InsertNode, InsertNodeUndo},
    move_node::MoveNode,
    remove_node::{RemoveNode, RemoveNodeUndo},
};

pub struct CommandPlugin;
impl Plugin for CommandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(UndoRedoManager::default());
    }
}

pub enum EditorCommand {
    Undo,
    Redo,
    InsertNode(InsertNode),
    InsertNodeUndo(InsertNodeUndo),
    RemoveNode(RemoveNode),
    RemoveNodeUndo(RemoveNodeUndo),
    MoveNode(MoveNode),
    EditProperty(EditProperty),
    AdapterCommand(AdapterCommand),
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
            EditorCommand::AdapterCommand(_) => "Adapter command",
        }
    }

    fn debug_name(&self) -> &str {
        match self {
            EditorCommand::Undo => "EditorCommand::Undo",
            EditorCommand::Redo => "EditorCommand::Redo",
            EditorCommand::InsertNode(_) => "EditorCommand::InsertNode",
            EditorCommand::InsertNodeUndo(_) => "EditorCommand::InsertNodeUndo",
            EditorCommand::RemoveNode(_) => "EditorCommand::RemoveNode",
            EditorCommand::RemoveNodeUndo(_) => "EditorCommand::RemoveNodeUndo",
            EditorCommand::MoveNode(_) => "EditorCommand::MoveNode",
            EditorCommand::EditProperty(_) => "EditorCommand::EditProperty",
            EditorCommand::AdapterCommand(_) => "EditorCommand::AdapterCommand",
        }
    }

    fn execute(self, style: &mut StyleDefinition, adapter: &mut Adapter) -> Option<EditorCommand> {
        match self {
            EditorCommand::Undo => unreachable!("Undo command should never be executed"),
            EditorCommand::Redo => unreachable!("Redo command should never be executed"),
            EditorCommand::InsertNode(o) => o.execute(style),
            EditorCommand::InsertNodeUndo(o) => o.execute(style),
            EditorCommand::RemoveNode(o) => o.execute(style),
            EditorCommand::RemoveNodeUndo(o) => o.execute(style),
            EditorCommand::MoveNode(o) => o.execute(style),
            EditorCommand::EditProperty(o) => o.execute(style),
            EditorCommand::AdapterCommand(o) => {
                o.execute(adapter);
                None
            }
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
        adapter: &mut Adapter,
    ) {
        if self.queue.is_empty() {
            return;
        }

        let mut style = savefile.style().clone();
        let commands: Vec<_> = self.queue.drain(0..).collect();
        for command in commands {
            match command {
                EditorCommand::Undo => {
                    self.undo
                        .pop()
                        .and_then(|undo| undo.execute(&mut style, adapter))
                        .map(|redo| self.redo.push(redo));
                }
                EditorCommand::Redo => {
                    self.redo
                        .pop()
                        .and_then(|redo| redo.execute(&mut style, adapter))
                        .map(|undo| self.undo.push(undo));
                }
                command => {
                    command.execute(&mut style, adapter).map(|undo| {
                        self.add_to_undo_list(undo);
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

    fn add_to_undo_list(&mut self, command: EditorCommand) {
        use EditorCommand as EC;
        let can_merge = self.undo.last().is_some_and(|last| match (last, &command) {
            (EC::EditProperty(last), EC::EditProperty(new)) => last.can_merge_with(new),
            _ => false,
        });

        if can_merge {
            let last = self
                .undo
                .pop()
                .expect("If two editor commands can merge the list can't be empty");
            let merged: EditorCommand = match (last, command) {
                (EC::EditProperty(last), EC::EditProperty(new)) => last.merge(new).into(),
                (a, b) => unreachable!(
                    "Editor commands {} and {} can merge but no merge action was defined",
                    a.debug_name(),
                    b.debug_name()
                ),
            };
            self.undo.push(merged);
        } else {
            self.undo.push(command);
        }
    }
}