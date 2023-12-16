use uuid::Uuid;

use backend::style::definitions::*;

use crate::style::visitors::{
    insert::InsertNodeVisitor,
    remove::{RemoveNodeVisitor, RemovedNode},
};

use super::EditorCommand;

pub struct RemoveNode {
    pub id: Uuid,
}
impl RemoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let Some(removed_node) = RemoveNodeVisitor::new(self.id).remove_from(style) else {
            return None;
        };
        Some(RemoveNodeUndo { removed_node }.into())
    }
}

impl From<RemoveNode> for EditorCommand {
    fn from(value: RemoveNode) -> Self {
        Self::RemoveNode(value)
    }
}

pub struct RemoveNodeUndo {
    removed_node: RemovedNode,
}
impl RemoveNodeUndo {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let RemovedNode {
            parent_id,
            node,
            position,
        } = self.removed_node;
        let node_id = *node.id();
        InsertNodeVisitor::new(parent_id, position, node).insert_into(style);
        Some(RemoveNode { id: node_id }.into())
    }
}
impl From<RemoveNodeUndo> for EditorCommand {
    fn from(value: RemoveNodeUndo) -> Self {
        Self::RemoveNodeUndo(value)
    }
}
