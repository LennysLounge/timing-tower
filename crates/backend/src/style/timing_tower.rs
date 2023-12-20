use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::Cell,
    clip_area::ClipArea,
    visitor::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut, NodeVisitorMut},
    StyleNode,
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub row: ClipArea<TimingTowerRow>,
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
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)?;
        self.row.walk(f)?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for TimingTower {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.row.walk_mut(visitor)?;
        visitor.leave(self.as_node_mut())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub row_offset: Vec2Property,
    pub columns: Vec<ColumnOrFolder>,
}
impl TimingTowerRow {
    pub fn contained_columns(&self) -> Vec<&TimingTowerColumn> {
        self.columns
            .iter()
            .flat_map(|c| match c {
                ColumnOrFolder::Column(t) => vec![t],
                ColumnOrFolder::Folder(f) => f.contained_columns(),
            })
            .collect()
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
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)?;
        self.columns.iter().try_for_each(|c| match c {
            ColumnOrFolder::Column(o) => o.walk(f),
            ColumnOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for TimingTowerRow {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.columns.iter_mut().try_for_each(|c| match c {
            ColumnOrFolder::Column(o) => o.walk_mut(visitor),
            ColumnOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        visitor.leave(self.as_node_mut())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerColumn {
    pub id: Uuid,
    pub cell: Cell,
    pub name: String,
}

impl TimingTowerColumn {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            cell: Cell::default(),
            name: "new column".to_string(),
        }
    }
}
impl StyleNode for TimingTowerColumn {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::TimingTowerColumn(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::TimingTowerColumn(self)
    }
}
impl NodeIterator for TimingTowerColumn {
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)
    }
}
impl NodeIteratorMut for TimingTowerColumn {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerColumnFolder {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<ColumnOrFolder>,
}
impl TimingTowerColumnFolder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Group"),
            content: Vec::new(),
        }
    }
    pub fn contained_columns(&self) -> Vec<&TimingTowerColumn> {
        self.content
            .iter()
            .flat_map(|af| match af {
                ColumnOrFolder::Column(a) => vec![a],
                ColumnOrFolder::Folder(f) => f.contained_columns(),
            })
            .collect()
    }
}
impl StyleNode for TimingTowerColumnFolder {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::TimingTowerColumnFolder(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::TimingTowerColumnFolder(self)
    }
}
impl NodeIterator for TimingTowerColumnFolder {
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)?;
        self.content.iter().try_for_each(|v| match v {
            ColumnOrFolder::Column(o) => o.walk(f),
            ColumnOrFolder::Folder(o) => o.walk(f),
        })?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for TimingTowerColumnFolder {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.content.iter_mut().try_for_each(|f| match f {
            ColumnOrFolder::Column(o) => o.walk_mut(visitor),
            ColumnOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        visitor.leave(self.as_node_mut())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum ColumnOrFolder {
    Column(TimingTowerColumn),
    Folder(TimingTowerColumnFolder),
}
impl ColumnOrFolder {
    pub fn id(&self) -> &Uuid {
        match self {
            ColumnOrFolder::Column(o) => &o.id,
            ColumnOrFolder::Folder(o) => &o.id,
        }
    }
}
