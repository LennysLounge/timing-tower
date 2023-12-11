use std::{any::Any, ops::ControlFlow};

use backend::style::{
    definitions::*,
    visitor::{NodeVisitor, Visitable},
};

pub struct DropAllowedVisitor<'a> {
    dragged_node: &'a dyn Any,
    drop_allowed: bool,
}
impl<'a> DropAllowedVisitor<'a> {
    pub fn new(dragged_node: &'a dyn Any) -> Self {
        Self {
            dragged_node,
            drop_allowed: false,
        }
    }
    pub fn test<V>(mut self, node: &V) -> bool
    where
        V: Visitable + ?Sized,
    {
        node.enter(&mut self);
        self.drop_allowed
    }
}
impl NodeVisitor for DropAllowedVisitor<'_> {
    fn visit_style(&mut self, _style: &StyleDefinition) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.type_id() == folder.content_type_id()
            || self.dragged_node.type_id() == folder.own_type_id();
        ControlFlow::Break(())
    }

    fn visit_timing_tower(&mut self, _tower: &TimingTower) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_timing_tower_table(&mut self, _table: &TimingTowerTable) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_timing_tower_row(&mut self, _row: &TimingTowerRow) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_timing_tower_column(&mut self, _column: &TimingTowerColumn) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_asset(&mut self, _asset: &AssetDefinition) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }

    fn visit_variable(&mut self, _variable: &VariableDefinition) -> ControlFlow<()> {
        self.drop_allowed = false;
        ControlFlow::Break(())
    }
}
