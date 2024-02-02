use std::ops::ControlFlow;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut};

use self::{
    assets::AssetOrFolder,
    cell::{FreeCellFolder, FreeCellOrFolder},
    definitions::*,
    scene::SceneDefinition,
    variables::VariableOrFolder,
};

pub mod assets;
pub mod cell;
pub mod component;
pub mod scene;
pub mod timing_tower;
pub mod variables;

pub mod definitions {
    pub use self::super::{
        assets::{AssetDefinition, AssetFolder},
        cell::{Cell, ClipArea, CornerOffsets, FreeCell, Rounding},
        component::Component,
        scene::SceneDefinition,
        timing_tower::{TimingTower, TimingTowerRow},
        variables::{VariableDefinition, VariableFolder},
        StyleDefinition,
    };
}

/// Base trait for all elements in the style definition.
pub trait StyleNode: Sync + Send + DynClone {
    fn id(&self) -> &Uuid;
    fn as_node<'a>(&'a self) -> Node<'a>;
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a>;
    fn to_node(self) -> OwnedNode;
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: AssetFolder,
    pub vars: VariableFolder,
    pub scene: SceneDefinition,
}
impl StyleNode for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Style(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Style(self)
    }
    fn to_node(self) -> OwnedNode {
        OwnedNode::Style(self)
    }
}

pub enum OwnedNode {
    Style(StyleDefinition),
    Variable(VariableDefinition),
    VariableFolder(VariableFolder),
    Asset(AssetDefinition),
    AssetFolder(AssetFolder),
    Scene(SceneDefinition),
    TimingTower(TimingTower),
    TimingTowerRow(TimingTowerRow),
    FreeCellFolder(FreeCellFolder),
    FreeCell(FreeCell),
    Component(Component),
}

impl TreeItem for OwnedNode {
    fn id(&self) -> Uuid {
        match self {
            OwnedNode::Style(o) => o.id,
            OwnedNode::Variable(o) => o.id,
            OwnedNode::VariableFolder(o) => o.id,
            OwnedNode::Asset(o) => o.id,
            OwnedNode::AssetFolder(o) => o.id,
            OwnedNode::Scene(o) => o.id,
            OwnedNode::TimingTower(o) => o.id,
            OwnedNode::TimingTowerRow(o) => o.id,
            OwnedNode::FreeCellFolder(o) => o.id,
            OwnedNode::FreeCell(o) => o.id,
            OwnedNode::Component(o) => o.id,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Node<'a> {
    Style(&'a StyleDefinition),
    Variable(&'a VariableDefinition),
    VariableFolder(&'a VariableFolder),
    Asset(&'a AssetDefinition),
    AssetFolder(&'a AssetFolder),
    Scene(&'a SceneDefinition),
    TimingTower(&'a TimingTower),
    TimingTowerRow(&'a TimingTowerRow),
    FreeCellFolder(&'a FreeCellFolder),
    FreeCell(&'a FreeCell),
    Component(&'a Component),
}

impl TreeItem for Node<'_> {
    fn id(&self) -> Uuid {
        match self {
            Node::Style(o) => o.id,
            Node::Variable(o) => o.id,
            Node::VariableFolder(o) => o.id,
            Node::Asset(o) => o.id,
            Node::AssetFolder(o) => o.id,
            Node::Scene(o) => o.id,
            Node::TimingTower(o) => o.id,
            Node::TimingTowerRow(o) => o.id,
            Node::FreeCellFolder(o) => o.id,
            Node::FreeCell(o) => o.id,
            Node::Component(o) => o.id,
        }
    }
}

impl TreeIterator for Node<'_> {
    type Item<'item> = Node<'item>;

    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            Node::Style(style) => {
                style.assets.as_node().walk(f)?;
                style.vars.as_node().walk(f)?;
                style.scene.as_node().walk(f)?;
            }
            Node::Variable(_) => (),
            Node::VariableFolder(var_folder) => {
                var_folder.content.iter().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.as_node().walk(f),
                    VariableOrFolder::Folder(o) => o.as_node().walk(f),
                })?;
            }
            Node::Asset(_) => (),
            Node::AssetFolder(asset_folder) => {
                asset_folder.content.iter().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.as_node().walk(f),
                    AssetOrFolder::Folder(o) => o.as_node().walk(f),
                })?;
            }
            Node::Scene(scene) => {
                scene.timing_tower.as_node().walk(f)?;
                scene
                    .components
                    .iter()
                    .try_for_each(|c| c.as_node().walk(f))?;
            }
            Node::TimingTower(tower) => {
                tower.row.as_node().walk(f)?;
                tower.cells.content.iter().try_for_each(|c| match c {
                    FreeCellOrFolder::Cell(o) => o.as_node().walk(f),
                    FreeCellOrFolder::Folder(o) => o.as_node().walk(f),
                })?;
            }
            Node::TimingTowerRow(tower_row) => {
                tower_row.columns.content.iter().try_for_each(|c| match c {
                    FreeCellOrFolder::Cell(o) => o.as_node().walk(f),
                    FreeCellOrFolder::Folder(o) => o.as_node().walk(f),
                })?;
            }
            Node::FreeCellFolder(cell_folder) => {
                cell_folder.content.iter().try_for_each(|v| match v {
                    FreeCellOrFolder::Cell(o) => o.as_node().walk(f),
                    FreeCellOrFolder::Folder(o) => o.as_node().walk(f),
                })?;
            }
            Node::FreeCell(_) => (),
            Node::Component(_) => (),
        }
        f(self, Method::Leave)
    }
}

pub enum NodeMut<'a> {
    Style(&'a mut StyleDefinition),
    Variable(&'a mut VariableDefinition),
    VariableFolder(&'a mut VariableFolder),
    Asset(&'a mut AssetDefinition),
    AssetFolder(&'a mut AssetFolder),
    Scene(&'a mut SceneDefinition),
    TimingTower(&'a mut TimingTower),
    TimingTowerRow(&'a mut TimingTowerRow),
    FreeCellFolder(&'a mut FreeCellFolder),
    FreeCell(&'a mut FreeCell),
    Component(&'a mut Component),
}

impl TreeItem for NodeMut<'_> {
    fn id(&self) -> Uuid {
        match self {
            NodeMut::Style(o) => o.id,
            NodeMut::Variable(o) => o.id,
            NodeMut::VariableFolder(o) => o.id,
            NodeMut::Asset(o) => o.id,
            NodeMut::AssetFolder(o) => o.id,
            NodeMut::Scene(o) => o.id,
            NodeMut::TimingTower(o) => o.id,
            NodeMut::TimingTowerRow(o) => o.id,
            NodeMut::FreeCellFolder(o) => o.id,
            NodeMut::FreeCell(o) => o.id,
            NodeMut::Component(o) => o.id,
        }
    }
}

impl TreeIteratorMut for NodeMut<'_> {
    type Item<'item> = NodeMut<'item>;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(&mut NodeMut<'_>, Method) -> ControlFlow<R>,
    {
        f(self, Method::Visit)?;
        match self {
            NodeMut::Style(style) => {
                style.assets.as_node_mut().walk_mut(f)?;
                style.vars.as_node_mut().walk_mut(f)?;
                style.scene.as_node_mut().walk_mut(f)?;
            }
            NodeMut::Variable(_) => (),
            NodeMut::VariableFolder(var_folder) => {
                var_folder.content.iter_mut().try_for_each(|v| match v {
                    VariableOrFolder::Variable(o) => o.as_node_mut().walk_mut(f),
                    VariableOrFolder::Folder(o) => o.as_node_mut().walk_mut(f),
                })?;
            }
            NodeMut::Asset(_) => (),
            NodeMut::AssetFolder(asset_folder) => {
                asset_folder.content.iter_mut().try_for_each(|v| match v {
                    AssetOrFolder::Asset(o) => o.as_node_mut().walk_mut(f),
                    AssetOrFolder::Folder(o) => o.as_node_mut().walk_mut(f),
                })?;
            }
            NodeMut::Scene(scene) => {
                scene.timing_tower.as_node_mut().walk_mut(f)?;
                scene
                    .components
                    .iter_mut()
                    .try_for_each(|c| c.as_node_mut().walk_mut(f))?;
            }
            NodeMut::TimingTower(tower) => {
                tower.row.as_node_mut().walk_mut(f)?;
                tower.cells.content.iter_mut().try_for_each(|c| match c {
                    FreeCellOrFolder::Cell(o) => o.as_node_mut().walk_mut(f),
                    FreeCellOrFolder::Folder(o) => o.as_node_mut().walk_mut(f),
                })?;
            }
            NodeMut::TimingTowerRow(tower_row) => {
                tower_row
                    .columns
                    .content
                    .iter_mut()
                    .try_for_each(|c| match c {
                        FreeCellOrFolder::Cell(o) => o.as_node_mut().walk_mut(f),
                        FreeCellOrFolder::Folder(o) => o.as_node_mut().walk_mut(f),
                    })?;
            }
            NodeMut::FreeCellFolder(cell_folder) => {
                cell_folder.content.iter_mut().try_for_each(|v| match v {
                    FreeCellOrFolder::Cell(o) => o.as_node_mut().walk_mut(f),
                    FreeCellOrFolder::Folder(o) => o.as_node_mut().walk_mut(f),
                })?;
            }
            NodeMut::FreeCell(_) => (),
            NodeMut::Component(_) => (),
        }
        f(self, Method::Leave)
    }
}
