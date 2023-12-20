use std::{any::Any, ops::ControlFlow};

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{
    definitions::*,
    scene::SceneDefinition,
    visitor::{Node, NodeMut, NodeVisitor, NodeVisitorMut, Visitable},
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
impl Visitable for StyleDefinition {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.assets.walk(visitor)?;
        self.vars.walk(visitor)?;
        self.scene.walk(visitor)?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit(Node::Style(self))
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave(Node::Style(self))
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.assets.walk_mut(visitor)?;
        self.vars.walk_mut(visitor)?;
        self.scene.walk_mut(visitor)?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit(NodeMut::Style(self))
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave(NodeMut::Style(self))
    }
}

/// Base trait for all elements in the style definition.
pub trait StyleNode: ToAny + Visitable + Sync + Send + DynClone {
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
