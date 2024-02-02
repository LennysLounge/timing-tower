use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::{ClipArea, FreeCell, FreeCellFolder},
    Node, NodeMut, OwnedNode, StyleNode,
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
    fn to_node(self) -> OwnedNode {
        OwnedNode::TimingTower(self)
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
    fn to_node(self) -> OwnedNode {
        OwnedNode::TimingTowerRow(self)
    }
}
