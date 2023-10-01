trait TreeNode {
    type NodeType;
    fn find(&mut self, value: i32) -> Option<&mut dyn TreeNode<NodeType = Self::NodeType>>;
    fn take_first(&mut self) -> Option<Self::NodeType>;
    //fn take_first_of(&mut self, value: i32) -> Option<Self::NodeType>;
    fn take_first_of(&mut self, value: i32) -> Option<Self::NodeType> {
        if let Some(parent) = self.find(value) {
            parent.take_first()
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Node>,
}

impl TreeNode for Node {
    type NodeType = Node;

    fn take_first(&mut self) -> Option<Self::NodeType> {
        Some(self.children.remove(0))
    }

    fn find(&mut self, value: i32) -> Option<&mut dyn TreeNode<NodeType = Self::NodeType>> {
        if self.value == value {
            Some(self)
        } else {
            self.children.iter_mut().find_map(|c| c.find(value))
        }
    }
}

fn main() {
    let mut tree = Node {
        value: 12,
        children: vec![Node {
            value: 24,
            children: vec![Node {
                value: 36,
                children: vec![Node {
                    value: 48,
                    children: vec![],
                }],
            }],
        }],
    };

    let node = tree.take_first();
    println!("Node: {:#?}", node);

    //let tree_ref = &mut tree as &mut dyn TreeNode<NodeType = Node>;
    //let tree_ref = tree.find(12).unwrap();
    println!("Node: {:#?}", tree.take_first_of(12));
}
