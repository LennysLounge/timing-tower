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

pub trait Visitable {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
}

pub trait NodeIterator: Visitable {
    fn search(&self, node_id: &Uuid, action: impl FnOnce(Node)) {
        let mut action = Some(action);
        self.walk(&mut |node: Node| {
            if node.id() == node_id {
                action.take().map(|action| (action)(node));
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
    }
    fn search_mut(&mut self, node_id: &Uuid, action: impl FnOnce(NodeMut)) {
        let mut action = Some(action);
        self.walk_mut(&mut |node: NodeMut| {
            if node.id() == node_id {
                action.take().map(|action| (action)(node));
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
    }
}
impl<T: Visitable> NodeIterator for T {}

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

pub trait NodeVisitor {
    fn visit(&mut self, node: Node) -> ControlFlow<()>;
    fn leave(&mut self, node: Node) -> ControlFlow<()>;
}

pub trait NodeVisitorMut {
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()>;
    fn leave(&mut self, node: NodeMut) -> ControlFlow<()>;
}

impl<T> NodeVisitor for T
where
    T: FnMut(Node) -> ControlFlow<()>,
{
    fn visit(&mut self, node: Node) -> ControlFlow<()> {
        (self)(node)
    }

    fn leave(&mut self, _node: Node) -> ControlFlow<()> {
        ControlFlow::Continue(())
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
