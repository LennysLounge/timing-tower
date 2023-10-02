#![allow(unused)]

use std::{any::Any, iter};

trait TreeNode: Any {
    fn get_id(&self) -> i32;
    fn as_dyn(&mut self) -> &mut dyn TreeNode;
    fn get_children(&mut self) -> Vec<&mut dyn TreeNode>;
    fn print(&mut self, depth: usize);
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn insert(&mut self, node: Box<dyn Any>);
    fn remove_node(&mut self, id: i32) -> Option<Box<dyn TreeNode>>;

    fn find(&mut self, id: i32) -> Option<&mut dyn TreeNode> {
        if self.get_id() == id {
            Some(self.as_dyn())
        } else {
            self.get_children().into_iter().find_map(|c| c.find(id))
        }
    }
}

enum Node {
    Dir(Dir),
    File(File),
}
impl TreeNode for Node {
    fn get_children(&mut self) -> Vec<&mut dyn TreeNode> {
        match self {
            Node::Dir(o) => o.get_children(),
            Node::File(o) => o.get_children(),
        }
    }

    fn print(&mut self, depth: usize) {
        match self {
            Node::Dir(o) => o.print(depth),
            Node::File(o) => o.print(depth),
        }
    }

    fn get_id(&self) -> i32 {
        match self {
            Node::Dir(o) => o.get_id(),
            Node::File(o) => o.get_id(),
        }
    }

    fn as_dyn(&mut self) -> &mut dyn TreeNode {
        match self {
            Node::Dir(o) => o.as_dyn(),
            Node::File(o) => o.as_dyn(),
        }
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn insert(&mut self, node: Box<dyn Any>) {
        match self {
            Node::Dir(o) => o.insert(node),
            Node::File(o) => o.insert(node),
        }
    }

    fn remove_node(&mut self, id: i32) -> Option<Box<dyn TreeNode>> {
        match self {
            Node::Dir(o) => o.remove_node(id),
            Node::File(o) => o.remove_node(id),
        }
    }
}

struct Dir {
    id: i32,
    meta_data: Option<Box<MetaData>>,
    children: Vec<Node>,
}

impl TreeNode for Dir {
    fn get_children(&mut self) -> Vec<&mut dyn TreeNode> {
        iter::once(self.meta_data.as_mut().map(|x| x.as_dyn()))
            .filter_map(|x| x)
            .chain(self.children.iter_mut().map(|c| c.as_dyn()))
            .collect()
    }

    fn print(&mut self, depth: usize) {
        println!("{:>depth$} Dir id({})", "", self.id);
        self.get_children()
            .into_iter()
            .for_each(|c| c.print(depth + 4));
    }
    fn get_id(&self) -> i32 {
        self.id
    }

    fn as_dyn(&mut self) -> &mut dyn TreeNode {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn insert(&mut self, node: Box<dyn Any>) {
        if let Ok(meta_data) = node.downcast::<MetaData>() {
            self.meta_data = Some(meta_data);
        }
    }

    fn remove_node(&mut self, id: i32) -> Option<Box<dyn TreeNode>> {
        if let Some(pos) = self.children.iter().position(|c| c.get_id() == id) {
            Some(Box::new(self.children.remove(pos)))
        } else {
            self.get_children()
                .into_iter()
                .find_map(|c| c.remove_node(id))
        }
    }
}

struct File {
    id: i32,
}
impl TreeNode for File {
    fn get_children(&mut self) -> Vec<&mut dyn TreeNode> {
        Vec::new()
    }

    fn print(&mut self, depth: usize) {
        println!("{:>depth$} File id({})", "", self.id);
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn as_dyn(&mut self) -> &mut dyn TreeNode {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn insert(&mut self, node: Box<dyn Any>) {}

    fn remove_node(&mut self, id: i32) -> Option<Box<dyn TreeNode>> {
        None
    }
}

struct MetaData {
    id: i32,
}
impl TreeNode for MetaData {
    fn get_children(&mut self) -> Vec<&mut dyn TreeNode> {
        Vec::new()
    }

    fn print(&mut self, depth: usize) {
        println!("{:>depth$} Meta Data id({})", "", self.id);
    }

    fn get_id(&self) -> i32 {
        self.id
    }

    fn as_dyn(&mut self) -> &mut dyn TreeNode {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn insert(&mut self, node: Box<dyn Any>) {}

    fn remove_node(&mut self, id: i32) -> Option<Box<dyn TreeNode>> {
        None
    }
}

fn insert(node: Box<dyn TreeNode>) {}

fn main() {
    let mut tree = Dir {
        id: 0,
        meta_data: None,
        children: vec![
            Node::Dir(Dir {
                id: 1,
                meta_data: Some(Box::new(MetaData { id: 7 })),
                children: vec![
                    Node::Dir(Dir {
                        id: 2,
                        meta_data: None,
                        children: vec![Node::File(File { id: 3 }), Node::File(File { id: 4 })],
                    }),
                    Node::File(File { id: 6 }),
                ],
            }),
            Node::File(File { id: 5 }),
        ],
    };
    tree.print(0);
    println!("------------");

    let mut dir = tree.find(2).unwrap();
    dir.print(0);
    println!("------------");

    let meta_data = Box::new(MetaData { id: 8 });
    dir.insert(meta_data.into_any());
    dir.print(0);

    println!("--------------");
    tree.print(0);
    
    println!("--------------");
    let x = tree.remove_node(4);
    tree.print(0);
}
