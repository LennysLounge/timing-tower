use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::{
    style::{
        self,
        cell::{FreeCellFolder, FreeCellOrFolder},
        variables::VariableOrFolder,
        NodeMut, OwnedNode, StyleDefinition, StyleNode,
    },
    tree_iterator::{TreeItem, TreeIteratorMut},
};

use super::{remove_node::remove_node, EditorCommand};

pub struct InsertNode {
    pub target_node: Uuid,
    pub position: DropPosition<Uuid>,
    pub node: OwnedNode,
}
impl InsertNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let id = self.node.id();
        style.as_node_mut().search_mut(self.target_node, |node| {
            insert(node, self.position, self.node)
        });
        Some(InsertNodeUndo { id }.into())
    }
}

impl From<InsertNode> for EditorCommand {
    fn from(value: InsertNode) -> Self {
        Self::InsertNode(value)
    }
}

pub struct InsertNodeUndo {
    id: Uuid,
}
impl InsertNodeUndo {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        remove_node(&self.id, &mut style.as_node_mut()).map(|removed_node| {
            InsertNode {
                target_node: removed_node.parent_id,
                position: removed_node.position,
                node: removed_node.node,
            }
            .into()
        })
    }
}
impl From<InsertNodeUndo> for EditorCommand {
    fn from(value: InsertNodeUndo) -> Self {
        Self::InsertNodeUndo(value)
    }
}

pub fn insert(
    node: &mut NodeMut,
    position: DropPosition<Uuid>,
    insert: OwnedNode,
) -> ControlFlow<()> {
    match node {
        NodeMut::AssetFolder(folder) => {
            let folder_or_asset = match insert {
                OwnedNode::Asset(asset) => style::assets::AssetOrFolder::Asset(asset),
                OwnedNode::AssetFolder(folder) => style::assets::AssetOrFolder::Folder(folder),
                _ => unreachable!("No other types are allowed to be inserted"),
            };
            match &position {
                DropPosition::First => folder.content.insert(0, folder_or_asset),
                DropPosition::Last => folder.content.insert(folder.content.len(), folder_or_asset),
                DropPosition::After(id) => {
                    if let Some(index) = folder.content.iter().position(|s| s.id() == id) {
                        folder.content.insert(index + 1, folder_or_asset);
                    }
                }
                DropPosition::Before(id) => {
                    if let Some(index) = folder.content.iter().position(|s| s.id() == id) {
                        folder.content.insert(index, folder_or_asset);
                    }
                }
            }
            ControlFlow::Break(())
        }
        NodeMut::VariableFolder(folder) => {
            let folder_or_asset = match insert {
                OwnedNode::Variable(asset) => VariableOrFolder::Variable(asset),
                OwnedNode::VariableFolder(folder) => VariableOrFolder::Folder(folder),
                _ => unreachable!("No other types are allowed to be inserted"),
            };
            match &position {
                DropPosition::First => folder.content.insert(0, folder_or_asset),
                DropPosition::Last => folder.content.insert(folder.content.len(), folder_or_asset),
                DropPosition::After(id) => {
                    if let Some(index) = folder.content.iter().position(|s| s.id() == id) {
                        folder.content.insert(index + 1, folder_or_asset);
                    }
                }
                DropPosition::Before(id) => {
                    if let Some(index) = folder.content.iter().position(|s| s.id() == id) {
                        folder.content.insert(index, folder_or_asset);
                    }
                }
            }
            ControlFlow::Break(())
        }

        NodeMut::TimingTower(tower) => {
            insert_into_free_cell_folder(&mut tower.cells, position, insert);
            ControlFlow::Break(())
        }
        NodeMut::TimingTowerRow(tower_row) => {
            insert_into_free_cell_folder(&mut tower_row.columns, position, insert);
            ControlFlow::Break(())
        }
        NodeMut::FreeCellFolder(folder) => {
            insert_into_free_cell_folder(folder, position, insert);
            ControlFlow::Break(())
        }

        NodeMut::Scene(scene) => {
            let OwnedNode::Component(component) = insert else {
                unreachable!("No other types are allowed to be inserted");
            };
            match &position {
                DropPosition::First => scene.components.insert(0, component),
                DropPosition::Last => scene.components.push(component),
                DropPosition::After(id) => {
                    if let Some(index) = scene.components.iter().position(|c| c.id() == id) {
                        scene.components.insert(index + 1, component);
                    }
                }
                DropPosition::Before(id) => {
                    if let Some(index) = scene.components.iter().position(|c| c.id() == id) {
                        scene.components.insert(index, component);
                    }
                }
            }

            ControlFlow::Break(())
        }

        NodeMut::Style(_) => ControlFlow::Continue(()),
        NodeMut::Variable(_) => ControlFlow::Continue(()),
        NodeMut::Asset(_) => ControlFlow::Continue(()),
        NodeMut::FreeCell(_) => ControlFlow::Continue(()),
        NodeMut::Component(_) => ControlFlow::Continue(()),
    }
}

fn insert_into_free_cell_folder(
    folder: &mut FreeCellFolder,
    position: DropPosition<Uuid>,
    insert: OwnedNode,
) {
    let column_or_folder = match insert {
        OwnedNode::FreeCellFolder(folder) => FreeCellOrFolder::Folder(folder),
        OwnedNode::FreeCell(cell) => FreeCellOrFolder::Cell(cell),
        _ => unreachable!("No other types are allowed to be inserted"),
    };

    match &position {
        DropPosition::First => folder.content.insert(0, column_or_folder),
        DropPosition::Last => folder.content.push(column_or_folder),
        DropPosition::After(id) => {
            if let Some(index) = folder.content.iter().position(|c| c.id() == id) {
                folder.content.insert(index + 1, column_or_folder);
            }
        }
        DropPosition::Before(id) => {
            if let Some(index) = folder.content.iter().position(|c| c.id() == id) {
                folder.content.insert(index, column_or_folder);
            }
        }
    }
}
