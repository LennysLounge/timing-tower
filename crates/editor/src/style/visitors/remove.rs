use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    folder::FolderOrT,
    visitor::{NodeVisitorMut, StyleNode, Visitable},
};
use uuid::Uuid;

pub struct RemoveNodeVisitor {
    id: Uuid,
    node: Option<Box<dyn StyleNode>>,
}
impl RemoveNodeVisitor {
    pub fn new(id: Uuid) -> Self {
        Self { id, node: None }
    }
    pub fn remove_from<V: Visitable>(mut self, visitable: &mut V) -> Option<Box<dyn StyleNode>> {
        visitable.walk_mut(&mut self);
        self.node
    }
}
impl NodeVisitorMut for RemoveNodeVisitor {
    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        if let Some(index) = folder
            .content()
            .into_iter()
            .position(|s| s.id() == &self.id)
        {
            self.node = folder.remove_index(index);
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        if let Some(index) = row.columns.iter().position(|s| s.id() == &self.id) {
            self.node = match row.columns.remove(index) {
                FolderOrT::T(t) => Some(Box::new(t)),
                FolderOrT::Folder(f) => Some(Box::new(f)),
            };
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
