use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{NodeVisitor, NodeVisitorMut, StyleNode},
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
    fn test(&mut self, node: &dyn StyleNode) -> ControlFlow<()> {
        if &self.id == node.id() {
            self.output = Some((self.action)(node));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitor for SearchVisitor<'a, T> {
    fn visit_style(&mut self, style: &StyleDefinition) -> ControlFlow<()> {
        self.test(style)
    }

    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.test(folder.as_style_node())
    }

    fn visit_timing_tower(&mut self, tower: &TimingTower) -> ControlFlow<()> {
        self.test(tower)
    }

    fn visit_timing_tower_table(&mut self, table: &TimingTowerTable) -> ControlFlow<()> {
        self.test(table)
    }

    fn visit_timing_tower_row(&mut self, row: &TimingTowerRow) -> ControlFlow<()> {
        self.test(row)
    }

    fn visit_timing_tower_column(&mut self, column: &TimingTowerColumn) -> ControlFlow<()> {
        self.test(column)
    }

    fn visit_asset(&mut self, asset: &AssetDefinition) -> ControlFlow<()> {
        self.test(asset)
    }

    fn visit_variable(&mut self, variable: &VariableDefinition) -> ControlFlow<()> {
        self.test(variable)
    }
}

pub struct SearchVisitorMut<'a, T> {
    id: Uuid,
    action: Box<dyn FnMut(&mut dyn StyleNode) -> T + 'a>,
    output: Option<T>,
}
impl<'a, T> SearchVisitorMut<'a, T> {
    pub fn new(id: Uuid, action: impl FnMut(&mut dyn StyleNode) -> T + 'a) -> Self {
        Self {
            id,
            action: Box::new(action),
            output: None,
        }
    }
    pub fn search_in<V>(mut self, node: &mut V) -> Option<T>
    where
        V: StyleNode,
    {
        node.walk_mut(&mut self);
        self.output
    }
    fn test(&mut self, node: &mut dyn StyleNode) -> ControlFlow<()> {
        if &self.id == node.id() {
            self.output = Some((self.action)(node));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitorMut for SearchVisitorMut<'a, T> {
    fn visit_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
        self.test(style)
    }

    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        self.test(folder.as_style_node_mut())
    }

    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        self.test(tower)
    }

    fn visit_timing_tower_table(&mut self, table: &mut TimingTowerTable) -> ControlFlow<()> {
        self.test(table)
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        self.test(row)
    }

    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        self.test(column)
    }

    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        self.test(asset)
    }

    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        self.test(variable)
    }
}