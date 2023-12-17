use std::{any::Any, ops::ControlFlow};

use backend::style::{
    definitions::*,
    timing_tower::TimingTowerColumnFolder,
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
    fn visit_asset_folder(&mut self, _folder: &AssetFolder) -> ControlFlow<()> {
        self.drop_allowed =
            self.dragged_node.is::<AssetDefinition>() || self.dragged_node.is::<AssetFolder>();
        ControlFlow::Break(())
    }

    fn visit_variable_folder(&mut self, _folder: &VariableFolder) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.is::<VariableDefinition>()
            || self.dragged_node.is::<VariableFolder>();
        ControlFlow::Break(())
    }

    fn visit_timing_tower_row(&mut self, _row: &TimingTowerRow) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.is::<TimingTowerColumn>()
            || self.dragged_node.is::<TimingTowerColumnFolder>();
        ControlFlow::Break(())
    }

    fn visit_timing_tower_column_folder(
        &mut self,
        _folder: &TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.is::<TimingTowerColumn>()
            || self.dragged_node.is::<TimingTowerColumnFolder>();
        ControlFlow::Break(())
    }
}
