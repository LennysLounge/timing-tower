use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::Cell,
    folder::Folder,
    visitor::{NodeVisitor, NodeVisitorMut, Visitable},
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}
impl Visitable for TimingTower {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.table.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_timing_tower(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_timing_tower(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.table.walk_mut(visitor)?;
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
pub struct TimingTowerTable {
    pub id: Uuid,
    pub cell: Cell,
    pub row_offset: Vec2Property,
    pub row: TimingTowerRow,
}
impl Visitable for TimingTowerTable {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.row.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_timing_tower_table(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_timing_tower_table(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.row.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_timing_tower_table(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_timing_tower_table(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTowerRow {
    pub id: Uuid,
    pub cell: Cell,
    pub columns: Folder<TimingTowerColumn>,
}
impl Visitable for TimingTowerRow {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.columns.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_timing_tower_row(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_timing_tower_row(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.columns.walk_mut(visitor)?;
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
impl Visitable for TimingTowerColumn {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_timing_tower_column(self)
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
