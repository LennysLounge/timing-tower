use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::style::{
    definitions::*,
    iterator::{Method, NodeIteratorMut, NodeMut},
    StyleNode,
};

use super::{insert_node::insert, EditorCommand};

pub struct RemoveNode {
    pub id: Uuid,
}
impl RemoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove_node(&self.id, style).map(|removed_node| RemoveNodeUndo { removed_node }.into())
    }
}

impl From<RemoveNode> for EditorCommand {
    fn from(value: RemoveNode) -> Self {
        Self::RemoveNode(value)
    }
}

pub struct RemoveNodeUndo {
    removed_node: RemovedNode,
}
impl RemoveNodeUndo {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let RemovedNode {
            parent_id,
            node,
            position,
        } = self.removed_node;
        let node_id = *node.id();
        style.as_node_mut().search_mut(&parent_id, |parent_node| {
            insert(parent_node, position, node.to_any())
        });
        Some(RemoveNode { id: node_id }.into())
    }
}
impl From<RemoveNodeUndo> for EditorCommand {
    fn from(value: RemoveNodeUndo) -> Self {
        Self::RemoveNodeUndo(value)
    }
}

pub struct RemovedNode {
    pub parent_id: Uuid,
    pub node: Box<dyn StyleNode>,
    pub position: DropPosition<Uuid>,
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

        (Method::Visit, NodeMut::TimingTower(TimingTower { cells: folder, .. }))
        | (
            Method::Visit,
            NodeMut::TimingTowerRow(TimingTowerRow {
                columns: folder, ..
            }),
        )
        | (Method::Visit, NodeMut::FreeCellFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: *folder.id(),
                    node: match folder.content.remove(index) {
                        backend::style::cell::FreeCellOrFolder::Cell(t) => Box::new(t),
                        backend::style::cell::FreeCellOrFolder::Folder(f) => Box::new(f),
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

        (Method::Visit, NodeMut::Scene(scene)) => {
            if let Some(index) = scene.components.iter().position(|c| c.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: scene.id,
                    node: Box::new(scene.components.remove(index)),
                    position: (index == 0)
                        .then_some(DropPosition::First)
                        .unwrap_or_else(|| {
                            DropPosition::After(*scene.components.get(index - 1).unwrap().id())
                        }),
                })
            } else {
                ControlFlow::Continue(())
            }
        }

        (Method::Visit, NodeMut::Style(_)) => ControlFlow::Continue(()),
        (Method::Visit, NodeMut::Variable(_)) => ControlFlow::Continue(()),
        (Method::Visit, NodeMut::Asset(_)) => ControlFlow::Continue(()),
        (Method::Visit, NodeMut::FreeCell(_)) => ControlFlow::Continue(()),
        (Method::Visit, NodeMut::Component(_)) => ControlFlow::Continue(()),
        (Method::Leave, _) => ControlFlow::Continue(()),
    }
}
