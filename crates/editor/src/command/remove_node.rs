use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::{
    style::{cell::FreeCellFolder, OwnedStyleItem, StyleDefinition, StyleItem, StyleItemMut},
    tree_iterator::{Method, TreeItem, TreeIteratorMut},
};

use super::{insert_node::insert, EditorCommand};

pub struct RemoveNode {
    pub id: Uuid,
}
impl RemoveNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove_node(&self.id, &mut style.as_mut())
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
            .as_mut()
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
    pub node: OwnedStyleItem,
    pub position: DropPosition<Uuid>,
}

pub fn remove_node(node_id: &Uuid, root: &mut StyleItemMut) -> Option<RemovedNode> {
    let output = root.walk_mut(&mut |node, method| remove(node, method, node_id));
    match output {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(x) => Some(x),
    }
}

fn remove(node: &mut StyleItemMut, method: Method, node_id: &Uuid) -> ControlFlow<RemovedNode> {
    match (method, node) {
        (Method::Visit, StyleItemMut::AssetFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::assets::AssetOrFolder::Asset(a) => a.to_owned(),
                        backend::style::assets::AssetOrFolder::Folder(f) => f.to_owned(),
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
        (Method::Visit, StyleItemMut::VariableFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::variables::VariableOrFolder::Variable(a) => a.to_owned(),
                        backend::style::variables::VariableOrFolder::Folder(f) => f.to_owned(),
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

        (Method::Visit, StyleItemMut::TimingTower(tower)) => {
            remove_node_from_folder(&mut tower.cells, node_id)
        }
        (Method::Visit, StyleItemMut::TimingTowerRow(tower_row)) => {
            remove_node_from_folder(&mut tower_row.columns, node_id)
        }
        (Method::Visit, StyleItemMut::FreeCellFolder(folder)) => {
            remove_node_from_folder(folder, node_id)
        }
        (Method::Visit, StyleItemMut::GraphicFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: *folder.id(),
                    node: match folder.content.remove(index) {
                        backend::style::graphic::GraphicOrFolder::Graphic(t) => t.to_owned(),
                        backend::style::graphic::GraphicOrFolder::Folder(f) => f.to_owned(),
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

        (Method::Visit, StyleItemMut::Scene(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItemMut::Style(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItemMut::Variable(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItemMut::Asset(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItemMut::FreeCell(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItemMut::Graphic(_)) => ControlFlow::Continue(()),
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
                backend::style::cell::FreeCellOrFolder::Cell(t) => t.to_owned(),
                backend::style::cell::FreeCellOrFolder::Folder(f) => f.to_owned(),
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
