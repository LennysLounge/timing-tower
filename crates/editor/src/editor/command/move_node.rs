use backend::style::StyleDefinition;
use egui_ltreeview::DropPosition;
use tracing::info;
use uuid::Uuid;

use crate::style::visitors::{insert::InsertNodeVisitor, remove::RemoveNodeVisitor};

use super::EditorCommand;

pub struct MoveNode {
    node_id: Uuid,
    parent_id: Uuid,
    position: DropPosition,
    taken_from: Option<Uuid>,
}

impl MoveNode {
    pub fn new(node_id: Uuid, parent_id: Uuid, position: DropPosition) -> Self {
        Self {
            node_id,
            parent_id,
            position,
            taken_from: None,
        }
    }
    pub fn undo(&self, style: &mut StyleDefinition) {
        if let Some(removed_node) = RemoveNodeVisitor::new(self.node_id).remove_from(style) {
            InsertNodeVisitor::new(
                self.taken_from.expect("The source parent id should be set"),
                DropPosition::Last,
                removed_node.node,
            )
            .insert_into(style);
        } else {
            info!("No node was removed from the tree");
        }
    }
    pub fn redo(&mut self, style: &mut StyleDefinition) {
        if let Some(removed_node) = RemoveNodeVisitor::new(self.node_id).remove_from(style) {
            InsertNodeVisitor::new(self.parent_id, self.position, removed_node.node)
                .insert_into(style);
            self.taken_from = Some(removed_node.parent_id);
        } else {
            info!("No node was removed from the tree");
        }
    }
}

impl From<MoveNode> for EditorCommand {
    fn from(value: MoveNode) -> Self {
        Self::MoveNode(value)
    }
}
