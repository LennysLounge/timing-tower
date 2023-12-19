use std::{any::Any, ops::ControlFlow};

use backend::style::{
    definitions::*,
    timing_tower::TimingTowerColumnFolder,
    visitor::{Node, NodeVisitor, Visitable},
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
    fn visit(&mut self, node: Node) -> ControlFlow<()> {
        self.drop_allowed = match node {
            Node::VariableFolder(_) => {
                self.dragged_node.is::<VariableDefinition>()
                    || self.dragged_node.is::<VariableFolder>()
            }
            Node::AssetFolder(_) => {
                self.dragged_node.is::<AssetDefinition>() || self.dragged_node.is::<AssetFolder>()
            }
            Node::TimingTowerRow(_) => {
                self.dragged_node.is::<TimingTowerColumn>()
                    || self.dragged_node.is::<TimingTowerColumnFolder>()
            }
            Node::TimingTowerColumnFolder(_) => {
                self.dragged_node.is::<TimingTowerColumn>()
                    || self.dragged_node.is::<TimingTowerColumnFolder>()
            }
            _ => false,
        };
        ControlFlow::Break(())
    }

    fn leave(&mut self, _node: Node) -> ControlFlow<()> {
        ControlFlow::Break(())
    }
}
