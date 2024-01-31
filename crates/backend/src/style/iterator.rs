use std::ops::ControlFlow;

use uuid::Uuid;

use super::{
    assets::{AssetDefinition, AssetFolder},
    cell::{FreeCell, FreeCellFolder},
    component::Component,
    scene::SceneDefinition,
    timing_tower::{TimingTower, TimingTowerRow},
    variables::{VariableDefinition, VariableFolder},
    StyleDefinition,
};

#[derive(PartialEq, Eq)]
pub enum Method {
    Visit,
    Leave,
}

pub trait NodeIterator {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>;

    fn search<T>(&self, node_id: &Uuid, action: impl FnOnce(Node) -> T) -> Option<T> {
        Self::search_key(&self, |node| node.id() == node_id, action)
    }
    fn search_key<T>(
        &self,
        mut key: impl FnMut(&Node) -> bool,
        action: impl FnOnce(Node) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk(&mut |node: Node, method: Method| {
            if method == Method::Visit && key(&node) {
                ControlFlow::Break(action.take().map(|action| (action)(node)))
            } else {
                ControlFlow::Continue(())
            }
        });
        match output {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(x) => x,
        }
    }
}

pub trait NodeIteratorMut {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>;

    fn search_mut<T>(&mut self, node_id: &Uuid, action: impl FnOnce(NodeMut) -> T) -> Option<T> {
        Self::search_key_mut(self, |node| node.id() == node_id, action)
    }
    fn search_key_mut<T>(
        &mut self,
        mut key: impl FnMut(&NodeMut) -> bool,
        action: impl FnOnce(NodeMut) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let output = self.walk_mut(&mut |node: NodeMut, method: Method| {
            if method == Method::Visit && key(&node) {
                ControlFlow::Break(action.take().map(|action| (action)(node)))
            } else {
                ControlFlow::Continue(())
            }
        });
        match output {
            ControlFlow::Continue(_) => None,
            ControlFlow::Break(x) => x,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Node<'a> {
    Style(&'a StyleDefinition),
    Variable(&'a VariableDefinition),
    VariableFolder(&'a VariableFolder),
    Asset(&'a AssetDefinition),
    AssetFolder(&'a AssetFolder),
    Scene(&'a SceneDefinition),
    TimingTower(&'a TimingTower),
    TimingTowerRow(&'a TimingTowerRow),
    FreeCellFolder(&'a FreeCellFolder),
    FreeCell(&'a FreeCell),
    Component(&'a Component),
}

impl Node<'_> {
    pub fn id(&self) -> &Uuid {
        match self {
            Node::Style(o) => &o.id,
            Node::Variable(o) => &o.id,
            Node::VariableFolder(o) => &o.id,
            Node::Asset(o) => &o.id,
            Node::AssetFolder(o) => &o.id,
            Node::Scene(o) => &o.id,
            Node::TimingTower(o) => &o.id,
            Node::TimingTowerRow(o) => &o.id,
            Node::FreeCellFolder(o) => &o.id,
            Node::FreeCell(o) => &o.id,
            Node::Component(o) => &o.id,
        }
    }
}
impl NodeIterator for Node<'_> {
    fn walk<F, R>(&self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(Node, Method) -> ControlFlow<R>,
    {
        match self {
            Node::Style(o) => o.walk(f),
            Node::Variable(o) => o.walk(f),
            Node::VariableFolder(o) => o.walk(f),
            Node::Asset(o) => o.walk(f),
            Node::AssetFolder(o) => o.walk(f),
            Node::Scene(o) => o.walk(f),
            Node::TimingTower(o) => o.walk(f),
            Node::TimingTowerRow(o) => o.walk(f),
            Node::FreeCellFolder(o) => o.walk(f),
            Node::FreeCell(o) => o.walk(f),
            Node::Component(o) => o.walk(f),
        }
    }
}

pub enum NodeMut<'a> {
    Style(&'a mut StyleDefinition),
    Variable(&'a mut VariableDefinition),
    VariableFolder(&'a mut VariableFolder),
    Asset(&'a mut AssetDefinition),
    AssetFolder(&'a mut AssetFolder),
    Scene(&'a mut SceneDefinition),
    TimingTower(&'a mut TimingTower),
    TimingTowerRow(&'a mut TimingTowerRow),
    FreeCellFolder(&'a mut FreeCellFolder),
    FreeCell(&'a mut FreeCell),
    Component(&'a mut Component),
}
impl NodeMut<'_> {
    pub fn id(&self) -> &Uuid {
        match self {
            NodeMut::Style(o) => &o.id,
            NodeMut::Variable(o) => &o.id,
            NodeMut::VariableFolder(o) => &o.id,
            NodeMut::Asset(o) => &o.id,
            NodeMut::AssetFolder(o) => &o.id,
            NodeMut::Scene(o) => &o.id,
            NodeMut::TimingTower(o) => &o.id,
            NodeMut::TimingTowerRow(o) => &o.id,
            NodeMut::FreeCellFolder(o) => &o.id,
            NodeMut::FreeCell(o) => &o.id,
            NodeMut::Component(o) => &o.id,
        }
    }
}
impl NodeIteratorMut for NodeMut<'_> {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> ControlFlow<R>
    where
        F: FnMut(NodeMut, Method) -> ControlFlow<R>,
    {
        match self {
            NodeMut::Style(o) => o.walk_mut(f),
            NodeMut::Variable(o) => o.walk_mut(f),
            NodeMut::VariableFolder(o) => o.walk_mut(f),
            NodeMut::Asset(o) => o.walk_mut(f),
            NodeMut::AssetFolder(o) => o.walk_mut(f),
            NodeMut::Scene(o) => o.walk_mut(f),
            NodeMut::TimingTower(o) => o.walk_mut(f),
            NodeMut::TimingTowerRow(o) => o.walk_mut(f),
            NodeMut::FreeCellFolder(o) => o.walk_mut(f),
            NodeMut::FreeCell(o) => o.walk_mut(f),
            NodeMut::Component(o) => o.walk_mut(f),
        }
    }
}
