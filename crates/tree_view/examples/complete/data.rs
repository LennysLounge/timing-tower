use std::ops::ControlFlow;

use uuid::{uuid, Uuid};

pub trait Visitable {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()>;
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()>;
}

pub trait NodeVisitor {
    #[allow(unused_variables)]
    fn visit_dir(&mut self, dir: &Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_dir(&mut self, dir: &Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_file(&mut self, file: &File) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

pub trait NodeVisitorMut {
    #[allow(unused_variables)]
    fn visit_dir(&mut self, dir: &mut Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn leave_dir(&mut self, dir: &mut Directory) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
    #[allow(unused_variables)]
    fn visit_file(&mut self, file: &mut File) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}

pub enum Node {
    Directory(Directory),
    File(File),
}
impl Visitable for Node {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.walk(visitor),
            Node::File(o) => o.walk(visitor),
        }
    }
    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.enter(visitor),
            Node::File(o) => o.enter(visitor),
        }
    }
    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.leave(visitor),
            Node::File(o) => o.leave(visitor),
        }
    }
    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.walk_mut(visitor),
            Node::File(o) => o.walk_mut(visitor),
        }
    }
    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.enter_mut(visitor),
            Node::File(o) => o.enter_mut(visitor),
        }
    }
    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        match self {
            Node::Directory(o) => o.leave_mut(visitor),
            Node::File(o) => o.leave_mut(visitor),
        }
    }
}
impl Node {
    pub fn id(&self) -> &Uuid {
        match self {
            Node::Directory(o) => &o.id,
            Node::File(o) => &o.id,
        }
    }
}

pub struct Directory {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<Node>,
    pub a_allowed: bool,
}
impl Visitable for Directory {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)?;
        self.nodes.iter().try_for_each(|n| n.walk(visitor))?;
        self.leave(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_dir(self)
    }

    fn leave(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.leave_dir(self)
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)?;
        self.nodes
            .iter_mut()
            .try_for_each(|n| n.walk_mut(visitor))?;
        self.leave_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_dir(self)
    }

    fn leave_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.leave_dir(self)
    }
}
impl Directory {
    pub fn new(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
            a_allowed: true,
        })
    }
    pub fn new_with_no_a(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
            a_allowed: false,
        })
    }
}

pub struct File {
    pub id: Uuid,
    pub name: String,
}
impl Visitable for File {
    fn walk(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        self.enter(visitor)
    }

    fn enter(&self, visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        visitor.visit_file(self)
    }

    fn leave(&self, _visitor: &mut dyn NodeVisitor) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }

    fn walk_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        self.enter_mut(visitor)
    }

    fn enter_mut(&mut self, visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        visitor.visit_file(self)
    }

    fn leave_mut(&mut self, _visitor: &mut dyn NodeVisitorMut) -> ControlFlow<()> {
        ControlFlow::Continue(())
    }
}
impl File {
    pub fn new(name: &str) -> Node {
        Node::File(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        })
    }
    pub fn new_with_id(name: &str, id: Uuid) -> Node {
        Node::File(Self {
            id,
            name: name.to_string(),
        })
    }
}

pub fn make_tree() -> Node {
    Directory::new(
        "Root",
        vec![
            Directory::new(
                "Things",
                vec![
                    Directory::new_with_no_a("Not A's", vec![File::new("GGGG")]),
                    File::new("CCCC"),
                    File::new("DDDD"),
                ],
            ),
            File::new("AAAA"),
            File::new_with_id("ABAB", uuid!("5ef68c19-45fd-4d34-84b5-89948df109f9")),
            File::new("BBBB"),
            Directory::new("Dodads", vec![File::new("EEEE"), File::new("FFFF")]),
        ],
    )
}
