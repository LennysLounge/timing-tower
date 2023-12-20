use backend::style::{visitor::NodeIterator, StyleDefinition};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

use crate::style::visitors::{insert, remove};

use super::EditorCommand;

pub struct MoveNode {
    pub id: Uuid,
    pub target_id: Uuid,
    pub position: DropPosition,
}
impl MoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove::remove_node(&self.id, style).map(|removed_node| {
            style.search_mut(&self.target_id, |node| {
                insert::insert(node, self.position, removed_node.node.to_any());
            });
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
