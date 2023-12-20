use std::{any::Any, ops::ControlFlow};

use backend::style::{
    self,
    definitions::*,
    timing_tower::TimingTowerColumnFolder,
    visitor::{NodeMut, NodeVisitorMut, Visitable},
    StyleNode,
};
use egui_ltreeview::DropPosition;
use uuid::Uuid;

pub struct InsertNodeVisitor {
    id: Uuid,
    position: DropPosition,
    // Option so we can leave an empty spot without moving any part of the parent struct.
    pub node: Option<Box<dyn StyleNode>>,
}
impl InsertNodeVisitor {
    pub fn new(id: Uuid, position: DropPosition, node: Box<dyn StyleNode>) -> Self {
        Self {
            id,
            position,
            node: Some(node),
        }
    }
    pub fn insert_into<V: Visitable>(mut self, visitable: &mut V) {
        visitable.walk_mut(&mut self);
    }
}
impl NodeVisitorMut for InsertNodeVisitor {
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()> {
        match node {
            NodeMut::AssetFolder(folder) => {
                if &self.id != folder.id() {
                    return ControlFlow::Continue(());
                }

                let folder_or_asset = {
                    let node = self.node.take().expect("Node should not be empty").to_any();
                    Err(node)
                        .or_else(|node| {
                            try_downcast_to::<AssetDefinition>(node)
                                .map(style::assets::AssetOrFolder::Asset)
                        })
                        .or_else(|node| {
                            try_downcast_to::<AssetFolder>(node)
                                .map(style::assets::AssetOrFolder::Folder)
                        })
                        .expect("No other types are allowed to be inserted")
                };

                match &self.position {
                    DropPosition::First => folder.content.insert(0, folder_or_asset),
                    DropPosition::Last => {
                        folder.content.insert(folder.content.len(), folder_or_asset)
                    }
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
                if &self.id != folder.id() {
                    return ControlFlow::Continue(());
                }

                let folder_or_asset = {
                    let node = self.node.take().expect("Node should not be empty").to_any();
                    Err(node)
                        .or_else(|node| {
                            try_downcast_to::<VariableDefinition>(node)
                                .map(style::variables::VariableOrFolder::Variable)
                        })
                        .or_else(|node| {
                            try_downcast_to::<VariableFolder>(node)
                                .map(style::variables::VariableOrFolder::Folder)
                        })
                        .expect("No other types are allowed to be inserted")
                };

                match &self.position {
                    DropPosition::First => folder.content.insert(0, folder_or_asset),
                    DropPosition::Last => {
                        folder.content.insert(folder.content.len(), folder_or_asset)
                    }
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
                if &self.id != row.id() {
                    return ControlFlow::Continue(());
                }
                let column_or_folder = {
                    let node = self.node.take().expect("Node should not be empty").to_any();
                    Err(node)
                        .or_else(|node| {
                            try_downcast_to::<TimingTowerColumnFolder>(node)
                                .map(style::timing_tower::ColumnOrFolder::Folder)
                        })
                        .or_else(|node| {
                            try_downcast_to::<TimingTowerColumn>(node)
                                .map(style::timing_tower::ColumnOrFolder::Column)
                        })
                        .expect("No other types are allowed to be inserted")
                };

                match &self.position {
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
                if &self.id != folder.id() {
                    return ControlFlow::Continue(());
                }
                let column_or_folder = {
                    let node = self.node.take().expect("Node should not be empty").to_any();
                    Err(node)
                        .or_else(|node| {
                            try_downcast_to::<TimingTowerColumnFolder>(node)
                                .map(style::timing_tower::ColumnOrFolder::Folder)
                        })
                        .or_else(|node| {
                            try_downcast_to::<TimingTowerColumn>(node)
                                .map(style::timing_tower::ColumnOrFolder::Column)
                        })
                        .expect("No other types are allowed to be inserted")
                };

                match &self.position {
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

    fn leave(&mut self, _node: NodeMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

fn try_downcast_to<T: 'static>(any: Box<dyn Any>) -> Result<T, Box<dyn Any>> {
    if any.is::<T>() {
        Ok(*any.downcast::<T>().expect("Cannot downcast but should"))
    } else {
        Err(any)
    }
}
