use uuid::Uuid;

use backend::style::definitions::*;

use crate::style::visitors::{
    insert::InsertNodeVisitor,
    remove::{RemoveNodeVisitor, RemovedNode},
};

use super::EditorCommand;

pub struct RemoveNode {
    id: Uuid,
    removed_node: Option<RemovedNode>,
}
impl RemoveNode {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            removed_node: None,
        }
    }
    pub fn undo(&mut self, style: &mut StyleDefinition) {
        if let Some(remove_node) = self.removed_node.as_ref() {
            InsertNodeVisitor::new(
                remove_node.parent_id,
                remove_node.position,
                (*remove_node.node).box_clone(),
            )
            .insert_into(style);
        }
    }
    pub fn redo(&mut self, style: &mut StyleDefinition) {
        self.removed_node = RemoveNodeVisitor::new(self.id).remove_from(style);
    }
}

impl From<RemoveNode> for EditorCommand {
    fn from(value: RemoveNode) -> Self {
        Self::RemoveNode(value)
    }
}
