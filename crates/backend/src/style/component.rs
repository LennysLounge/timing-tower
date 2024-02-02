use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    cell::{ClipArea, FreeCell},
    Node, NodeMut, OwnedNode, StyleNode,
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

    fn as_node<'a>(&'a self) -> Node<'a> {
        Node::Component(self)
    }

    fn as_node_mut<'a>(&'a mut self) -> NodeMut<'a> {
        NodeMut::Component(self)
    }
    fn to_node(self) -> OwnedNode {
        OwnedNode::Component(self)
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
impl Element {
    pub fn id(&self) -> &Uuid {
        match self {
            Element::Cell(cell) => &cell.id,
            Element::ClipArea(clip_area) => &clip_area.id,
            Element::DriverTable => todo!(),
        }
    }
}

/// An element that restaints the contained elements
/// to a sepcified area in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FreeClipArea {
    pub id: Uuid,
    pub clip_area: ClipArea,
    pub elements: Vec<Element>,
}
impl FreeClipArea {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            clip_area: ClipArea::default(),
            elements: Vec::new(),
        }
    }
}
