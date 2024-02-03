use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::{
    style::{
        self,
        cell::{FreeCellFolder, FreeCellOrFolder},
        graphic::GraphicOrFolder,
        variables::VariableOrFolder,
        OwnedStyleItem, StyleDefinition, StyleItem, StyleItemMut,
    },
    tree_iterator::{TreeItem, TreeIteratorMut},
};

use super::{remove_node::remove_node, EditorCommand};

pub struct InsertNode {
    pub target_node: Uuid,
    pub position: DropPosition<Uuid>,
    pub node: OwnedStyleItem,
}
impl InsertNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let id = self.node.id();
        style.as_mut().search_mut(self.target_node, |node| {
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
        remove_node(&self.id, &mut style.as_mut()).map(|removed_node| {
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
    node: &mut StyleItemMut,
    position: DropPosition<Uuid>,
    insert: OwnedStyleItem,
) -> ControlFlow<()> {
    match node {
        StyleItemMut::AssetFolder(folder) => {
            let folder_or_asset = match insert {
                OwnedStyleItem::Asset(asset) => style::assets::AssetOrFolder::Asset(asset),
                OwnedStyleItem::AssetFolder(folder) => style::assets::AssetOrFolder::Folder(folder),
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
        StyleItemMut::VariableFolder(folder) => {
            let folder_or_asset = match insert {
                OwnedStyleItem::Variable(asset) => VariableOrFolder::Variable(asset),
                OwnedStyleItem::VariableFolder(folder) => VariableOrFolder::Folder(folder),
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

        StyleItemMut::FreeCellFolder(folder) => {
            insert_into_free_cell_folder(folder, position, insert);
            ControlFlow::Break(())
        }
        StyleItemMut::GraphicFolder(folder) => {
            let column_or_folder = match insert {
                OwnedStyleItem::GraphicFolder(folder) => GraphicOrFolder::Folder(folder),
                OwnedStyleItem::Graphic(cell) => GraphicOrFolder::Graphic(cell),
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
            ControlFlow::Break(())
        }

        StyleItemMut::Scene(_) => ControlFlow::Continue(()),
        StyleItemMut::Style(_) => ControlFlow::Continue(()),
        StyleItemMut::Variable(_) => ControlFlow::Continue(()),
        StyleItemMut::Asset(_) => ControlFlow::Continue(()),
        StyleItemMut::FreeCell(_) => ControlFlow::Continue(()),
        StyleItemMut::Graphic(_) => ControlFlow::Continue(()),
    }
}

fn insert_into_free_cell_folder(
    folder: &mut FreeCellFolder,
    position: DropPosition<Uuid>,
    insert: OwnedStyleItem,
) {
    let column_or_folder = match insert {
        OwnedStyleItem::FreeCellFolder(folder) => FreeCellOrFolder::Folder(folder),
        OwnedStyleItem::FreeCell(cell) => FreeCellOrFolder::Cell(cell),
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
