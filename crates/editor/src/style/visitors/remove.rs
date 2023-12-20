use std::ops::ControlFlow;

use backend::style::{
    iterator::{Method, NodeIteratorMut, NodeMut},
    StyleNode,
};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

pub struct RemovedNode {
    pub parent_id: Uuid,
    pub node: Box<dyn StyleNode>,
    pub position: DropPosition,
}

pub fn remove_node<V: NodeIteratorMut>(node_id: &Uuid, visitable: &mut V) -> Option<RemovedNode> {
    let output =
        visitable.walk_mut(&mut |node: NodeMut, method: Method| remove(node, method, node_id));
    match output {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(x) => Some(x),
    }
}

fn remove(node: NodeMut, method: Method, node_id: &Uuid) -> ControlFlow<RemovedNode> {
    match (method, node) {
        (Method::Visit, NodeMut::AssetFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::assets::AssetOrFolder::Asset(a) => Box::new(a),
                        backend::style::assets::AssetOrFolder::Folder(f) => Box::new(f),
                    },
                    position: (index == 0)
                        .then_some(DropPosition::First)
                        .unwrap_or_else(|| {
                            DropPosition::After(*folder.content.get(index - 1).unwrap().id())
                        }),
                })
            } else {
                ControlFlow::Continue(())
            }
        }
        (Method::Visit, NodeMut::VariableFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::variables::VariableOrFolder::Variable(a) => Box::new(a),
                        backend::style::variables::VariableOrFolder::Folder(f) => Box::new(f),
                    },
                    position: (index == 0)
                        .then_some(DropPosition::First)
                        .unwrap_or_else(|| {
                            DropPosition::After(*folder.content.get(index - 1).unwrap().id())
                        }),
                })
            } else {
                ControlFlow::Continue(())
            }
        }

        (Method::Visit, NodeMut::TimingTowerRow(row)) => {
            if let Some(index) = row.columns.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: *row.id(),
                    node: match row.columns.remove(index) {
                        backend::style::timing_tower::ColumnOrFolder::Column(t) => Box::new(t),
                        backend::style::timing_tower::ColumnOrFolder::Folder(f) => Box::new(f),
                    },
                    position: (index == 0)
                        .then_some(DropPosition::First)
                        .unwrap_or_else(|| {
                            DropPosition::After(*row.columns.get(index - 1).unwrap().id())
                        }),
                })
            } else {
                ControlFlow::Continue(())
            }
        }

        (Method::Visit, NodeMut::TimingTowerColumnFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: *folder.id(),
                    node: match folder.content.remove(index) {
                        backend::style::timing_tower::ColumnOrFolder::Column(t) => Box::new(t),
                        backend::style::timing_tower::ColumnOrFolder::Folder(f) => Box::new(f),
                    },
                    position: (index == 0)
                        .then_some(DropPosition::First)
                        .unwrap_or_else(|| {
                            DropPosition::After(*folder.content.get(index - 1).unwrap().id())
                        }),
                })
            } else {
                ControlFlow::Continue(())
            }
        }
        _ => ControlFlow::Continue(()),
    }
}
