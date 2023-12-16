use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::style::{definitions::*, visitor::StyleNode};

use crate::style::visitors::{insert::InsertNodeVisitor, remove::RemoveNodeVisitor};

use super::EditorCommand;

pub struct InsertNode {
    pub target_node: Uuid,
    pub position: DropPosition,
    pub node: Box<dyn StyleNode + Sync + Send>,
}
impl InsertNode {
    pub fn undo(&self, style: &mut StyleDefinition) {
        RemoveNodeVisitor::new(*self.node.id()).remove_from(style);
    }
    pub fn redo(&self, style: &mut StyleDefinition) {
        InsertNodeVisitor::new(self.target_node, self.position, (*self.node).box_clone())
            .insert_into(style);
    }
}

impl From<InsertNode> for EditorCommand {
    fn from(value: InsertNode) -> Self {
        Self::InsertNode(value)
    }
}
