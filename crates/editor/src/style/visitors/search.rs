use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{NodeVisitor, StyleNode},
};
use uuid::Uuid;

pub struct SearchVisitor<'a, T> {
    id: Uuid,
    action: Box<dyn FnMut(&dyn StyleNode) -> T + 'a>,
    output: Option<T>,
}
impl<'a, T> SearchVisitor<'a, T> {
    pub fn new(id: Uuid, action: impl FnMut(&dyn StyleNode) -> T + 'a) -> Self {
        Self {
            id,
            action: Box::new(action),
            output: None,
        }
    }
    pub fn search_in<V>(mut self, node: &V) -> Option<T>
    where
        V: StyleNode,
    {
        node.walk(&mut self);
        self.output
    }
    fn test(&mut self, id: &Uuid, visitable: &dyn StyleNode) -> ControlFlow<()> {
        if &self.id == id {
            self.output = Some((self.action)(visitable));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitor for SearchVisitor<'a, T> {
    fn visit_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        self.test(&style.id, style)
    }

    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.test(&folder.id(), folder.as_style_node())
    }

    fn visit_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        self.test(&tower.id, tower)
    }

    fn visit_timing_tower_table(&mut self, table: &TimingTowerTable) -> ControlFlow<()> {
        self.test(&table.id, table)
    }

    fn visit_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        self.test(&row.id, row)
    }

    fn visit_timing_tower_column(&mut self, column: &TimingTowerColumn) -> ControlFlow<()> {
        self.test(&column.id, column)
    }

    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        self.test(&asset.id, asset)
    }

    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        self.test(&variable.id, variable)
    }
}
