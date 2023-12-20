use std::ops::ControlFlow;

use backend::style::{
    visitor::{NodeMut, VisitableMut},
    StyleNode,
};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

pub struct RemovedNode {
    pub parent_id: Uuid,
    pub node: Box<dyn StyleNode>,
    pub position: DropPosition,
}

pub fn remove_node<V: VisitableMut>(node_id: &Uuid, visitable: &mut V) -> Option<RemovedNode> {
    let mut removed_node = None;
    visitable.walk_mut(&mut |node: NodeMut| match node {
        NodeMut::AssetFolder(folder) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                removed_node =
                    Some(RemovedNode {
                        parent_id: folder.id,
                        node: match folder.content.remove(index) {
                            backend::style::assets::AssetOrFolder::Asset(a) => Box::new(a),
                            backend::style::assets::AssetOrFolder::Folder(f) => Box::new(f),
                        },
                        position: (index == 0).then_some(DropPosition::First).unwrap_or_else(
                            || DropPosition::After(*folder.content.get(index - 1).unwrap().id()),
                        ),
                    });
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }
        NodeMut::VariableFolder(folder) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                removed_node =
                    Some(RemovedNode {
                        parent_id: folder.id,
                        node: match folder.content.remove(index) {
                            backend::style::variables::VariableOrFolder::Variable(a) => Box::new(a),
                            backend::style::variables::VariableOrFolder::Folder(f) => Box::new(f),
                        },
                        position: (index == 0).then_some(DropPosition::First).unwrap_or_else(
                            || DropPosition::After(*folder.content.get(index - 1).unwrap().id()),
                        ),
                    });
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }

        NodeMut::TimingTowerRow(row) => {
            if let Some(index) = row.columns.iter().position(|s| s.id() == node_id) {
                removed_node =
                    Some(RemovedNode {
                        parent_id: *row.id(),
                        node: match row.columns.remove(index) {
                            backend::style::timing_tower::ColumnOrFolder::Column(t) => Box::new(t),
                            backend::style::timing_tower::ColumnOrFolder::Folder(f) => Box::new(f),
                        },
                        position: (index == 0).then_some(DropPosition::First).unwrap_or_else(
                            || DropPosition::After(*row.columns.get(index - 1).unwrap().id()),
                        ),
                    });
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }

        NodeMut::TimingTowerColumnFolder(folder) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                removed_node =
                    Some(RemovedNode {
                        parent_id: *folder.id(),
                        node: match folder.content.remove(index) {
                            backend::style::timing_tower::ColumnOrFolder::Column(t) => Box::new(t),
                            backend::style::timing_tower::ColumnOrFolder::Folder(f) => Box::new(f),
                        },
                        position: (index == 0).then_some(DropPosition::First).unwrap_or_else(
                            || DropPosition::After(*folder.content.get(index - 1).unwrap().id()),
                        ),
                    });
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        }
        _ => ControlFlow::Continue(()),
    });
    removed_node
}
