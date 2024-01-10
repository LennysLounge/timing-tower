use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::{ClipArea, FreeCell, FreeCellFolder, FreeCellOrFolder},
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub position: Vec2Property,
    pub row: TimingTowerRow,
    #[serde(default)]
    pub cells: FreeCellFolder,
}
impl StyleNode for TimingTower {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::TimingTower(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::TimingTower(self)
    }
}
impl NodeIterator for TimingTower {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.row.walk(f)?;
        self.cells.content.iter().try_for_each(|c| match c {
            FreeCellOrFolder::Cell(o) => o.walk(f),
            FreeCellOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for TimingTower {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.row.walk_mut(f)?;
        self.cells.content.iter_mut().try_for_each(|c| match c {
            FreeCellOrFolder::Cell(o) => o.walk_mut(f),
            FreeCellOrFolder::Folder(o) => o.walk_mut(f),
        })?;
        f(self.as_node_mut(), Method::Leave)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub row_offset: Vec2Property,
    pub clip_area: ClipArea,
    pub columns: FreeCellFolder,
}
impl TimingTowerRow {
    pub fn contained_cells(&self) -> Vec<&FreeCell> {
        self.columns.contained_cells()
    }
}
impl StyleNode for TimingTowerRow {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::TimingTowerRow(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::TimingTowerRow(self)
    }
}
impl NodeIterator for TimingTowerRow {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        self.columns.content.iter().try_for_each(|c| match c {
            FreeCellOrFolder::Cell(o) => o.walk(f),
            FreeCellOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for TimingTowerRow {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        self.columns.content.iter_mut().try_for_each(|c| match c {
            FreeCellOrFolder::Cell(o) => o.walk_mut(f),
            FreeCellOrFolder::Folder(o) => o.walk_mut(f),
        })?;
        f(self.as_node_mut(), Method::Leave)
    }
}
