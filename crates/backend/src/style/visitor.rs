use std::ops::ControlFlow;

use uuid::Uuid;

use super::{
    assets::{AssetDefinition, AssetFolder},
    clip_area::ClipArea,
    scene::SceneDefinition,
    timing_tower::{TimingTower, TimingTowerColumn, TimingTowerColumnFolder, TimingTowerRow},
    variables::{VariableDefinition, VariableFolder},
    StyleDefinition,
};

#[derive(PartialEq, Eq)]
pub enum Method {
    Visit,
    Leave,
}

pub trait NodeIterator {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;

    fn search<T>(&self, node_id: &Uuid, action: impl FnOnce(Node) -> T) -> Option<T> {
        Self::search_key(&self, |node| node.id() == node_id, action)
    }
    fn search_key<T>(
        &self,
        mut key: impl FnMut(&Node) -> bool,
        action: impl FnOnce(Node) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let mut output = None;
        self.walk(&mut |node: Node, method: Method| {
            if method == Method::Visit && key(&node) {
                output = action.take().map(|action| (action)(node));
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        output
    }
}

pub trait NodeIteratorMut {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;

    fn search_mut<T>(&mut self, node_id: &Uuid, action: impl FnOnce(NodeMut) -> T) -> Option<T> {
        Self::search_key_mut(self, |node| node.id() == node_id, action)
    }
    fn search_key_mut<T>(
        &mut self,
        mut key: impl FnMut(&NodeMut) -> bool,
        action: impl FnOnce(NodeMut) -> T,
    ) -> Option<T> {
        let mut action = Some(action);
        let mut output = None;
        self.walk_mut(&mut |node: NodeMut| {
            if key(&node) {
                output = action.take().map(|action| (action)(node));
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        output
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
    TimingTowerColumn(&'a TimingTowerColumn),
    TimingTowerColumnFolder(&'a TimingTowerColumnFolder),
    ClipArea(&'a ClipArea<TimingTowerRow>),
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
            Node::TimingTowerColumn(o) => &o.id,
            Node::TimingTowerColumnFolder(o) => &o.id,
            Node::ClipArea(o) => &o.data.id,
        }
    }
}
impl NodeIterator for Node<'_> {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        match self {
            Node::Style(o) => o.walk(visitor),
            Node::Variable(o) => o.walk(visitor),
            Node::VariableFolder(o) => o.walk(visitor),
            Node::Asset(o) => o.walk(visitor),
            Node::AssetFolder(o) => o.walk(visitor),
            Node::Scene(o) => o.walk(visitor),
            Node::TimingTower(o) => o.walk(visitor),
            Node::TimingTowerRow(o) => o.walk(visitor),
            Node::TimingTowerColumn(o) => o.walk(visitor),
            Node::TimingTowerColumnFolder(o) => o.walk(visitor),
            Node::ClipArea(o) => o.walk(visitor),
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
    TimingTowerColumn(&'a mut TimingTowerColumn),
    TimingTowerColumnFolder(&'a mut TimingTowerColumnFolder),
    ClipArea(&'a mut ClipArea<TimingTowerRow>),
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
            NodeMut::TimingTowerColumn(o) => &o.id,
            NodeMut::TimingTowerColumnFolder(o) => &o.id,
            NodeMut::ClipArea(o) => &o.data.id,
        }
    }
}
impl NodeIteratorMut for NodeMut<'_> {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        match self {
            NodeMut::Style(o) => o.walk_mut(visitor),
            NodeMut::Variable(o) => o.walk_mut(visitor),
            NodeMut::VariableFolder(o) => o.walk_mut(visitor),
            NodeMut::Asset(o) => o.walk_mut(visitor),
            NodeMut::AssetFolder(o) => o.walk_mut(visitor),
            NodeMut::Scene(o) => o.walk_mut(visitor),
            NodeMut::TimingTower(o) => o.walk_mut(visitor),
            NodeMut::TimingTowerRow(o) => o.walk_mut(visitor),
            NodeMut::TimingTowerColumn(o) => o.walk_mut(visitor),
            NodeMut::TimingTowerColumnFolder(o) => o.walk_mut(visitor),
            NodeMut::ClipArea(o) => o.walk_mut(visitor),
        }
    }
}

pub trait NodeVisitor {
    fn visit(&mut self, node: Node, method: Method) -> ControlFlow<()>;
}

pub trait NodeVisitorMut {
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()>;
    fn leave(&mut self, node: NodeMut) -> ControlFlow<()>;
}

impl<T> NodeVisitor for T
where
    T: FnMut(Node, Method) -> ControlFlow<()>,
{
    fn visit(&mut self, node: Node, method: Method) -> ControlFlow<()> {
        (self)(node, method)
    }
}

impl<T> NodeVisitorMut for T
where
    T: FnMut(NodeMut) -> ControlFlow<()>,
{
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()> {
        (self)(node)
    }

    fn leave(&mut self, _node: NodeMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}
