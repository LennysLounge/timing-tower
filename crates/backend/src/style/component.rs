use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    cell::{ClipArea, FreeCell},
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    StyleNode,
};

/// A visual graphic component in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub elements: Vec<Element>,
}
impl Component {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Component"),
            elements: Vec::new(),
        }
    }
}

impl StyleNode for Component {
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    fn as_node<'a>(&'a self) -> super::iterator::Node<'a> {
        Node::Component(self)
    }

    fn as_node_mut<'a>(&'a mut self) -> super::iterator::NodeMut<'a> {
        NodeMut::Component(self)
    }
}
impl NodeIterator for Component {
    fn walk<F, R>(&self, f: &mut F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(super::iterator::Node, super::iterator::Method) -> std::ops::ControlFlow<R>,
    {
        f(self.as_node(), Method::Visit)?;
        f(self.as_node(), Method::Leave)
    }
}
impl NodeIteratorMut for Component {
    fn walk_mut<F, R>(&mut self, f: &mut F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(super::iterator::NodeMut, Method) -> std::ops::ControlFlow<R>,
    {
        f(self.as_node_mut(), Method::Visit)?;
        f(self.as_node_mut(), Method::Leave)
    }
}

/// A visual element that implements some functionality
/// or graphic.
#[derive(Serialize, Deserialize, Clone)]
pub enum Element {
    Cell(FreeCell),
    ClipArea(FreeClipArea),
    DriverTable,
}

/// An element that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FreeClipArea {
    pub id: Uuid,
    pub clip_area: ClipArea,
    pub elements: Vec<Element>,
}
