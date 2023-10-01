trait TreeNode {
    fn children(&self) -> Vec<&dyn TreeNode>;
    fn children_mut(&self) -> Vec<&mut dyn TreeNode>;
    fn remove(&mut self, value: i32) -> Option<Self>
    where
        Self: Sized;
    fn value(&self) -> i32;
    fn as_trait(&self) -> &dyn TreeNode;
    fn as_trait_mut(&mut self) -> &mut dyn TreeNode;

    fn find(&self, value: i32) -> Option<&dyn TreeNode> {
        if self.value() == value {
            Some(self.as_trait())
        } else {
            self.children().into_iter().find_map(|c| c.find(value))
        }
    }
    fn find_mut(&self, value: i32) -> Option<&mut dyn TreeNode> {
        if self.value() == value {
            Some(self.as_trait_mut())
        } else {
            self.children().into_iter().find_map(|c| c.find_mut(value))
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Node>,
}

impl TreeNode for Node {
    fn children(&self) -> Vec<&dyn TreeNode> {
        self.children.iter().map(|c| c as &dyn TreeNode).collect()
    }

    fn children_mut(&self) -> Vec<&mut dyn TreeNode> {
        self.children
            .iter_mut()
            .map(|c| c as &mut dyn TreeNode)
            .collect()
    }

    fn remove(&mut self, value: i32) -> Option<Node> {
        if let Some(pos) = self.children.iter().position(|c| c.value == value) {
            Some(self.children.remove(pos))
        } else {
            self.children.iter_mut().find_map(|c| c.remove(value))
        }
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn as_trait(&self) -> &dyn TreeNode {
        self
    }
    fn as_trait_mut(&mut self) -> &mut dyn TreeNode {
        self
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

    let node = tree.find(12);
    println!("Node: ({:?})", node.map(|n| n.value()));

    let node = tree.find_mut(12).unwrap().remove(36);
    println!("Node: ({:#?})", node);
    println!("tree: ({:#?})", tree);
}
