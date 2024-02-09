use std::ops::ControlFlow;

use egui_ltreeview::DropPosition;

use backend::{
    exact_variant::ExactVariant,
    style::{
        self, graphic::GraphicOrFolder, variables::VariableOrFolder, StyleDefinition, StyleId,
        StyleItem,
    },
    tree_iterator::{TreeItem, TreeIteratorMut},
};

use super::{remove_node::remove_node, EditorCommand};

pub struct InsertNode {
    pub target_node: StyleId,
    pub position: DropPosition<StyleId>,
    pub node: StyleItem,
}
impl InsertNode {
    pub fn execute(
        self,
        style: &mut ExactVariant<StyleItem, StyleDefinition>,
    ) -> Option<EditorCommand> {
        let id = self.node.id();
        style.search_mut(self.target_node, |node| {
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
    id: StyleId,
}
impl InsertNodeUndo {
    pub fn execute(
        self,
        style: &mut ExactVariant<StyleItem, StyleDefinition>,
    ) -> Option<EditorCommand> {
        remove_node(&self.id, style.as_enum_mut()).map(|removed_node| {
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
    node: &mut StyleItem,
    position: DropPosition<StyleId>,
    insert: StyleItem,
) -> ControlFlow<()> {
    match node {
        StyleItem::AssetFolder(folder) => {
            let folder_or_asset = match insert {
                StyleItem::Asset(asset) => style::assets::AssetOrFolder::Asset(asset.into()),
                StyleItem::AssetFolder(folder) => {
                    style::assets::AssetOrFolder::Folder(folder.into())
                }
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
        StyleItem::VariableFolder(folder) => {
            let folder_or_asset = match insert {
                StyleItem::Variable(asset) => VariableOrFolder::Variable(asset.into()),
                StyleItem::VariableFolder(folder) => VariableOrFolder::Folder(folder.into()),
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

        StyleItem::GraphicFolder(folder) => {
            let column_or_folder = match insert {
                StyleItem::GraphicFolder(folder) => GraphicOrFolder::Folder(folder.into()),
                StyleItem::Graphic(cell) => GraphicOrFolder::Graphic(cell.into()),
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

        StyleItem::Scene(_) => ControlFlow::Continue(()),
        StyleItem::Style(_) => ControlFlow::Continue(()),
        StyleItem::Variable(_) => ControlFlow::Continue(()),
        StyleItem::Asset(_) => ControlFlow::Continue(()),
        StyleItem::Graphic(_) => ControlFlow::Continue(()),
    }
}
