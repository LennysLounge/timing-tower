use std::{any::Any, ops::ControlFlow};

use egui_ltreeview::DropPosition;
use uuid::Uuid;

use backend::style::{
    self,
    definitions::*,
    iterator::{NodeIteratorMut, NodeMut},
    StyleNode,
};

use super::{remove_node::remove_node, EditorCommand};

pub struct InsertNode {
    pub target_node: Uuid,
    pub position: DropPosition,
    pub node: Box<dyn StyleNode>,
}
impl InsertNode {
    pub fn execute(self, style: &mut StyleDefinition) -> Option<EditorCommand> {
        let id = *self.node.id();
        style.as_node_mut().search_mut(&self.target_node, |node| {
            insert(node, self.position, self.node.clone().to_any())
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
        remove_node(&self.id, style).map(|removed_node| {
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

pub fn insert(node: NodeMut, position: DropPosition, insert: Box<dyn Any>) -> ControlFlow<()> {
    match node {
        NodeMut::AssetFolder(folder) => {
            let folder_or_asset = Err(insert)
                .or_else(|insert| {
                    insert
                        .downcast::<AssetDefinition>()
                        .map(|i| style::assets::AssetOrFolder::Asset(*i))
                })
                .or_else(|inset| {
                    inset
                        .downcast::<AssetFolder>()
                        .map(|i| style::assets::AssetOrFolder::Folder(*i))
                })
                .expect("No other types are allowed to be inserted");

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
            let folder_or_asset = Err(insert)
                .or_else(|insert| {
                    insert
                        .downcast::<VariableDefinition>()
                        .map(|i| style::variables::VariableOrFolder::Variable(*i))
                })
                .or_else(|insert| {
                    insert
                        .downcast::<VariableFolder>()
                        .map(|i| style::variables::VariableOrFolder::Folder(*i))
                })
                .expect("No other types are allowed to be inserted");

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
        NodeMut::TimingTowerRow(row) => {
            let column_or_folder = Err(insert)
                .or_else(|insert| {
                    insert
                        .downcast::<TimingTowerColumnFolder>()
                        .map(|i| style::timing_tower::ColumnOrFolder::Folder(*i))
                })
                .or_else(|node| {
                    node.downcast::<TimingTowerColumn>()
                        .map(|i| style::timing_tower::ColumnOrFolder::Column(*i))
                })
                .expect("No other types are allowed to be inserted");

            match &position {
                DropPosition::First => row.columns.insert(0, column_or_folder),
                DropPosition::Last => row.columns.push(column_or_folder),
                DropPosition::After(id) => {
                    if let Some(index) = row.columns.iter().position(|c| c.id() == id) {
                        row.columns.insert(index + 1, column_or_folder);
                    }
                }
                DropPosition::Before(id) => {
                    if let Some(index) = row.columns.iter().position(|c| c.id() == id) {
                        row.columns.insert(index, column_or_folder);
                    }
                }
            }
            ControlFlow::Break(())
        }
        NodeMut::TimingTowerColumnFolder(folder) => {
            let column_or_folder = Err(insert)
                .or_else(|insert| {
                    insert
                        .downcast::<TimingTowerColumnFolder>()
                        .map(|i| style::timing_tower::ColumnOrFolder::Folder(*i))
                })
                .or_else(|insert| {
                    insert
                        .downcast::<TimingTowerColumn>()
                        .map(|i| style::timing_tower::ColumnOrFolder::Column(*i))
                })
                .expect("No other types are allowed to be inserted");

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
        _ => ControlFlow::Continue(()),
    }
}
