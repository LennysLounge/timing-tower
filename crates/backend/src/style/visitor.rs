use std::ops::ControlFlow;

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
    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
}

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

pub trait NodeVisitor {
    fn visit(&mut self, node: Node) -> ControlFlow<()>;
    fn leave(&mut self, node: Node) -> ControlFlow<()>;
}

pub trait NodeVisitorMut {
    fn visit(&mut self, node: NodeMut) -> ControlFlow<()>;
    fn leave(&mut self, node: NodeMut) -> ControlFlow<()>;
}
