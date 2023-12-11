use std::ops::ControlFlow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::Vec2Property;

use super::{
    cell::Cell,
    folder::FolderOrT,
    visitor::{NodeVisitor, NodeVisitorMut, StyleNode, Visitable},
};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TimingTower {
    pub id: Uuid,
    pub cell: Cell,
    pub table: TimingTowerTable,
}
impl StyleNode for TimingTower {
    fn id(&self) -> &Uuid {
        &self.id
    }
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
impl StyleNode for TimingTowerTable {
    fn id(&self) -> &Uuid {
        &self.id
    }
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
    pub columns: Vec<FolderOrT<TimingTowerColumn>>,
}
impl TimingTowerRow {
    pub fn all_columns(&self) -> Vec<&TimingTowerColumn> {
        self.columns
            .iter()
            .flat_map(|c| match c {
                FolderOrT::T(t) => vec![t],
                FolderOrT::Folder(f) => f.all_t(),
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
            FolderOrT::T(t) => t.walk(visitor),
            FolderOrT::Folder(f) => f.walk(visitor),
        })?;
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
        self.columns.iter_mut().try_for_each(|c| match c {
            FolderOrT::T(t) => t.walk_mut(visitor),
            FolderOrT::Folder(f) => f.walk_mut(visitor),
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
