use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;

use backend::{
    exact_variant::ExactVariant,
    style::{StyleDefinition, StyleId, StyleItem},
    tree_iterator::{Method, TreeItem, TreeIteratorMut},
};

use super::{insert_node::insert, EditorCommand};

pub struct RemoveNode {
    pub id: StyleId,
}
impl RemoveNode {
    pub fn execute(
        self,
        style: &mut ExactVariant<StyleItem, StyleDefinition>,
    ) -> Option<EditorCommand> {
        remove_node(&self.id, style.as_enum_mut())
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
    pub fn execute(
        self,
        style: &mut ExactVariant<StyleItem, StyleDefinition>,
    ) -> Option<EditorCommand> {
        let RemovedNode {
            parent_id,
            node,
            position,
        } = self.removed_node;
        let node_id = node.id();
        style.search_mut(parent_id, |parent_node| insert(parent_node, position, node));
        Some(RemoveNode { id: node_id }.into())
    }
}
impl From<RemoveNodeUndo> for EditorCommand {
    fn from(value: RemoveNodeUndo) -> Self {
        Self::RemoveNodeUndo(value)
    }
}

pub struct RemovedNode {
    pub parent_id: StyleId,
    pub node: StyleItem,
    pub position: DropPosition<StyleId>,
}

pub fn remove_node(node_id: &StyleId, root: &mut StyleItem) -> Option<RemovedNode> {
    let output = root.walk_mut(&mut |node, method| remove(node, method, node_id));
    match output {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(x) => Some(x),
    }
}

fn remove(node: &mut StyleItem, method: Method, node_id: &StyleId) -> ControlFlow<RemovedNode> {
    match (method, node) {
        (Method::Visit, StyleItem::AssetFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::assets::AssetOrFolder::Asset(a) => a.to_enum(),
                        backend::style::assets::AssetOrFolder::Folder(f) => f.to_enum(),
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
        (Method::Visit, StyleItem::VariableFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::variables::VariableOrFolder::Variable(a) => a.to_enum(),
                        backend::style::variables::VariableOrFolder::Folder(f) => f.to_enum(),
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

        (Method::Visit, StyleItem::GraphicFolder(folder)) => {
            if let Some(index) = folder.content.iter().position(|s| s.id() == node_id) {
                ControlFlow::Break(RemovedNode {
                    parent_id: folder.id,
                    node: match folder.content.remove(index) {
                        backend::style::graphic::GraphicOrFolder::Graphic(t) => t.to_enum(),
                        backend::style::graphic::GraphicOrFolder::Folder(f) => f.to_enum(),
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

        (Method::Visit, StyleItem::Scene(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItem::Style(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItem::Variable(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItem::Asset(_)) => ControlFlow::Continue(()),
        (Method::Visit, StyleItem::Graphic(_)) => ControlFlow::Continue(()),
        (Method::Leave, _) => ControlFlow::Continue(()),
    }
}
