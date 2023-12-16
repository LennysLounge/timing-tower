use backend::style::StyleDefinition;
use egui_ltreeview::DropPosition;
use uuid::Uuid;

use crate::style::visitors::{insert::InsertNodeVisitor, remove::RemoveNodeVisitor};

use super::EditorCommand;

pub struct MoveNode {
    pub id: Uuid,
    pub target_id: Uuid,
    pub position: DropPosition,
}
impl MoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        RemoveNodeVisitor::new(self.id)
            .remove_from(style)
            .map(|removed_node| {
                InsertNodeVisitor::new(self.target_id, self.position, removed_node.node)
                    .insert_into(style);
                MoveNode {
                    id: self.id,
                    target_id: removed_node.parent_id,
                    position: removed_node.position,
                }
                .into()
            })
    }
}
impl From<MoveNode> for EditorCommand {
    fn from(value: MoveNode) -> Self {
        Self::MoveNode(value)
    }
}
