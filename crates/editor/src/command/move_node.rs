use backend::{
    exact_variant::ExactVariant,
    style::{StyleDefinition, StyleItem},
    tree_iterator::TreeIteratorMut,
};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

use super::{insert_node::insert, remove_node::remove_node, EditorCommand};

pub struct MoveNode {
    pub id: Uuid,
    pub target_id: Uuid,
    pub position: DropPosition<Uuid>,
}
impl MoveNode {
    pub fn execute(
        self,
        style: &mut ExactVariant<StyleItem, StyleDefinition>,
    ) -> Option<EditorCommand> {
        remove_node(&self.id, style.as_enum_mut()).map(|removed_node| {
            style.search_mut(self.target_id, |node| {
                insert(node, self.position, removed_node.node);
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
