use std::any::Any;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{
    definitions::*,
    iterator::{Node, NodeMut},
    scene::SceneDefinition,
};

pub mod assets;
pub mod cell;
pub mod component;
pub mod iterator;
pub mod scene;
pub mod timing_tower;
pub mod variables;

pub mod definitions {
    pub use self::super::{
        assets::{AssetDefinition, AssetFolder},
        cell::{Cell, ClipArea, CornerOffsets, FreeCell, Rounding},
        component::Component,
        scene::SceneDefinition,
        timing_tower::{TimingTower, TimingTowerRow},
        variables::{VariableDefinition, VariableFolder},
        StyleDefinition,
    };
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: AssetFolder,
    pub vars: VariableFolder,
    pub scene: SceneDefinition,
}
impl StyleNode for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Style(self)
    }
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Style(self)
    }
}

/// Base trait for all elements in the style definition.
pub trait StyleNode: ToAny + Sync + Send + DynClone {
    fn id(&self) -> &Uuid;
    fn as_node<'a>(&'a self) -> Node<'a>;
    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a>;
}

dyn_clone::clone_trait_object!(StyleNode);

/// Utilities for converting a `StyleNode` into any.
pub trait ToAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
}
impl<T> ToAny for T
where
    T: StyleNode + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
