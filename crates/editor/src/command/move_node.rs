use backend::{
    exact_variant::ExactVariant,
    style::{StyleDefinition, StyleId, StyleItem},
    tree_iterator::TreeIteratorMut,
};
use egui_ltreeview::DropPosition;

use super::{insert_node::insert, remove_node::remove_node, EditorCommand};

pub struct MoveNode {
    pub id: StyleId,
    pub target_id: StyleId,
    pub position: DropPosition<StyleId>,
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
