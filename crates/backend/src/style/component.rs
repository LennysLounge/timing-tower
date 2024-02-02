use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{elements::Element, Node, NodeMut, OwnedNode, StyleNode};

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
