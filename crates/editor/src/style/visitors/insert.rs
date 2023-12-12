use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    folder::FolderOrT,
    visitor::{NodeVisitorMut, StyleNode, Visitable},
};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

pub struct InsertNodeVisitor {
    id: Uuid,
    position: DropPosition,
    // Option so we can leave an empty spot without moving any part of the parent struct.
    pub node: Option<Box<dyn StyleNode>>,
}
impl InsertNodeVisitor {
    pub fn new(id: Uuid, position: DropPosition, node: Box<dyn StyleNode>) -> Self {
        Self {
            id,
            position,
            node: Some(node),
        }
    }
    pub fn insert_into<V: Visitable>(mut self, visitable: &mut V) {
        visitable.walk_mut(&mut self);
    }
}
impl NodeVisitorMut for InsertNodeVisitor {
    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        if &self.id != folder.id() {
            return ControlFlow::Continue(());
        }
        let node = self.node.take().expect("Node should not be empty");
        match &self.position {
            DropPosition::First => folder.insert_index(0, node),
            DropPosition::Last => folder.insert_index(folder.content().len(), node),
            DropPosition::After(id) => {
                if let Some(index) = folder.content().into_iter().position(|s| s.id() == id) {
                    folder.insert_index(index + 1, node);
                }
            }
            DropPosition::Before(id) => {
                if let Some(index) = folder.content().into_iter().position(|s| s.id() == id) {
                    folder.insert_index(index, node);
                }
            }
        }
        ControlFlow::Break(())
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        if &self.id != row.id() {
            return ControlFlow::Continue(());
        }
        let folder_or_t = {
            let node = self.node.take().expect("Node should not be empty").to_any();
            if node.is::<Folder<TimingTowerColumn>>() {
                let folder = node
                    .downcast::<Folder<TimingTowerColumn>>()
                    .expect("Cannot downcast but should");
                FolderOrT::Folder(*folder)
            } else if node.is::<TimingTowerColumn>() {
                let column = node
                    .downcast::<TimingTowerColumn>()
                    .expect("Cannot downcast but should");
                FolderOrT::T(*column)
            } else {
                unreachable!("No other types should be inserted into a row");
            }
        };
        match &self.position {
            DropPosition::First => row.columns.insert(0, folder_or_t),
            DropPosition::Last => row.columns.push(folder_or_t),
            DropPosition::After(id) => {
                if let Some(index) = row.columns.iter().position(|c| c.id() == id) {
                    row.columns.insert(index + 1, folder_or_t);
                }
            }
            DropPosition::Before(id) => {
                if let Some(index) = row.columns.iter().position(|c| c.id() == id) {
                    row.columns.insert(index, folder_or_t);
                }
            }
        }
        ControlFlow::Break(())
    }
}
