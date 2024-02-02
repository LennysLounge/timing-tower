use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::{
    style::{cell::FreeCellFolder, definitions::*, NodeMut, OwnedNode, StyleNode},
    tree_iterator::{Method, TreeItem, TreeIteratorMut},
};

use super::{insert_node::insert, EditorCommand};

pub struct RemoveNode {
    pub id: Uuid,
}
impl RemoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove_node(&self.id, &mut style.as_node_mut())
            .map(|removed_node| RemoveNodeUndo { removed_node }.into())
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
        let node_id = node.id();
        style
            .as_node_mut()
            .search_mut(parent_id, |parent_node| insert(parent_node, position, node));
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
    pub node: OwnedNode,
    pub position: DropPosition<Uuid>,
}

pub fn remove_node(node_id: &Uuid, root: &mut NodeMut) -> Option<RemovedNode> {
    let output = root.walk_mut(&mut |node, method| remove(node, method, node_id));
    match output {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(x) => Some(x),
    }
}

fn remove(node: &mut NodeMut, method: Method, node_id: &Uuid) -> ControlFlow<RemovedNode> {
    match (method, node) {
        (Method::Visit, NodeMut::AssetFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::assets::AssetOrFolder::Asset(a) => a.to_node(),
                        backend::style::assets::AssetOrFolder::Folder(f) => f.to_node(),
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
                        backend::style::variables::VariableOrFolder::Variable(a) => a.to_node(),
                        backend::style::variables::VariableOrFolder::Folder(f) => f.to_node(),
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

        (Method::Visit, NodeMut::TimingTower(tower)) => {
            remove_node_from_folder(&mut tower.cells, node_id)
        }
        (Method::Visit, NodeMut::TimingTowerRow(tower_row)) => {
            remove_node_from_folder(&mut tower_row.columns, node_id)
        }
        (Method::Visit, NodeMut::FreeCellFolder(folder)) => {
            remove_node_from_folder(folder, node_id)
        }

        (Method::Visit, NodeMut::Scene(scene)) => {
            if let Some(index) = scene.components.iter().position(|c| c.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: scene.id,
                    node: scene.components.remove(index).to_node(),
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

fn remove_node_from_folder(
    folder: &mut FreeCellFolder,
    node_id: &Uuid,
) -> ControlFlow<RemovedNode> {
    if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
        ControlFlow::Break(RemovedNode {
            parent_id: *folder.id(),
            node: match folder.content.remove(index) {
                backend::style::cell::FreeCellOrFolder::Cell(t) => t.to_node(),
                backend::style::cell::FreeCellOrFolder::Folder(f) => f.to_node(),
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
