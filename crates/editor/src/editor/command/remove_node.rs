use uuid::Uuid;

use backend::style::{definitions::*, iterator::NodeIteratorMut, StyleNode};

use crate::style::visitors::{
    insert,
    remove::{self, RemovedNode},
};

use super::EditorCommand;

pub struct RemoveNode {
    pub id: Uuid,
}
impl RemoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove::remove_node(&self.id, style)
            .map(|removed_node| RemoveNodeUndo { removed_node }.into())
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
        style.as_node_mut().search_mut(&parent_id, |parent_node| {
            insert::insert(parent_node, position, node.to_any())
        });
        Some(RemoveNode { id: node_id }.into())
    }
}
impl From<RemoveNodeUndo> for EditorCommand {
    fn from(value: RemoveNodeUndo) -> Self {
        Self::RemoveNodeUndo(value)
    }
}
