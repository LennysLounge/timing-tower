use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::Cell,
    clip_area::ClipArea,
    visitor::{Node, NodeVisitor, NodeVisitorMut, Visitable},
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
}
impl Visitable for TimingTower {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.row.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::TimingTower(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::TimingTower(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.row.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_timing_tower(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_timing_tower(self)
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
}
impl Visitable for TimingTowerRow {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.columns.iter().try_for_each(|c| match c {
            ColumnOrFolder::Column(o) => o.walk(visitor),
            ColumnOrFolder::Folder(o) => o.walk(visitor),
        })?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::TimingTowerRow(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::TimingTowerRow(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.columns.iter_mut().try_for_each(|c| match c {
            ColumnOrFolder::Column(o) => o.walk_mut(visitor),
            ColumnOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_timing_tower_row(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_timing_tower_row(self)
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
}
impl Visitable for TimingTowerColumn {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::TimingTowerColumn(self))
    }

    fn leave(&self, _visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_timing_tower_column(self)
    }

    fn leave_mut(&mut self, _visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
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
}
impl Visitable for TimingTowerColumnFolder {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.content.iter().try_for_each(|f| match f {
            ColumnOrFolder::Column(o) => o.walk(visitor),
            ColumnOrFolder::Folder(o) => o.walk(visitor),
        })?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::TimingTowerColumnFolder(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::TimingTowerColumnFolder(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.content.iter_mut().try_for_each(|f| match f {
            ColumnOrFolder::Column(o) => o.walk_mut(visitor),
            ColumnOrFolder::Folder(o) => o.walk_mut(visitor),
        })?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_timing_tower_column_folder(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_timing_tower_column_folder(self)
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
