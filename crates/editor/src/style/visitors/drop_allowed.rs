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
    fn visit_folder(&mut self, folder: &dyn FolderInfo) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.type_id() == folder.content_type_id()
            || self.dragged_node.type_id() == folder.own_type_id();
        ControlFlow::Break(())
    }

    fn visit_asset_folder(&mut self, _folder: &AssetFolder) -> ControlFlow<()> {
        self.drop_allowed =
            self.dragged_node.is::<AssetDefinition>() || self.dragged_node.is::<AssetFolder>();
        ControlFlow::Break(())
    }

    fn visit_timing_tower_row(&mut self, _row: &TimingTowerRow) -> ControlFlow<()> {
        self.drop_allowed = self.dragged_node.is::<TimingTowerColumn>()
            || self.dragged_node.is::<Folder<TimingTowerColumn>>();
        ControlFlow::Break(())
    }
}
