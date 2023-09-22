use crate::style_def::TimingTowerStyleDef;

use super::tree_view::{StyleElement, ElementTree, TreeNode};

fn parse_style_def(style: &TimingTowerStyleDef, tree: &mut ElementTree) {
    tree.add(TreeNode {
        id: tree.next_id(),
        node: Box::new(TimingTowerElement {}),
    });
}

pub struct TimingTowerElement;
impl StyleElement for TimingTowerElement {
    fn get_children(&self) -> Vec<super::tree_view::TreeNodeRef> {
        vec![]
    }

    fn get_children_mut(&mut self) -> Vec<super::tree_view::TreeNodeRefMut> {
        vec![]
    }

    fn print(&self, depth: usize, id: i32) {}

    fn mutate(&mut self, value: i32, id_source: &mut super::tree_view::IdSource) {}
}
