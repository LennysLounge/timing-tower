use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::style::{definitions::*, StyleNode};

use crate::style::visitors::{insert::InsertNodeVisitor, remove::RemoveNodeVisitor};

use super::EditorCommand;

pub struct InsertNode {
    pub target_node: Uuid,
    pub position: DropPosition,
    pub node: Box<dyn StyleNode>,
}
impl InsertNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let id = *self.node.id();
        InsertNodeVisitor::new(self.target_node, self.position, (*self.node).box_clone())
            .insert_into(style);
        Some(InsertNodeUndo { id }.into())
    }
}

impl From<InsertNode> for EditorCommand {
    fn from(value: InsertNode) -> Self {
        Self::InsertNode(value)
    }
}

pub struct InsertNodeUndo {
    id: Uuid,
}
impl InsertNodeUndo {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        RemoveNodeVisitor::new(self.id)
            .remove_from(style)
            .map(|removed_node| {
                InsertNode {
                    target_node: removed_node.parent_id,
                    position: removed_node.position,
                    node: removed_node.node,
                }
                .into()
            })
    }
}
impl From<InsertNodeUndo> for EditorCommand {
    fn from(value: InsertNodeUndo) -> Self {
        Self::InsertNodeUndo(value)
    }
}
