use std::{any::Any, ops::ControlFlow};

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{
    definitions::*,
    scene::SceneDefinition,
    visitor::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut, NodeVisitorMut},
};

pub mod assets;
pub mod cell;
pub mod clip_area;
pub mod scene;
pub mod timing_tower;
pub mod variables;
pub mod visitor;

pub mod definitions {
    pub use self::super::{
        assets::{AssetDefinition, AssetFolder},
        clip_area::{ClipArea, ClipAreaData, DynClipArea},
        scene::SceneDefinition,
        timing_tower::{TimingTower, TimingTowerColumn, TimingTowerColumnFolder, TimingTowerRow},
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
impl NodeIterator for StyleDefinition {
    fn walk<F>(&self, f: &mut F) -> ControlFlow<()>
    where
        F: FnMut(Node, Method) -> ControlFlow<()>,
    {
        f(self.as_node(), Method::Visit)?;
        self.assets.walk(f)?;
        self.vars.walk(f)?;
        self.scene.walk(f)?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for StyleDefinition {
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(self.as_node_mut())?;
        self.assets.walk_mut(visitor)?;
        self.vars.walk_mut(visitor)?;
        self.scene.walk_mut(visitor)?;
        visitor.leave(self.as_node_mut())
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

// /// Allows for cloneing a `StyleNode` as a trait object.
// pub trait BoxClone {
//     /// Clone the element into a `StyleNode` trait object.
//     fn box_clone(&self) -> Box<dyn StyleNode>;
// }
// impl<T> BoxClone for T
// where
//     T: StyleNode + Clone + 'static,
// {
//     fn box_clone(&self) -> Box<dyn StyleNode> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn StyleNode> {
//     fn clone(&self) -> Self {
//         (*self).box_clone()
//     }
// }
