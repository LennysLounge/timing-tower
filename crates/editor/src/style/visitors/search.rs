use std::ops::ControlFlow;

use backend::style::{
    visitor::{NodeMut, NodeVisitorMut},
    StyleNode,
};
use uuid::Uuid;

pub struct SearchVisitorMut<'a, T> {
    id: Uuid,
    action: Option<Box<dyn FnOnce(&mut dyn StyleNode) -> T + 'a>>,
    output: Option<T>,
}
impl<'a, T> SearchVisitorMut<'a, T> {
    pub fn new(id: Uuid, action: impl FnOnce(&mut dyn StyleNode) -> T + 'a) -> Self {
        Self {
            id,
            action: Some(Box::new(action)),
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
            self.output = self.action.take().map(|action| (action)(node));
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }
}
impl<'a, T> NodeVisitorMut for SearchVisitorMut<'a, T> {
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()> {
        match node {
            NodeMut::Style(o) => self.test(o),
            NodeMut::Variable(o) => self.test(o),
            NodeMut::VariableFolder(o) => self.test(o),
            NodeMut::Asset(o) => self.test(o),
            NodeMut::AssetFolder(o) => self.test(o),
            NodeMut::Scene(o) => self.test(o),
            NodeMut::TimingTower(o) => self.test(o),
            NodeMut::TimingTowerRow(o) => self.test(o),
            NodeMut::TimingTowerColumn(o) => self.test(o),
            NodeMut::TimingTowerColumnFolder(o) => self.test(o),
            NodeMut::ClipArea(o) => self.test(o),
        }
    }
    fn leave(&mut self, _node: NodeMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}
